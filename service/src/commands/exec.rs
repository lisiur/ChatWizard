use serde::Serialize;
use serde_json::{from_value, to_value};
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};

use super::cmd::*;
use crate::result::Result;
use crate::{DbConn, Error, Id};
pub trait IntoResult {
    fn into_result(self) -> Result<Box<dyn erased_serde::Serialize>>;
}

impl<T: Serialize + 'static> IntoResult for Result<T> {
    fn into_result(self) -> Result<Box<dyn erased_serde::Serialize>> {
        let value = self?;

        Ok(Box::new(value))
    }
}

#[derive(serde::Serialize, Debug)]
pub struct CommandEvent {
    pub name: String,
    pub payload: serde_json::Value,
}

#[derive(Clone, Default, Debug)]
pub struct CommandExecutor {
    stop_reply_sender_map: Arc<Mutex<HashMap<Id, oneshot::Sender<()>>>>,
}

impl CommandExecutor {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn exec_command<Fut>(
        &self,
        command: String,
        payload: serde_json::Value,
        conn: &DbConn,
        send: impl Fn(CommandEvent) -> Fut + Send + 'static,
    ) -> Result<Box<dyn erased_serde::Serialize>>
    where
        Fut: Future<Output = Result<()>> + Send,
    {
        log::debug!("exec_command: {} {:?}", command, payload);
        match command.as_ref() {
            "new_chat" => from_value::<NewChatCommand>(payload)?
                .exec(conn)
                .into_result(),

            "get_chat" => from_value::<GetChatCommand>(payload)?
                .exec(conn)
                .into_result(),

            "all_chats_except_casual" => from_value::<AllChatsExceptCasualCommand>(payload)?
                .exec(conn)
                .into_result(),

            "casual_chat" => from_value::<CasualChatCommand>(payload)?
                .exec(conn)
                .into_result(),

            "load_chat_log_by_cursor" => from_value::<LoadChatLogByCursorCommand>(payload)?
                .exec(conn)
                .into_result(),

            "update_chat" => from_value::<UpdateChatCommand>(payload)?
                .exec(conn)
                .into_result(),

            "remove_chat_prompt" => from_value::<RemoveChatPromptCommand>(payload)?
                .exec(conn)
                .into_result(),

            "delete_chat" => from_value::<DeleteChatCommand>(payload)?
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

            "update_chat_log" => from_value::<UpdateChatLogCommand>(payload)?
                .exec(conn)
                .into_result(),

            "delete_chat_log" => from_value::<DeleteChatLogCommand>(payload)?
                .exec(conn)
                .into_result(),

            "send_message" => {
                let command = from_value::<SendMessageCommand>(payload)?;
                let (mut receiver, stop_sender, message_id, reply_id) = command.exec(conn).await?;

                self.stop_reply_sender_map
                    .lock()
                    .await
                    .insert(message_id, stop_sender);

                let stop_reply_sender_map = self.stop_reply_sender_map.clone();
                tokio::spawn(async move {
                    let event_id = message_id.to_string();
                    while let Some(content) = receiver.recv().await {
                        let result = send(CommandEvent {
                            name: event_id.clone(),
                            payload: to_value(&content).unwrap(),
                        });
                        result.await.unwrap();
                    }

                    stop_reply_sender_map.lock().await.remove(&message_id);
                });

                Ok(Box::new((message_id, reply_id)))
            }

            "resend_message" => {
                let command = from_value::<ResendMessageCommand>(payload)?;
                let (mut receiver, stop_sender, message_id, reply_id) = command.exec(conn).await?;

                self.stop_reply_sender_map
                    .lock()
                    .await
                    .insert(message_id, stop_sender);

                let stop_reply_sender_map = self.stop_reply_sender_map.clone();
                tokio::spawn(async move {
                    let event_id = message_id.to_string();
                    while let Some(content) = receiver.recv().await {
                        let result = send(CommandEvent {
                            name: event_id.clone(),
                            payload: to_value(&content).unwrap(),
                        });
                        result.await.unwrap();
                    }

                    stop_reply_sender_map.lock().await.remove(&message_id);
                });

                Ok(Box::new((message_id, reply_id)))
            }

            "stop_reply" => {
                let command = from_value::<StopReplyCommand>(payload)?;
                let message_id = command.message_id;
                let sender = self.stop_reply_sender_map.lock().await.remove(&message_id);

                if let Some(sender) = sender {
                    sender.send(()).unwrap();
                }
                Ok(Box::new(()))
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

            "update_settings" => {
                let command = from_value::<UpdateSettingCommand>(payload)?;

                if let Some(theme) = &command.payload.theme {
                    send(CommandEvent {
                        name: "theme-changed".to_string(),
                        payload: to_value(theme).unwrap(),
                    })
                    .await
                    .unwrap();
                }

                if let Some(local) = &command.payload.language {
                    send(CommandEvent {
                        name: "locale-changed".to_string(),
                        payload: to_value(local).unwrap(),
                    })
                    .await
                    .unwrap();
                }

                command.exec(conn).into_result()
            }

            "get_locale" => from_value::<GetLocaleCommand>(payload)?
                .exec(conn)
                .into_result(),

            _ => Err(Error::Unknown(command)),
        }
    }
}
