use std::path::PathBuf;

use askai_api::{OpenAIApi, StreamContent};
use serde_json::json;
use tauri::{AppHandle, Manager, State, Window};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::chat::chat_manager::ChatUpdatePayload;
use crate::chat::chat_store::{ChatData, ChatIndex, ChatMetadata};
use crate::error::Error;
use crate::prompt::prompt_manager::PromptUpdatePayload;
use crate::prompt::prompt_store::{PromptData, PromptIndex, PromptMetadata};
use crate::result::Result;
use crate::setting::{Settings, Theme};
use crate::state::AppState;
use crate::window::{self, WindowOptions};

// chats

#[tauri::command]
pub async fn new_chat(
    title: Option<String>,
    prompt_id: Option<Uuid>,
    state: State<'_, AppState>,
) -> Result<Uuid> {
    let mut chat_manager = state.chat_manager.lock().await;

    let title = title.as_deref().unwrap_or("New Chat");
    let chat_id = chat_manager.create(title, prompt_id).await?;

    Ok(chat_id)
}

#[tauri::command]
pub async fn all_chats(state: State<'_, AppState>) -> Result<Vec<ChatIndex>> {
    let chat_manager = state.chat_manager.lock().await;

    let index_list = chat_manager.list();

    Ok(index_list)
}

#[tauri::command]
pub async fn update_chat(payload: ChatUpdatePayload, state: State<'_, AppState>) -> Result<()> {
    let mut chat_manager = state.chat_manager.lock().await;

    log::debug!("Updating chat: {:?}", payload);

    chat_manager.update(&payload).await?;

    Ok(())
}

#[tauri::command]
pub async fn load_chat(
    chat_id: Uuid,
    state: State<'_, AppState>,
) -> Result<(ChatMetadata, ChatData)> {
    let chat_manager = state.chat_manager.lock().await;

    let chat = chat_manager.load(chat_id).await?;
    let chat = chat.lock().await;

    let metadata = chat.as_metadata();
    let data = chat.as_data().await;

    Ok((metadata, data))
}

#[tauri::command]
pub async fn delete_chat(chat_id: Uuid, state: State<'_, AppState>) -> Result<()> {
    let mut chat_manager = state.chat_manager.lock().await;

    chat_manager.delete(chat_id).await?;

    Ok(())
}

#[tauri::command]
pub async fn save_as_markdown(
    chat_id: Uuid,
    path: PathBuf,
    state: State<'_, AppState>,
) -> Result<()> {
    let chat_manager = state.chat_manager.lock().await;

    let chat = chat_manager.load(chat_id).await?;
    let chat = chat.lock().await;
    chat.save_as_markdown(path.as_path()).await?;

    Ok(())
}

