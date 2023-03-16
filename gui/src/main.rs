#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod chat;
mod commands;
mod error;
mod project;
mod result;
mod setting;
mod state;
mod store;

#[tokio::main]
async fn main() {
    env_logger::init();
    tauri::Builder::default()
        .manage(state::AppState::init().await.unwrap())
        .invoke_handler(tauri::generate_handler![
            commands::all_chats,
            commands::read_chat,
            commands::new_chat,
            commands::delete_chat,
            commands::send_message,
            commands::resend_message,
            commands::reset_chat,
            commands::set_api_key,
            commands::check_api_key,
            commands::set_proxy,
            commands::get_proxy,
            commands::has_api_key,
            commands::show_main_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
