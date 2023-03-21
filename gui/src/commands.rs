use std::path::PathBuf;

use askai_api::{OpenAIApi, StreamContent};
use tauri::{AppHandle, Manager, State, Window};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::chat::{ChatData, ChatMetadata, ChatUpdatePayload};
use crate::prompt::{Prompt, PromptMeta, PromptUpdatePayload};
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
    let chat_id = chat_manager.create_chat(prompt_id, title).await?;

    Ok(chat_id)
}

#[tauri::command]
pub async fn all_chats(state: State<'_, AppState>) -> Result<Vec<ChatMetadata>> {
    let chat_manager = state.chat_manager.lock().await;

    let chat_metadata_list = chat_manager.all_chat_meta().await;

    Ok(chat_metadata_list)
}

#[tauri::command]
pub async fn update_chat(payload: ChatUpdatePayload, state: State<'_, AppState>) -> Result<()> {
    let mut chat_manager = state.chat_manager.lock().await;

    log::debug!("Updating chat: {:?}", payload);

    chat_manager.update_chat(payload).await?;

    Ok(())
}

#[tauri::command]
pub async fn load_chat(chat_id: Uuid, state: State<'_, AppState>) -> Result<ChatData> {
    let chat_manager = state.chat_manager.lock().await;

    let chat = chat_manager.get_chat(chat_id).await?;
    let chat = chat.lock().await;

    let logs = chat.get_logs().await;
    let config = chat.config.clone();

    Ok(ChatData { logs, config })
}

#[tauri::command]
pub async fn delete_chat(chat_id: Uuid, state: State<'_, AppState>) -> Result<()> {
    let mut chat_manager = state.chat_manager.lock().await;

    chat_manager.delete_chat(chat_id).await?;

    Ok(())
}

#[tauri::command]
pub async fn save_as_markdown(
    chat_id: Uuid,
    path: PathBuf,
    state: State<'_, AppState>,
) -> Result<()> {
    let chat_manager = state.chat_manager.lock().await;

    let chat = chat_manager.get_chat(chat_id).await?;
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
    let chat = chat_manager.get_chat(chat_id).await?;
    let chat = chat.lock().await;
    let (sender, mut receiver) = mpsc::channel::<StreamContent>(20);
    let message_id = chat.send_message(sender, Some(&message), api).await?;

    let chat_id = chat.id;
    let chat_manager = state.chat_manager.clone();
    tokio::spawn(async move {
        let event_id = message_id.to_string();
        while let Some(content) = receiver.recv().await {
            window.emit(&event_id, content).unwrap();
        }
        chat_manager.lock().await.save_chat(chat_id).await.unwrap();
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
    let chat = chat_manager.lock().await.get_chat(chat_id).await?;
    let (sender, mut receiver) = mpsc::channel::<StreamContent>(20);
    chat.lock()
        .await
        .resend_message(sender, message_id, api)
        .await?;

    let chat_id = chat.lock().await.id;
    tokio::spawn(async move {
        let event_id = message_id.to_string();
        while let Some(content) = receiver.recv().await {
            window.emit(&event_id, content).unwrap();
        }
        chat_manager.lock().await.save_chat(chat_id).await.unwrap();
    });

    Ok(())
}

// prompts

#[tauri::command]
pub async fn all_prompts(state: State<'_, AppState>) -> Result<Vec<PromptMeta>> {
    let prompt_manager = state.prompt_manager.lock().await;

    let prompt_list = prompt_manager.all_prompt_meta().clone();

    Ok(prompt_list)
}

#[tauri::command]
pub async fn load_prompt(id: Uuid, state: State<'_, AppState>) -> Result<Option<Prompt>> {
    let mut prompt_manager = state.prompt_manager.lock().await;

    let prompt = prompt_manager.get_prompt(id).await?;

    Ok(prompt)
}

#[tauri::command]
pub async fn create_prompt(
    act: String,
    prompt: String,
    state: State<'_, AppState>,
) -> Result<Uuid> {
    let mut prompt_manager = state.prompt_manager.lock().await;

    prompt_manager.add_prompt(&act, &prompt).await
}

#[tauri::command]
pub async fn update_prompt(payload: PromptUpdatePayload, state: State<'_, AppState>) -> Result<()> {
    let mut prompt_manager = state.prompt_manager.lock().await;

    prompt_manager.update_prompt(&payload).await?;

    Ok(())
}

#[tauri::command]
pub async fn delete_prompt(id: Uuid, state: State<'_, AppState>) -> Result<()> {
    let mut prompt_manager = state.prompt_manager.lock().await;

    prompt_manager.delete_prompt(id).await?;

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
