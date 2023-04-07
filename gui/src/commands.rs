use chat_wizard_service::commands::{CommandEvent, CommandExecutor};
use chat_wizard_service::DbConn;
use tauri::{AppHandle, State, Window};

use crate::result::Result;
use crate::window::{self, WindowOptions};
use crate::EventBus;

#[tauri::command]
pub async fn exec_command(
    command: String,
    payload: serde_json::Value,
    conn: State<'_, DbConn>,
    event_bus: State<'_, EventBus>,
    executor: State<'_, CommandExecutor>,
) -> Result<Box<dyn erased_serde::Serialize>> {
    let sender = event_bus.sender.clone();
    let send = move |event: CommandEvent| {
        let inner_sender = sender.clone();
        async move {
            inner_sender.clone().send(event).await.unwrap();
            Ok(())
        }
    };
    let result = executor.exec_command(command, payload, &conn, send).await?;

    Ok(result)
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
