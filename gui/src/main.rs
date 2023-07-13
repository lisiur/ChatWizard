#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::exit;
use std::sync::Arc;

use chat_wizard_service::project::Project;
use chat_wizard_service::services::plugin::PluginService;
use tauri::api::cli::SubcommandMatches;
use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::Mutex;

use chat_wizard_api::app as api_app;
use chat_wizard_service::{
    commands::{CommandEvent, CommandExecutor},
    Id, Setting, SettingService,
};
use tauri::{AppHandle, Manager};
use window::{create_tray_window_in_background, show_or_create_main_window};

use crate::utils::is_free_tcp;

mod commands;
mod error;
mod result;
mod schema_server;
mod tray;
mod utils;
mod window;

static WEB_SERVER_PORT: u16 = 23333;
static GUI_SERVER_PORT: u16 = 23334;

pub struct SchemaPort(u16);

pub struct EventBus {
    pub sender: Sender<CommandEvent>,
}
pub struct AppSetting(Arc<Mutex<Setting>>);

impl EventBus {
    fn new(app_handle: AppHandle) -> Self {
        let (sender, mut receiver) = channel::<CommandEvent>(20);

        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                app_handle.emit_all(&event.name, event.payload).unwrap();
            }
        });

        Self { sender }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let project = Project::init().unwrap();
    let conn = chat_wizard_service::init(&project.db_url).unwrap();

    let setting = SettingService::new(conn.clone())
        .get_setting(Id::local())
        .unwrap();

    let enable_web_server = setting.enable_web_server;
    let hide_main_window = setting.hide_main_window;

    let app_setting = AppSetting(Arc::new(Mutex::new(setting)));

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_single_instance::init(|handle, _argv, _cwd| {
            let handle = handle.clone();
            tokio::spawn(async move {
                show_or_create_main_window(&handle, "index.html")
                    .await
                    .unwrap();
            });
        }))
        .system_tray(tray::system_tray())
        .on_system_tray_event(tray::on_system_tray_event)
        .manage(conn.clone())
        .manage(CommandExecutor::new())
        .manage(app_setting)
        .setup(move |app| {
            let app_handle = app.handle();
            app.manage(EventBus::new(app_handle.clone()));

            // cli handler
            let instance_exist = !is_free_tcp(GUI_SERVER_PORT);
            if !instance_exist {
                // start gui server
                // let port = portpicker::pick_unused_port().unwrap();
                let gui_server_port = GUI_SERVER_PORT;
                let schema_port = SchemaPort(gui_server_port);
                app_handle.manage(schema_port);

                let handle = app_handle.clone();
                tokio::spawn(async move {
                    schema_server::serve(gui_server_port, handle).await;
                });
            }

            match app.get_cli_matches() {
                Ok(matches) => {
                    if matches.args.contains_key("help") {
                        let output = matches.args.get("help").unwrap().value.as_str().unwrap();
                        println!("{}", output);
                        exit(0);
                    } else if let Some(SubcommandMatches { name, matches, .. }) =
                        matches.subcommand.as_deref()
                    {
                        if name == "exec" {
                            let command = matches
                                .args
                                .get("command")
                                .unwrap()
                                .value
                                .as_str()
                                .unwrap()
                                .to_string();

                            let conn = conn.clone();
                            tokio::spawn(async move {
                                let plugin_service = PluginService::new(conn);
                                plugin_service.execute_by_name(&command).await.unwrap();
                                exit(0);
                            });
                        } else {
                            exit(0);
                        }
                    }
                }
                Err(err) => {
                    println!("{}", err);
                    exit(1);
                }
            };
            if instance_exist {
                app_handle.tray_handle().destroy().unwrap();
                return Ok(());
            }

            // start web server
            let web_server_port = WEB_SERVER_PORT;
            if enable_web_server {
                tokio::spawn(api_app(web_server_port, conn));
            }

            // show main window
            if !hide_main_window {
                let handle = app_handle.clone();
                tokio::spawn(async move {
                    show_or_create_main_window(&handle, "index.html")
                        .await
                        .unwrap();
                });
            }

            // create tray window
            create_tray_window_in_background(&app_handle).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::exec_command,
            commands::show_or_create_window,
            commands::show_window,
            commands::create_window,
            commands::open,
            commands::save_file,
            commands::debug_log,
        ])
        .on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                let win = event.window();
                if win.label() == "main" {
                    win.hide().unwrap();
                    api.prevent_close();
                } else {
                    win.close().unwrap();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
