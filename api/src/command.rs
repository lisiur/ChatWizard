use std::{collections::HashSet, sync::Arc};

use crate::{result::Result, Error};
use axum::extract::ws::Message;
use chat_wizard_service::{commands::*, result::Result as ServiceResult, Id};
use serde::Serialize;
use serde_json::{from_value, json};
use tokio::sync::Mutex;
pub trait IntoResult {
    fn into_result(self) -> Result<Box<dyn erased_serde::Serialize>>;
}

impl<T: Serialize + 'static> IntoResult for ServiceResult<T> {
    fn into_result(self) -> Result<Box<dyn erased_serde::Serialize>> {
        let value = self?;

        Ok(Box::new(value))
    }
}

use crate::AppState;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandPayload {
    pub command: String,
    pub payload: serde_json::Value,
    #[serde(default)]
    pub user_id: Id,
}

pub async fn handle_command(
    params: CommandPayload,
    state: AppState,
    client_id: Id,
) -> Result<Box<dyn erased_serde::Serialize>> {
    let conn = &state.conn;
    let users = state.users.clone();
    let CommandPayload {
        command,
        payload,
        user_id,
    } = params;

    match command.as_ref() {
        "connect" => {
            let user_id = Id::local();
            let mut map = users.lock().await;
            map.entry(user_id)
                .or_insert_with(|| Arc::new(Mutex::new(HashSet::new())));
            map.get(&user_id).unwrap().lock().await.insert(client_id);

            Ok(Box::new(()))
        }

        "new_chat" => from_value::<NewChatCommand>(payload)?
            .exec(conn)
            .into_result(),

        "get_chat" => from_value::<GetChatCommand>(payload)?
            .exec(conn)
            .into_result(),

        "all_chats" => from_value::<AllChatsCommand>(payload)?
            .exec(conn)
            .into_result(),

        "load_chat_log_by_cursor" => from_value::<LoadChatLogByCursorCommand>(payload)?
            .exec(conn)
            .into_result(),

        "update_chat" => from_value::<UpdateChatCommand>(payload)?
            .exec(conn)
            .into_result(),

        "delete_chat" => from_value::<DeleteChatCommand>(payload)?
            .exec(conn)
            .into_result(),

        "all_non_stick_chats" => from_value::<AllNonStickChatsCommand>(payload)?
            .exec(conn)
            .into_result(),

        "all_stick_chats" => from_value::<AllStickChatsCommand>(payload)?
            .exec(conn)
            .into_result(),

        "all_archive_chats" => from_value::<AllArchiveChatsCommand>(payload)?
            .exec(conn)
            .into_result(),

        "set_chat_archive" => from_value::<SetChatArchiveCommand>(payload)?
            .exec(conn)
            .into_result(),

        "set_chat_stick" => from_value::<SetChatStickCommand>(payload)?
            .exec(conn)
            .into_result(),

        "move_stick_chat" => from_value::<MoveStickChatCommand>(payload)?
            .exec(conn)
            .into_result(),

        "move_non_stick_chat" => from_value::<MoveNonStickChatCommand>(payload)?
            .exec(conn)
            .into_result(),

        "delete_chat_log" => from_value::<DeleteChatLogCommand>(payload)?
            .exec(conn)
            .into_result(),

        "send_message" => {
            let command = from_value::<SendMessageCommand>(payload)?;
            let (mut receiver, message_id, reply_id) = command.exec(conn).await?;
            tokio::spawn(async move {
                while let Some(content) = receiver.recv().await {
                    let payload = json!({
                        "id": message_id,
                        "payload": content,
                    });
                    let message = Message::Text(serde_json::to_string(&payload).unwrap());
                    state.send_message(user_id, message).await;
                }
            });

            Ok(Box::new((message_id, reply_id)))
        }

        "resend_message" => {
            let user_id = Id::local();
            let command = from_value::<ResendMessageCommand>(payload)?;
            let (mut receiver, message_id, reply_id) = command.exec(conn).await?;
            tokio::spawn(async move {
                while let Some(content) = receiver.recv().await {
                    let payload = json!({
                        "id": message_id,
                        "payload": content,
                    });
                    let message = Message::Text(serde_json::to_string(&payload).unwrap());
                    state.send_message(user_id, message).await;
                }
            });

            Ok(Box::new((message_id, reply_id)))
        }

        "get_chat_models" => from_value::<GetChatModelsCommand>(payload)?
            .exec(conn)
            .into_result(),

        "all_prompts" => from_value::<AllPromptsCommand>(payload)?
            .exec(conn)
            .into_result(),

        "load_prompt" => from_value::<LoadPromptCommand>(payload)?
            .exec(conn)
            .into_result(),

        "create_prompt" => from_value::<CreatePromptCommand>(payload)?
            .exec(conn)
            .into_result(),

        "update_prompt" => from_value::<UpdatePromptCommand>(payload)?
            .exec(conn)
            .into_result(),

        "delete_prompt" => from_value::<DeletePromptCommand>(payload)?
            .exec(conn)
            .into_result(),

        "get_prompt_sources" => from_value::<GetPromptSourcesCommand>(payload)?
            .exec(conn)
            .into_result(),

        "get_prompt_source_prompts" => from_value::<GetPromptSourcePromptsCommand>(payload)?
            .exec(conn)
            .await
            .into_result(),

        "install_market_prompt" => from_value::<InstallMarketPromptCommand>(payload)?
            .exec(conn)
            .into_result(),

        "install_market_prompt_and_create_chat" => {
            from_value::<InstallMarketPromptAndCreateChatCommand>(payload)?
                .exec(conn)
                .into_result()
        }

        "get_settings" => from_value::<GetSettingsCommand>(payload)?
            .exec(conn)
            .into_result(),

        "get_theme" => from_value::<GetThemeCommand>(payload)?
            .exec(conn)
            .into_result(),

        "update_settings" => from_value::<UpdateSettingCommand>(payload)?
            .exec(conn)
            .into_result(),

        "get_locale" => from_value::<GetLocaleCommand>(payload)?
            .exec(conn)
            .into_result(),

        _ => Err(Error::UnknownCommand(command)),
    }
}
