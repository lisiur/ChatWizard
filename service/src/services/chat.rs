use chrono::Utc;
use futures::StreamExt;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::Receiver;
use tokio::task::JoinHandle;

use crate::api::openai::chat::params::{OpenAIChatMessage, OpenAIChatParams, OpenAIChatRole};
use crate::database::pagination::PaginatedRecords;
use crate::models::chat::{Chat, NewChat, PatchChat};
use crate::models::chat_log::{ChatLog, NewChatLog, PatchChatLog, Role};
use crate::models::chat_model::ChatModel;
use crate::repositories::chat::ChatRepo;
use crate::repositories::chat_log::{ChatLogQueryParams, ChatLogRepo};
use crate::repositories::chat_model::ChatModelRepo;
use crate::repositories::prompt::PromptRepo;
use crate::repositories::setting::SettingRepo;
use crate::result::Result;
use crate::types::{PageQueryParams, StreamContent};
use crate::{database::DbConn, models::chat::ChatConfig, types::Id};
use crate::{CursorDirection, CursorQueryResult};

#[derive(Clone)]
pub struct ChatService {
    #[allow(unused)]
    conn: DbConn,
    chat_repo: ChatRepo,
    chat_log_repo: ChatLogRepo,
    prompt_repo: PromptRepo,
    setting_repo: SettingRepo,
    chat_model_repo: ChatModelRepo,
}

impl From<DbConn> for ChatService {
    fn from(conn: DbConn) -> Self {
        Self::new(conn)
    }
}

impl ChatService {
    pub fn new(conn: DbConn) -> Self {
        Self {
            chat_repo: ChatRepo::new(conn.clone()),
            chat_log_repo: ChatLogRepo::new(conn.clone()),
            chat_model_repo: ChatModelRepo::new(conn.clone()),
            prompt_repo: PromptRepo::new(conn.clone()),
            setting_repo: SettingRepo::new(conn.clone()),
            conn,
        }
    }

    pub fn create_chat(&self, payload: CreateChatPayload) -> Result<Id> {
        let chat_id = Id::random();

        let non_stick_min_order = self.chat_repo.select_non_stick_min_order(payload.user_id)?;

        let new_chat = NewChat {
            id: chat_id,
            user_id: payload.user_id,
            title: payload.title,
            prompt_id: payload.prompt_id,
            config: payload.config.into(),
            vendor: payload.vendor,
            sort: non_stick_min_order - 1,
            ..Default::default()
        };

        self.chat_repo.insert(&new_chat)?;

        Ok(chat_id)
    }

    pub fn get_chat(&self, id: Id) -> Result<Chat> {
        let chat = self.chat_repo.select_by_id(id)?;

        Ok(chat)
    }

    pub fn get_casual_chat(&self, user_id: Id) -> Result<Chat> {
        let chat = self.chat_repo.select_casual(user_id)?;

        Ok(chat)
    }

    pub fn get_all_chats_except_casual(&self, user_id: Id) -> Result<Vec<Chat>> {
        let records = self.chat_repo.select_all_except_casual(user_id)?;

        Ok(records)
    }

    pub fn search_chat_logs(
        &self,
        payload: SearchChatLogPayload,
    ) -> Result<PaginatedRecords<ChatLog>> {
        self.chat_log_repo.select(PageQueryParams {
            user_id: payload.user_id,
            query: ChatLogQueryParams {
                chat_id: payload.chat_id,
            },
            ..Default::default()
        })
    }

    pub fn get_chat_logs_by_cursor(
        &self,
        payload: GetChatLogByCursorPayload,
    ) -> Result<CursorQueryResult<ChatLog>> {
        let chat_id = payload.chat_id;
        let cursor = payload.cursor;
        let size = payload.size;
        let direction = payload.direction;

        let result = self
            .chat_log_repo
            .select_by_cursor(crate::CursorQueryParams {
                cursor,
                direction,
                size,
                query: ChatLogQueryParams { chat_id },
                ..Default::default()
            })?;

        Ok(result)
    }

