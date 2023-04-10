#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::Mutex;

use chat_wizard_api::app as api_app;
use chat_wizard_service::{
    commands::{CommandEvent, CommandExecutor},
    Id, Setting, SettingService,
};
use project::Project;
use tauri::{AppHandle, Manager};
use window::{create_tray_window_in_background, show_or_create_main_window};

mod commands;
mod error;
mod project;
mod result;
mod tray;
mod utils;
mod window;

pub struct Port(u16);
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
    let project = Project::init().await.unwrap();
    let conn = chat_wizard_service::init(&project.db_url).unwrap();

    let setting = SettingService::new(conn.clone())
        .get_setting(Id::local())
        .unwrap();

    let enable_web_server = setting.enable_web_server;
    let hide_main_window = setting.hide_main_window;

    let app_setting = AppSetting(Arc::new(Mutex::new(setting)));

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(main_window) = app.get_window("main") {
                main_window.show().unwrap();
                main_window.unminimize().unwrap();
                main_window.set_focus().unwrap();
            }
        }))
        .system_tray(tray::system_tray())
        .on_system_tray_event(tray::on_system_tray_event)
        .manage(conn.clone())
        .manage(CommandExecutor::new())
        .manage(app_setting)
        .setup(move |app| {
            let app_handle = app.handle();
            app.manage(EventBus::new(app_handle.clone()));

            // start web server
            let port = 23333;
            app.manage(Port(port));
            if enable_web_server {
                // let port = portpicker::pick_unused_port().unwrap();
                tokio::spawn(api_app(port, conn));
            }

            // show main window
            if !hide_main_window {
                let handle = app_handle.clone();
                tokio::spawn(async move {
                    show_or_create_main_window(&handle).await.unwrap();
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
        ])
        .on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                let win = event.window();
                if win.label() == "main" {
                    #[cfg(target_os = "macos")]
                    {
                        // Since currently skip_taskbar is not supported on macOS,
                        // and tauri doesn't support handle the click event of the dock icon
                        // we need to minimize the window instead of hide it
                        // otherwise we cannot show the window again from the dock icon, it will be very confusing
                        win.minimize().unwrap();
                    }
                    #[cfg(not(target_os = "macos"))]
                    {
                        win.hide().unwrap();
                    }
                    api.prevent_close();
                } else {
                    win.close().unwrap();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
