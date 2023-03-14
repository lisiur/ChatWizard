use askai_api::{OpenAIApi, StreamContent};
use tauri::{Manager, State, Window};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::result::Result;
use crate::state::AppState;

#[tauri::command]
pub async fn new_chat(topic: Option<String>, state: State<'_, AppState>) -> Result<Uuid> {
    let chat_id = state.add_chat(topic).await;

    Ok(chat_id)
}

#[tauri::command]
pub async fn send_message(
    chat_id: Uuid,
    message: String,
    window: Window,
    state: State<'_, AppState>,
) -> Result<Uuid> {
    let api = state.create_api().await?;
    let chat = state.get_chat(chat_id).await;
    let (sender, mut receiver) = mpsc::channel::<StreamContent>(20);
    let message_id = chat.lock().await.send_message(sender, &message, api).await;

    tokio::spawn(async move {
        let event_id = message_id.to_string();
        while let Some(content) = receiver.recv().await {
            window.emit(&event_id, content).unwrap();
        }
    });

    Ok(message_id)
}

#[tauri::command]
pub async fn resend_message(
    chat_id: Uuid,
    message_id: Uuid,
    window: Window,
    state: State<'_, AppState>,
) -> Result<()> {
    let api = state.create_api().await?;
    let chat = state.get_chat(chat_id).await;
    let (sender, mut receiver) = mpsc::channel::<StreamContent>(20);
    chat.lock()
        .await
        .resend_message(sender, message_id, api)
        .await?;

    tokio::spawn(async move {
        let event_id = message_id.to_string();
        while let Some(content) = receiver.recv().await {
            window.emit(&event_id, content).unwrap();
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn reset_chat(chat_id: Uuid, state: State<'_, AppState>) -> Result<()> {
    state.get_chat(chat_id).await.lock().await.reset().await;
    Ok(())
}

#[tauri::command]
pub async fn set_api_key(api_key: String, state: State<'_, AppState>) -> Result<()> {
    state.set_api_key(&api_key).await?;

    OpenAIApi::check_api_key(&api_key).await?;

    Ok(())
}

#[tauri::command]
pub async fn check_api_key(api_key: String) -> Result<()> {
    OpenAIApi::check_api_key(&api_key).await?;
    Ok(())
}

#[tauri::command]
pub async fn set_proxy(proxy: String, state: State<'_, AppState>) -> Result<()> {
    state.set_proxy(&proxy).await?;

    Ok(())
}

#[tauri::command]
pub async fn get_proxy(state: State<'_, AppState>) -> Result<Option<String>> {
    state.get_proxy().await
}

#[tauri::command]
pub async fn has_api_key(state: State<'_, AppState>) -> Result<bool> {
    state.has_api_key().await
}

#[tauri::command]
pub async fn show_main_window(window: Window) {
    window.get_window("main").unwrap().show().unwrap();
}
