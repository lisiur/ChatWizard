#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use askai_service::{
    services::prompt_market::PromptMarketService, ChatService, PromptService, SettingService,
};
use project::Project;
use tauri::{WindowBuilder, WindowUrl};

mod commands;
mod error;
mod project;
mod result;
mod utils;
mod window;

#[tokio::main]
async fn main() {
    env_logger::init();
    let project = Project::init().await.unwrap();
    let conn = askai_service::init(&project.db_url).unwrap();
    tauri::Builder::default()
        .manage(SettingService::new(conn.clone()))
        .manage(ChatService::new(conn.clone()))
        .manage(PromptService::new(conn.clone()))
        .manage(PromptMarketService::new(conn))
        .setup(|app| {
            let mut main_window_builder =
                WindowBuilder::new(app, "main", WindowUrl::App("index.html".into()))
                    .title("AskAI")
                    .inner_size(860.0, 720.0)
                    .min_inner_size(720.0, 640.0)
                    .resizable(true)
                    .visible(false);

            #[cfg(target_os = "macos")]
            {
                main_window_builder = main_window_builder
                    .title("")
                    .title_bar_style(tauri::TitleBarStyle::Overlay);
            }

            main_window_builder.build().unwrap();
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
            commands::show_window,
            commands::debug_log,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
