#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chat_wizard_service::{
    services::prompt_market::PromptMarketService, ChatService, PromptService, SettingService,
};
use project::Project;
use window::{create_window, WindowOptions};

mod commands;
mod error;
mod project;
mod result;
mod tray;
mod utils;
mod window;

#[tokio::main]
async fn main() {
    env_logger::init();
    let project = Project::init().await.unwrap();
    let conn = chat_wizard_service::init(&project.db_url).unwrap();

    tauri::Builder::default()
        .system_tray(tray::system_tray())
        .on_system_tray_event(tray::on_system_tray_event)
        .manage(SettingService::new(conn.clone()))
        .manage(ChatService::new(conn.clone()))
        .manage(PromptService::new(conn.clone()))
        .manage(PromptMarketService::new(conn))
        .setup(|app| {
            create_window(
                "main",
                WindowOptions {
                    title: "chat-wizard".to_string(),
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
            commands::get_chat,
            commands::all_chats,
            commands::load_chat,
            commands::load_chat_log_by_cursor,
            commands::new_chat,
            commands::update_chat,
            commands::delete_chat,
            commands::all_non_stick_chats,
            commands::all_stick_chats,
            commands::set_chat_stick,
            commands::move_stick_chat,
            commands::move_non_stick_chat,
            commands::all_archive_chats,
            commands::set_chat_archive,
            commands::delete_chat_log,
            commands::send_message,
            commands::resend_message,
            commands::get_chat_models,
            commands::get_settings,
            commands::update_settings,
            commands::get_theme,
            commands::get_proxy,
            commands::has_api_key,
            commands::get_locale,
            commands::export_markdown,
            commands::all_prompts,
            commands::create_prompt,
            commands::update_prompt,
            commands::delete_prompt,
            commands::load_prompt,
            commands::get_prompt_sources,
            commands::get_prompt_source_prompts,
            commands::install_market_prompt,
            commands::install_market_prompt_and_create_chat,
            commands::show_or_create_window,
            commands::show_window,
            commands::create_window,
            commands::open,
            commands::debug_log,
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
