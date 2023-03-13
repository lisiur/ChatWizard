use askai_api::StreamContent;
use futures::StreamExt;
use tauri::{Manager, State, Window};
use uuid::Uuid;

use crate::result::Result;
use crate::{state::AppState, utils::create_topic};

#[tauri::command]
pub async fn send_message(
    message: String,
    window: Window,
    state: State<'_, AppState>,
) -> Result<Uuid> {
    let event_id = Uuid::new_v4();

    let topic = state.topic.clone();
    tokio::spawn(async move {
        let message_id = event_id;
        let event_id = event_id.to_string();
        match topic
            .lock()
            .await
            .send_message(&message, Some(message_id))
            .await
        {
            Ok(mut stream) => {
                while let Some(content) = stream.next().await {
                    window.emit(&event_id, content).unwrap();
                }
            }
            Err(err) => {
                window.emit(&event_id, StreamContent::Error(err)).unwrap();
            }
        }
    });

    Ok(event_id)
}

#[tauri::command]
pub async fn resend_message(id: Uuid, window: Window, state: State<'_, AppState>) -> Result<Uuid> {
    let event_id = id;

    let topic = state.topic.clone();
    tokio::spawn(async move {
        let event_id = event_id.to_string();
        match topic.lock().await.resend_message(id).await {
            Ok(mut stream) => {
                while let Some(content) = stream.next().await {
                    window.emit(&event_id, content).unwrap();
                }
            }
            Err(err) => {
                window.emit(&event_id, StreamContent::Error(err)).unwrap();
            }
        }
    });

    Ok(event_id)
}

#[tauri::command]
pub async fn reset_topic(state: State<'_, AppState>) -> Result<()> {
    let setting = state.setting.lock().await;
    *state.topic.lock().await = create_topic(&setting).await;
    Ok(())
}

#[tauri::command]
pub async fn set_api_key(api_key: String, state: State<'_, AppState>) -> Result<()> {
    let mut setting = state.setting.lock().await;
    setting.set_api_key(&api_key).unwrap();
    *state.topic.lock().await = create_topic(&setting).await;
    Ok(())
}

#[tauri::command]
pub async fn set_proxy(proxy: String, state: State<'_, AppState>) -> Result<()> {
    let mut setting = state.setting.lock().await;
    setting.set_proxy(&proxy).unwrap();

    let topic = state.topic.lock().await;
    topic.set_proxy(&proxy).await;
    Ok(())
}

#[tauri::command]
pub async fn get_proxy(state: State<'_, AppState>) -> Result<Option<String>> {
    let setting = state.setting.lock().await;
    Ok(setting.opts.proxy.clone())
}

#[tauri::command]
pub async fn has_api_key(state: State<'_, AppState>) -> Result<bool> {
    let setting = state.setting.lock().await;
    Ok(setting.opts.api_key.is_some())
}

#[tauri::command]
pub async fn show_main_window(window: Window) {
    window.get_window("main").unwrap().show().unwrap();
}
