use chat_wizard_service::commands::*;
use chat_wizard_service::{DbConn, Result as ServiceResult};
use serde::Serialize;
use serde_json::from_value;
use tauri::{AppHandle, Manager, State, Window};

use crate::error::Error;
use crate::result::Result;
use crate::window::{self, WindowOptions};

pub trait IntoResult {
    fn into_result(self) -> Result<Box<dyn erased_serde::Serialize>>;
}

impl<T: Serialize + 'static> IntoResult for ServiceResult<T> {
    fn into_result(self) -> Result<Box<dyn erased_serde::Serialize>> {
        let value = self?;

        Ok(Box::new(value))
    }
}

#[tauri::command]
pub async fn exec_command(
    command: String,
    payload: serde_json::Value,
    conn: State<'_, DbConn>,
    window: Window,
) -> Result<Box<dyn erased_serde::Serialize>> {
    match command.as_ref() {
        "new_chat" => from_value::<NewChatCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "get_chat" => from_value::<GetChatCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "all_chats_except_casual" => from_value::<AllChatsExceptCasualCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "casual_chat" => from_value::<CasualChatCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "load_chat_log_by_cursor" => from_value::<LoadChatLogByCursorCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "update_chat" => from_value::<UpdateChatCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "delete_chat" => from_value::<DeleteChatCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "set_chat_archive" => from_value::<SetChatArchiveCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "set_chat_stick" => from_value::<SetChatStickCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "move_stick_chat" => from_value::<MoveStickChatCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "move_non_stick_chat" => from_value::<MoveNonStickChatCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "update_chat_log" => from_value::<UpdateChatLogCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "delete_chat_log" => from_value::<DeleteChatLogCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "send_message" => {
            let command = from_value::<SendMessageCommand>(payload)?;
            let (mut receiver, message_id, reply_id) = command.exec(&conn).await?;
            tokio::spawn(async move {
                let event_id = message_id.to_string();
                while let Some(content) = receiver.recv().await {
                    window.emit(&event_id, content).unwrap();
                }
            });

            Ok(Box::new((message_id, reply_id)))
        }

        "resend_message" => {
            let command = from_value::<ResendMessageCommand>(payload)?;
            let (mut receiver, message_id, reply_id) = command.exec(&conn).await?;
            tokio::spawn(async move {
                let event_id = message_id.to_string();
                while let Some(content) = receiver.recv().await {
                    window.emit(&event_id, content).unwrap();
                }
            });

            Ok(Box::new((message_id, reply_id)))
        }

        "get_chat_models" => from_value::<GetChatModelsCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "all_prompts" => from_value::<AllPromptsCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "load_prompt" => from_value::<LoadPromptCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "create_prompt" => from_value::<CreatePromptCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "update_prompt" => from_value::<UpdatePromptCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "delete_prompt" => from_value::<DeletePromptCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "get_prompt_sources" => from_value::<GetPromptSourcesCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "get_prompt_source_prompts" => from_value::<GetPromptSourcePromptsCommand>(payload)?
            .exec(&conn)
            .await
            .into_result(),

        "install_market_prompt" => from_value::<InstallMarketPromptCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "install_market_prompt_and_create_chat" => {
            from_value::<InstallMarketPromptAndCreateChatCommand>(payload)?
                .exec(&conn)
                .into_result()
        }

        "get_settings" => from_value::<GetSettingsCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "get_theme" => from_value::<GetThemeCommand>(payload)?
            .exec(&conn)
            .into_result(),

        "update_settings" => {
            let command = from_value::<UpdateSettingCommand>(payload)?;

            if let Some(theme) = &command.payload.theme {
                let windows = window.windows();
                windows.values().for_each(|win| {
                    win.emit("theme-changed", theme).unwrap();
                });
            }

            if let Some(local) = &command.payload.language {
                let windows = window.windows();
                windows.values().for_each(|win| {
                    win.emit("locale-changed", local).unwrap();
                });
            }

            command.exec(&conn).into_result()
        }

        "get_locale" => from_value::<GetLocaleCommand>(payload)?
            .exec(&conn)
            .into_result(),

        _ => Err(Error::UnknownCommand(command)),
    }
}

#[tauri::command]
pub async fn show_window(label: &str, window: Window) -> Result<()> {
    log::debug!("show_window: {}", label);
    window::show_window(label, window)?;

    Ok(())
}

#[tauri::command]
pub async fn show_or_create_window(
    label: &str,
    options: WindowOptions,
    window: Window,
    handle: AppHandle,
) -> Result<()> {
    log::debug!("show_or_create_window: {} {:?}", label, options);
    window::show_or_create_window(label, window, handle, options)?;

    Ok(())
}

#[tauri::command]
pub async fn create_window(label: &str, options: WindowOptions, handle: AppHandle) -> Result<()> {
    log::debug!("create_window: {} {:?}", label, options);
    window::create_window(label, options, handle)?;

    Ok(())
}

#[tauri::command]
pub async fn open(url: String) -> Result<()> {
    log::debug!("open: {}", url);
    open::that(url)?;

    Ok(())
}
