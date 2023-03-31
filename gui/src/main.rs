#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use askai_service::{
    services::prompt_market::PromptMarketService, ChatService, PromptService, SettingService,
};
use project::Project;
use window::{create_window, WindowOptions};

mod commands;
mod error;
mod project;
mod result;
mod tray;
mod uri_schema_handler;
mod utils;
mod window;

#[tokio::main]
async fn main() {
    env_logger::init();
    let project = Project::init().await.unwrap();
    let conn = askai_service::init(&project.db_url).unwrap();

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
                    title: "AskAI".to_string(),
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
            commands::new_chat,
            commands::update_chat,
            commands::delete_chat,
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
            commands::debug_log,
        ])
        .register_uri_scheme_protocol("askai", uri_schema_handler::uri_schema_handler)
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
