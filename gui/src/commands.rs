use askai_api::{ChatLog, OpenAIApi, StreamContent};
use tauri::{Manager, State, Window};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::result::Result;
use crate::state::AppState;
use crate::store::ChatMetadata;

#[tauri::command]
pub async fn new_chat(
    topic: Option<String>,
    title: Option<String>,
    state: State<'_, AppState>,
) -> Result<Uuid> {
    let title = title.as_deref().unwrap_or("New Chat");
    let chat_id = state.create_chat(topic, title).await?;

    Ok(chat_id)
}

#[tauri::command]
pub async fn all_chats(state: State<'_, AppState>) -> Result<Vec<ChatMetadata>> {
    let store = state.store.lock().await;
    let chat_metadata_list = store.all_chats().await?;

    Ok(chat_metadata_list)
}

#[tauri::command]
pub async fn read_chat(chat_id: Uuid, state: State<'_, AppState>) -> Result<Vec<ChatLog>> {
    let chat = state.read_chat(chat_id).await?;

    let chat_log = chat.lock().await.topic.lock().await.logs.clone();

    Ok(chat_log)
}

#[tauri::command]
pub async fn delete_chat(chat_id: Uuid, state: State<'_, AppState>) -> Result<()> {
    state.delete_chat(chat_id).await?;

    Ok(())
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

    let store = state.store.clone();
    let chat = state.get_chat(chat_id).await;
    tokio::spawn(async move {
        let event_id = message_id.to_string();
        while let Some(content) = receiver.recv().await {
            window.emit(&event_id, content).unwrap();
        }
        let chat = chat.lock().await;
        store.lock().await.save_chat(&chat).await.unwrap();
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

    let store = state.store.clone();
    let chat = state.get_chat(chat_id).await;
    tokio::spawn(async move {
        let event_id = message_id.to_string();
        while let Some(content) = receiver.recv().await {
            window.emit(&event_id, content).unwrap();
        }
        let chat = chat.lock().await;
        store.lock().await.save_chat(&chat).await.unwrap();
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
