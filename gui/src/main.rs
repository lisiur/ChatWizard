#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chat_wizard_api::app;
use chat_wizard_service::{
    services::prompt_market::PromptMarketService, ChatService, PromptService, SettingService,
};
use project::Project;
use tauri::Manager;
use window::{create_window, WindowOptions};

mod commands;
mod error;
mod project;
mod result;
mod tray;
mod utils;
mod window;

pub struct Port(u16);

#[tokio::main]
async fn main() {
    env_logger::init();
    let project = Project::init().await.unwrap();
    let conn = chat_wizard_service::init(&project.db_url).unwrap();

    // let port = portpicker::pick_unused_port().unwrap();
    let port = 23333;

    tokio::spawn(app(port, conn.clone()));

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(main_window) = app.get_window("main") {
                main_window.show().unwrap();
                main_window.unminimize().unwrap();
                main_window.set_focus().unwrap();
            }
        }))
        .system_tray(tray::system_tray())
        .on_system_tray_event(tray::on_system_tray_event)
        .manage(Port(port))
        .manage(SettingService::new(conn.clone()))
        .manage(ChatService::new(conn.clone()))
        .manage(PromptService::new(conn.clone()))
        .manage(PromptMarketService::new(conn.clone()))
        .manage(conn)
        .setup(|app| {
            create_window(
                "main",
                WindowOptions {
                    title: "ChatWizard".to_string(),
                    url: "".to_string(),
                    width: 860.0,
                    height: 720.0,
                    ..Default::default()
                },
                app.handle(),
            )
            .unwrap();
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
                        // or we cannot show the window again from the dock icon, it will be very confusing
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