    pub fn update_chat(&self, payload: UpdateChatPayload) -> Result<()> {
        let patch_chat = PatchChat {
            id: payload.id,
            title: payload.title,
            prompt_id: payload.prompt_id,
            config: payload.config.map(|c| c.into()),
            sort: payload.sort,
            ..Default::default()
        };

        self.chat_repo.update(&patch_chat)?;

        Ok(())
    }

    pub fn remove_prompt(&self, chat_id: Id) -> Result<()> {
        self.chat_repo.remove_prompt(chat_id)?;

        Ok(())
    }

    pub fn set_chat_archive(&self, chat_id: Id) -> Result<()> {
        self.chat_repo.update(&PatchChat {
            id: chat_id,
            archive: Some(true),
            archived_at: Some(Utc::now().naive_local()),
            ..Default::default()
        })?;

        Ok(())
    }

    pub fn set_chat_stick(&self, user_id: Id, chat_id: Id, stick: bool) -> Result<()> {
        if stick {
            let order = self.chat_repo.select_stick_min_order(user_id)?;
            self.chat_repo.update(&PatchChat {
                id: chat_id,
                stick: Some(true),
                sort: Some(order - 1),
                ..Default::default()
            })?;
        } else {
            let order = self.chat_repo.select_non_stick_max_order(user_id)?;
            self.chat_repo.update(&PatchChat {
                id: chat_id,
                stick: Some(false),
                sort: Some(order + 1),
                ..Default::default()
            })?;
        }

        Ok(())
    }

    pub fn move_non_stick_chat(&self, payload: MoveChatPayload) -> Result<()> {
        let user_id = payload.user_id;
        let from_chat = self.chat_repo.select_by_id(payload.from)?;
        let to_chat = self.chat_repo.select_by_id(payload.to)?;

        if from_chat.sort > to_chat.sort {
            // update sort from to_chat to from_chat
            self.chat_repo
                .increase_non_stick_order(user_id, to_chat.sort, from_chat.sort)?;
        } else {
            // update sort from from_chat to to_chat
            self.chat_repo
                .decrease_non_stick_order(user_id, from_chat.sort, to_chat.sort)?;
        }

        // reset from_chat's sort to_chat's sort
        self.update_chat(UpdateChatPayload {
            id: from_chat.id,
            sort: Some(to_chat.sort),
            ..Default::default()
        })?;

        Ok(())
    }

    pub fn move_stick_chat(&self, payload: MoveChatPayload) -> Result<()> {
        let user_id = payload.user_id;
        let from_chat = self.chat_repo.select_by_id(payload.from)?;
        let to_chat = self.chat_repo.select_by_id(payload.to)?;

        if from_chat.sort > to_chat.sort {
            // update sort from to_chat to from_chat
            self.chat_repo
                .increase_stick_order(user_id, to_chat.sort, from_chat.sort)?;
        } else {
            // update sort from from_chat to to_chat
            self.chat_repo
                .decrease_stick_order(user_id, from_chat.sort, to_chat.sort)?;
        }

        // reset from_chat's sort to_chat's sort
        self.update_chat(UpdateChatPayload {
            id: from_chat.id,
            sort: Some(to_chat.sort),
            ..Default::default()
        })?;

        Ok(())
    }

    pub fn delete_chat(&self, payload: DeleteChatPayload) -> Result<()> {
        self.chat_repo.delete_by_id(payload.id)?;
        self.chat_log_repo.delete_by_chat_id(payload.id)?;

        Ok(())
    }

    pub fn update_chat_log(&self, payload: UpdateChatLogPayload) -> Result<()> {
        let patch_chat_log = PatchChatLog {
            id: payload.id,
            message: Some(payload.content),
            ..Default::default()
        };

        self.chat_log_repo.update(&patch_chat_log)?;

        Ok(())
    }

    pub fn delete_chat_log_since_id(&self, id: Id) -> Result<ChatLog> {
        let chat_log = self.chat_log_repo.delete_since_id(id)?;

        Ok(chat_log)
    }

    pub fn delete_chat_log(&self, id: Id) -> Result<()> {
        self.chat_log_repo.delete_by_id(id)?;

        Ok(())
    }

