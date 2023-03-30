#![allow(unused)]
use std::path::PathBuf;

use askai_service::models::chat::ChatConfig;
use askai_service::models::chat_log::ChatLog;
use askai_service::models::setting::Setting;
use askai_service::{
    Chat, ChatService, CreateChatPayload, CreatePromptPayload, DeleteChatPayload, Id, PatchSetting,
    Prompt, PromptIndex, PromptService, ResendMessagePayload, SearchChatLogPayload,
    SearchChatPayload, SearchPromptPayload, SendMessagePayload, SettingService, StreamContent,
    Theme, UpdateChatPayload, UpdatePromptPayload, UpdateSettingPayload,
};
use tauri::{AppHandle, Manager, State, Window};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::result::Result;
use crate::window::{self, WindowOptions};

// chats

#[tauri::command]
pub async fn new_chat(
    title: Option<String>,
    prompt_id: Option<Id>,
    chat_service: State<'_, ChatService>,
) -> Result<Id> {
    let title = title.as_deref().unwrap_or("New Chat");
    let chat_id = chat_service.create_chat(CreateChatPayload {
        title: title.to_string(),
        user_id: Id::local(),
        prompt_id,
        vendor: "openai".to_string(),
        config: ChatConfig::default(),
    })?;

    Ok(chat_id)
}

#[tauri::command]
pub async fn get_chat(id: Id, chat_service: State<'_, ChatService>) -> Result<Chat> {
    let chat = chat_service.get_chat(id)?;

    Ok(chat)
}

#[tauri::command]
pub async fn all_chats(chat_service: State<'_, ChatService>) -> Result<Vec<Chat>> {
    let records = chat_service.search_chats(SearchChatPayload::default())?;

    Ok(records.records)
}

#[tauri::command]
pub async fn update_chat(
    payload: UpdateChatPayload,
    chat_service: State<'_, ChatService>,
) -> Result<()> {
    chat_service.update_chat(payload)?;

    Ok(())
}

#[tauri::command]
pub fn load_chat(chat_id: Id, chat_service: State<'_, ChatService>) -> Result<Vec<ChatLog>> {
    let chat = chat_service.search_chat_logs(SearchChatLogPayload {
        chat_id: Some(chat_id),
        user_id: Id::local(),
        ..Default::default()
    })?;

    Ok(chat.records)
}

#[tauri::command]
pub async fn delete_chat(chat_id: Id, chat_service: State<'_, ChatService>) -> Result<()> {
    chat_service.delete_chat(DeleteChatPayload { id: chat_id })?;

    Ok(())
}

#[tauri::command]
pub async fn export_markdown(
    chat_id: Uuid,
    path: PathBuf,
    chat_service: State<'_, ChatService>,
) -> Result<()> {
    todo!();
}

#[tauri::command]
pub async fn send_message(
    chat_id: Id,
    message: String,
    window: Window,
    chat_service: State<'_, ChatService>,
) -> Result<Id> {
    let (sender, mut receiver) = mpsc::channel::<StreamContent>(20);

    let (message_id, reply_id, _) = chat_service
        .send_message(SendMessagePayload { chat_id, message }, sender)
        .await?;

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
    message_id: Id,
    window: Window,
    chat_service: State<'_, ChatService>,
) -> Result<Id> {
    let (sender, mut receiver) = mpsc::channel::<StreamContent>(20);

    let (message_id, reply_id, _) = chat_service
        .resend_message(ResendMessagePayload { id: message_id }, sender)
        .await?;

    tokio::spawn(async move {
        let event_id = message_id.to_string();
        while let Some(content) = receiver.recv().await {
            window.emit(&event_id, content).unwrap();
        }
    });

    Ok(message_id)
}

// prompts

#[tauri::command]
pub async fn all_prompts(prompt_service: State<'_, PromptService>) -> Result<Vec<PromptIndex>> {
    let res = prompt_service.search_prompts(SearchPromptPayload::default())?;

    Ok(res.records)
}

#[tauri::command]
pub async fn load_prompt(id: Id, prompt_service: State<'_, PromptService>) -> Result<Prompt> {
    let prompt = prompt_service.get_prompt(id)?;

    Ok(prompt)
}

#[tauri::command]
pub async fn create_prompt(
    name: String,
    content: String,
    prompt_service: State<'_, PromptService>,
) -> Result<Id> {
    let id = prompt_service.create_prompt(CreatePromptPayload {
        name,
        content,
        user_id: Id::local(),
    })?;

    Ok(id)
}

#[tauri::command]
pub async fn update_prompt(
    payload: UpdatePromptPayload,
    prompt_service: State<'_, PromptService>,
) -> Result<()> {
    prompt_service.update_prompt(payload)?;

    Ok(())
}

#[tauri::command]
pub async fn delete_prompt(id: Id, prompt_service: State<'_, PromptService>) -> Result<()> {
    prompt_service.delete_prompt(id)?;

    Ok(())
}

// settings

#[tauri::command]
pub fn get_settings(setting_service: State<'_, SettingService>) -> Result<Setting> {
    let setting = setting_service.get_setting(Id::local())?;
    Ok(setting)
}

#[tauri::command]
pub fn get_theme(setting_service: State<'_, SettingService>) -> Result<Theme> {
    let setting = setting_service.get_setting(Id::local())?;

    Ok(setting.theme.0)
}

#[tauri::command]
pub async fn update_settings(
    mut payload: UpdateSettingPayload,
    setting_service: State<'_, SettingService>,
    window: Window,
) -> Result<()> {
    if let Some(theme) = &payload.theme {
        let windows = window.windows();
        windows.values().for_each(|win| {
            win.emit("theme-changed", theme).unwrap();
        });
    }

    if let Some(local) = &payload.language {
        let windows = window.windows();
        windows.values().for_each(|win| {
            win.emit("locale-changed", local).unwrap();
        });
    }

    payload.user_id = Some(Id::local());
    setting_service.update_setting(payload)?;

    Ok(())
}

#[tauri::command]
pub async fn get_proxy(setting_service: State<'_, SettingService>) -> Result<Option<String>> {
    let setting = setting_service.get_setting(Id::local())?;

    Ok(setting.proxy)
}

#[tauri::command]
pub async fn has_api_key(setting_service: State<'_, SettingService>) -> Result<bool> {
    let setting = setting_service.get_setting(Id::local())?;

    Ok(setting.api_key.is_some())
}

#[tauri::command]
pub async fn get_locale(setting_service: State<'_, SettingService>) -> Result<String> {
    let setting = setting_service.get_setting(Id::local())?;

    Ok(setting.language)
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
