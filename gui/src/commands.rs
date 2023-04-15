use chat_wizard_service::commands::{CommandEvent, CommandExecutor};
use chat_wizard_service::DbConn;
use tauri::api::dialog::FileDialogBuilder;
use tauri::{AppHandle, State, Window};

use crate::result::Result;
use crate::window::{self, WindowOptions};
use crate::{utils, EventBus};

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
    window::focus_window(&window);

    Ok(())
}

#[tauri::command]
pub async fn show_or_create_window(
    label: &str,
    options: WindowOptions,
    handle: AppHandle,
) -> Result<()> {
    log::debug!("show_or_create_window: {} {:?}", label, options);
    window::show_or_create_window_in_background(&handle, label, options)?;

    Ok(())
}

#[tauri::command]
pub async fn create_window(label: &str, options: WindowOptions, handle: AppHandle) -> Result<()> {
    log::debug!("create_window: {} {:?}", label, options);
    window::create_window_in_background(&handle, label, options)?;

    Ok(())
}

#[tauri::command]
pub async fn open(url: String) -> Result<()> {
    log::debug!("open: {}", url);
    open::that(url)?;

    Ok(())
}

#[tauri::command]
pub async fn save_file(file_name: String, data: Vec<u8>) -> Result<()> {
    log::debug!("export_image");
    FileDialogBuilder::new()
        .set_file_name(&file_name)
        .save_file(move |file_path| {
            if let Some(path) = file_path {
                utils::save_file_sync(&path, &data).unwrap();
            }
        });
    Ok(())
}

#[tauri::command]
pub async fn debug_log(message: String) -> Result<()> {
    log::debug!("{}", message);

    Ok(())
}