    pub async fn resend_message(
        &self,
        payload: ResendMessagePayload,
        sender: Sender<StreamContent>,
        stop_receiver: Receiver<()>,
    ) -> Result<(Id, Id, JoinHandle<()>)> {
        let message_id = payload.id;
        let chat_log = self.delete_chat_log_since_id(message_id)?;

        self.send_message(
            SendMessagePayload {
                chat_id: chat_log.chat_id,
                message: chat_log.message,
            },
            sender,
            stop_receiver,
        )
        .await
    }

    pub async fn send_message(
        &self,
        payload: SendMessagePayload,
        sender: Sender<StreamContent>,
        mut stop_receiver: Receiver<()>,
    ) -> Result<(Id, Id, JoinHandle<()>)> {
        let SendMessagePayload { chat_id, message } = payload;

        let Chat {
            user_id,
            prompt_id,
            config,
            ..
        } = self.chat_repo.select_by_id(chat_id)?;

        let config = config.0;
        let params = config.params;
        let backtrack = config.backtrack;
        let model = params.model;

        let chat_model = self.chat_model_repo.select_by_name(&model)?;

        let mut messages: Vec<OpenAIChatMessage> = vec![];

        // Add prompt to messages
        if let Some(prompt_id) = prompt_id {
            let prompt = self.prompt_repo.select_by_id(prompt_id)?;
            messages.push(OpenAIChatMessage {
                role: OpenAIChatRole::User,
                content: prompt.content,
            })
        }

        // Add previous logs to messages
        let logs = self
            .chat_log_repo
            .select_last_n(backtrack as i64, payload.chat_id)?;
        for log in logs {
            messages.push(OpenAIChatMessage {
                role: log.role.0.into(),
                content: log.message,
            })
        }

        // Add user message to messages
        let user_message = OpenAIChatMessage {
            role: OpenAIChatRole::User,
            content: message.clone(),
        };
        let user_token = user_message.tokens();
        messages.push(user_message);

        // Add user log to database
        let user_log_id = Id::random();
        let user_log = NewChatLog {
            id: user_log_id,
            chat_id,
            role: Role::User.into(),
            message,
            model: model.clone(),
            tokens: user_token as i32,
            cost: chat_model.calc_cost(user_token),
            finished: false,
        };
        self.chat_log_repo.insert(&user_log)?;

        // Create OpenAI API
        let setting = self.setting_repo.select_by_user_id(user_id)?;
        let api = setting.create_openai_chat();
        let api_params = OpenAIChatParams {
            stream: true,
            model: model.clone(),
            messages,
            frequency_penalty: params.frequency_penalty,
            presence_penalty: params.presence_penalty,
            temperature: params.temperature,
            ..Default::default()
        };
        let total_tokens = api_params.calc_tokens();
        let question_cost = chat_model.calc_cost(total_tokens);

        let chat_repo = self.chat_repo.clone();
        let chat_log_repo = self.chat_log_repo.clone();
        let mut reply = Some(String::new());
        let reply_log_id = Id::random();

        let send = |sender: Sender<StreamContent>, content: StreamContent| async move {
            sender.send(content).await.expect("send message failed");
        };

        let handle = tokio::spawn(async move {
            let save_reply = |reply_message: &str, finished: bool| {
                let reply_tokens =
                    OpenAIChatMessage::calc_tokens(&OpenAIChatRole::Assistant, reply_message);
                let reply_cost = chat_model.calc_cost(reply_tokens);
                let total_cost = question_cost + reply_cost;
                let reply_log = NewChatLog {
                    id: reply_log_id,
                    chat_id,
                    role: Role::Assistant.into(),
                    message: reply_message.to_string(),
                    model: model.clone(),
                    tokens: reply_tokens as i32,
                    cost: total_cost,
                    finished,
                };

                // Add reply log to database
                chat_log_repo.insert(&reply_log).unwrap();

                // Update chat cost
                chat_repo.add_cost_and_update(chat_id, total_cost).unwrap();

                // Mark user log as finished
                chat_log_repo
                    .update(&PatchChatLog {
                        id: user_log_id,
                        finished: Some(true),
                        ..Default::default()
                    })
                    .unwrap();
            };
            let stream = api.send_message(api_params).await;
            match stream {
                Ok(mut stream) => {
                    while let Some(content) = stream.next().await {
                        match &content {
                            StreamContent::Data(data) => match &mut reply {
                                Some(reply) => reply.push_str(data),
                                None => unreachable!(),
                            },
                            StreamContent::Done => {
                                save_reply(reply.as_deref().unwrap_or_default(), true);
                            }
                            _ => {}
                        }
                        send(sender.clone(), content).await;

                        if stop_receiver.try_recv().is_ok() {
                            save_reply(reply.as_deref().unwrap_or_default(), false);
                            break;
                        }
                    }
                    drop(stream);
                }
                Err(err) => send(sender.clone(), StreamContent::Error(err.into())).await,
            }
        });

        Ok((user_log_id, reply_log_id, handle))
    }

