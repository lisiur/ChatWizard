#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod error;
mod result;
mod setting;
mod state;
mod utils;

use state::AppState;

#[tokio::main]
async fn main() {
    env_logger::init();
    tauri::Builder::default()
        .manage(AppState::init().await.unwrap())
        .invoke_handler(tauri::generate_handler![
            commands::send_message,
            commands::resend_message,
            commands::reset_topic,
            commands::set_api_key,
            commands::set_proxy,
            commands::get_proxy,
            commands::has_api_key,
            commands::show_main_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