#[tauri::command]
pub async fn send_message(
    chat_id: Uuid,
    message: String,
    window: Window,
    state: State<'_, AppState>,
) -> Result<Uuid> {
    let setting = state.setting.lock().await;
    let chat_manager = state.chat_manager.lock().await;

    let api = setting.create_api().await?;
    let chat = chat_manager.load(chat_id).await?;
    let chat = chat.lock().await;
    let (sender, mut receiver) = mpsc::channel::<StreamContent>(20);
    let message_id = chat.send_message(sender, &message, api).await?;

    let chat_id = chat.index.id;
    let chat_manager = state.chat_manager.clone();

    tokio::spawn(async move {
        let event_id = message_id.to_string();
        while let Some(content) = receiver.recv().await {
            window.emit(&event_id, content).unwrap();
        }
        // save message
        let chat_manager = chat_manager.lock().await;
        let chat = chat_manager.load(chat_id).await.unwrap();
        let cost = chat.lock().await.get_cost();
        chat_manager.save_data(chat_id).await.unwrap();
        window.emit(&format!("{event_id}-cost"), json!({ "cost": cost }))
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
    let setting = state.setting.lock().await;
    let chat_manager = state.chat_manager.clone();

    let api = setting.create_api().await?;
    let chat = chat_manager.lock().await.load(chat_id).await?;
    let (sender, mut receiver) = mpsc::channel::<StreamContent>(20);
    chat.lock()
        .await
        .resend_message(sender, message_id, api)
        .await?;

    let chat_id = chat.lock().await.index.id;
    tokio::spawn(async move {
        let event_id = message_id.to_string();
        while let Some(content) = receiver.recv().await {
            window.emit(&event_id, content).unwrap();
        }
        chat_manager.lock().await.save_data(chat_id).await.unwrap();
    });

    Ok(())
}

// prompts

#[tauri::command]
pub async fn all_prompts(state: State<'_, AppState>) -> Result<Vec<PromptIndex>> {
    let prompt_manager = state.prompt_manager.lock().await;

    let prompt_list = prompt_manager.list();

    Ok(prompt_list)
}

#[tauri::command]
pub async fn load_prompt(
    id: Uuid,
    state: State<'_, AppState>,
) -> Result<(PromptMetadata, PromptData)> {
    let mut prompt_manager = state.prompt_manager.lock().await;

    let prompt = prompt_manager
        .load(id)
        .await?
        .ok_or(Error::NotFound("prompt".to_string()))?;

    Ok((prompt.as_metadata(), prompt.as_data()))
}

#[tauri::command]
pub async fn create_prompt(
    act: String,
    prompt: String,
    state: State<'_, AppState>,
) -> Result<Uuid> {
    let mut prompt_manager = state.prompt_manager.lock().await;

    prompt_manager.create(&act, &prompt).await
}

#[tauri::command]
pub async fn update_prompt(payload: PromptUpdatePayload, state: State<'_, AppState>) -> Result<()> {
    let mut prompt_manager = state.prompt_manager.lock().await;

    prompt_manager.update(&payload).await?;

    Ok(())
}

#[tauri::command]
pub async fn delete_prompt(id: Uuid, state: State<'_, AppState>) -> Result<()> {
    let mut prompt_manager = state.prompt_manager.lock().await;

    prompt_manager.delete(id).await?;

    Ok(())
}

// settings

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Settings> {
    let setting = state.setting.lock().await;

    Ok(setting.settings.clone())
}

#[tauri::command]
pub async fn get_theme(state: State<'_, AppState>) -> Result<Theme> {
    let setting = state.setting.lock().await;

    Ok(setting.get_theme())
}

#[tauri::command]
pub async fn set_theme(theme: Theme, state: State<'_, AppState>, window: Window) -> Result<()> {
    let mut setting = state.setting.lock().await;

    setting.set_theme(theme.clone()).await?;

    let windows = window.windows();
    windows.values().for_each(|win| {
        win.emit("theme-changed", theme.clone()).unwrap();
    });

    Ok(())
}

#[tauri::command]
pub async fn set_api_key(api_key: String, state: State<'_, AppState>) -> Result<()> {
    let mut setting = state.setting.lock().await;

    setting.set_api_key(&api_key).await?;

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
    let mut setting = state.setting.lock().await;

    setting.set_proxy(&proxy).await?;

    Ok(())
}

#[tauri::command]
pub async fn get_proxy(state: State<'_, AppState>) -> Result<Option<String>> {
    let mut setting = state.setting.lock().await;

    Ok(setting.get_proxy().clone())
}

#[tauri::command]
pub async fn has_api_key(state: State<'_, AppState>) -> Result<bool> {
    let setting = state.setting.lock().await;

    Ok(setting.has_api_key())
}

#[tauri::command]
pub async fn get_locale(state: State<'_, AppState>) -> Result<String> {
    let setting = state.setting.lock().await;

    Ok(setting.get_locale())
}

#[tauri::command]
pub async fn set_locale(locale: String, state: State<'_, AppState>, window: Window) -> Result<()> {
    let mut setting = state.setting.lock().await;

    setting.set_locale(&locale).await?;

    let windows = window.windows();
    windows.values().for_each(|win| {
        win.emit("locale-changed", locale.clone()).unwrap();
    });

    Ok(())
}

// others

#[tauri::command]
pub async fn show_window(
    label: String,
    options: Option<WindowOptions>,
    window: Window,
    handle: AppHandle,
) -> Result<()> {
    log::debug!("show_window: {} {:?}", label, options);
    window::show_window_lazy(label, options, window, handle)
}

#[tauri::command]
pub async fn debug_log(log: String) -> Result<()> {
    log::debug!("[debug] {}", log);
    Ok(())
}