    pub fn get_chat_models(&self) -> Result<Vec<ChatModel>> {
        self.chat_model_repo.select()
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateChatPayload {
    pub user_id: Id,
    pub title: String,
    pub prompt_id: Option<Id>,
    pub config: ChatConfig,
    pub vendor: String,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchChatPayload {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub user_id: Option<Id>,
    pub title: Option<String>,
    pub prompt_id: Option<Id>,
}

impl<T: Default, U: Default> From<SearchChatPayload> for PageQueryParams<T, U> {
    fn from(value: SearchChatPayload) -> Self {
        let mut params = PageQueryParams::default();

        if let Some(page) = value.page {
            params.page = page;
        }
        if let Some(per_page) = value.per_page {
            params.per_page = per_page;
        }

        params
    }
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchChatLogPayload {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub chat_id: Option<Id>,
    pub user_id: Id,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetChatLogByCursorPayload {
    pub cursor: Option<Id>,
    pub direction: CursorDirection,
    pub chat_id: Option<Id>,
    pub size: i64,
    pub user_id: Id,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChatPayload {
    pub id: Id,
    pub title: Option<String>,
    pub prompt_id: Option<Id>,
    pub config: Option<ChatConfig>,
    pub sort: Option<i32>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveChatPayload {
    pub user_id: Id,
    pub from: Id,
    pub to: Id,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteChatPayload {
    pub id: Id,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessagePayload {
    pub chat_id: Id,
    pub message: String,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChatLogPayload {
    pub id: Id,
    pub content: String,
}

pub struct ResendMessagePayload {
    pub id: Id,
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc::channel;
    use tokio::sync::oneshot;

    use crate::{
        models::chat::ChatConfig,
        result::Result,
        services::chat::{ChatService, CreateChatPayload, SendMessagePayload},
        test::establish_connection,
        types::{Id, StreamContent},
    };

    #[tokio::test]
    async fn test_send_message() -> Result<()> {
        let conn = establish_connection();
        let chat_service = ChatService::new(conn);
        let user_id = Id::local();

        let chat_id = chat_service.create_chat(CreateChatPayload {
            title: "test".to_string(),
            prompt_id: None,
            vendor: "openai".to_string(),
            user_id,
            config: ChatConfig::default(),
        })?;

        let (sender, mut receiver) = channel::<StreamContent>(20);
        let (_stop_sender, stop_receiver) = oneshot::channel::<()>();

        let (user_log_id, reply_log_id, handle) = chat_service
            .send_message(
                SendMessagePayload {
                    chat_id,
                    message: "reply Hi! to me, no more other words".to_string(),
                },
                sender,
                stop_receiver,
            )
            .await
            .unwrap();

        let mut reply = String::new();
        while let Some(content) = receiver.recv().await {
            match content {
                StreamContent::Data(data) => reply.push_str(&data),
                StreamContent::Done => {
                    assert_eq!(reply, "Hi!");
                }
                _ => {}
            }
        }

        handle.await.unwrap();

        chat_service.delete_chat_log(user_log_id)?;
        chat_service.delete_chat_log(reply_log_id)?;

        Ok(())
    }
}
