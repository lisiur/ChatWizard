use futures::StreamExt;
use tokio::sync::mpsc::Sender;

use crate::api::openai::chat::params::{OpenAIChatMessage, OpenAIChatParams, OpenAIChatRole};
use crate::database::pagination::PaginatedRecords;
use crate::models::chat::{Chat, NewChat, PatchChat};
use crate::models::chat_log::{NewChatLog, Role};
use crate::repositories::chat::ChatRepo;
use crate::repositories::chat_log::ChatLogRepo;
use crate::repositories::chat_model::ChatModelRepo;
use crate::repositories::setting::SettingRepo;
use crate::result::Result;
use crate::types::{PageQueryParams, StreamContent};
use crate::{database::DbConn, models::chat::ChatConfig, types::Id};

pub struct ChatService {
    #[allow(unused)]
    conn: DbConn,
    chat_repo: ChatRepo,
    chat_log_repo: ChatLogRepo,
    setting_repo: SettingRepo,
    chat_model_repo: ChatModelRepo,
}

impl ChatService {
    pub fn new(conn: DbConn) -> Self {
        Self {
            chat_repo: ChatRepo::new(conn.clone()),
            chat_log_repo: ChatLogRepo::new(conn.clone()),
            chat_model_repo: ChatModelRepo::new(conn.clone()),
            setting_repo: SettingRepo::new(conn.clone()),
            conn,
        }
    }

    pub fn create_chat(&self, payload: CreateChatPayload) -> Result<Id> {
        let chat_id = Id::random();

        let new_chat = NewChat {
            id: chat_id,
            user_id: payload.user_id,
            title: payload.title,
            prompt_id: payload.prompt_id,
            config: payload.config.into(),
            cost: 0.0,
            vendor: payload.vendor,
        };

        self.chat_repo.insert(&new_chat)?;

        Ok(chat_id)
    }

    pub fn search_chat(&self, payload: SearchChatPayload) -> Result<PaginatedRecords<Chat>> {
        let mut params = PageQueryParams::default();

        if let Some(page) = payload.page {
            params.page = page;
        }
        if let Some(per_page) = payload.per_page {
            params.per_page = per_page;
        }

        let records = self.chat_repo.select(params)?;

        Ok(records)
    }

    pub fn update_chat(&self, payload: UpdateChatPayload) -> Result<()> {
        let patch_chat = PatchChat {
            id: payload.id,
            title: payload.title,
            prompt_id: payload.prompt_id,
            ..Default::default()
        };

        self.chat_repo.update(&patch_chat)?;

        Ok(())
    }

    pub fn delete_chat(&self, payload: DeleteChatPayload) -> Result<()> {
        self.chat_repo.delete_by_id(payload.id)?;

        Ok(())
    }

    pub async fn send_message(
        &self,
        payload: SendMessagePayload,
        sender: Sender<StreamContent>,
    ) -> Result<()> {
        let setting = self.setting_repo.select_by_user_id(payload.user_id)?;
        let Chat { config, .. } = self.chat_repo.select_by_id(payload.chat_id)?;

        let config = config.0;
        let params = config.params;
        let backtrack = config.backtrack;
        let chat_id = payload.chat_id;
        let model = params.model;

        let chat_model = self.chat_model_repo.select_by_name(&model)?;

        let user_message = OpenAIChatMessage {
            role: OpenAIChatRole::User,
            content: payload.message.clone(),
        };
        let user_token = user_message.tokens();

        let user_log = NewChatLog {
            id: Id::random(),
            chat_id,
            role: Role::User.into(),
            message: payload.message,
            model: model.clone(),
            tokens: user_token as i32,
            cost: chat_model.calc_cost(user_token),
        };

        self.chat_log_repo.insert(&user_log)?;

        let logs = self
            .chat_log_repo
            .select_last_n(backtrack as i64, payload.chat_id)?;

        let mut messages: Vec<OpenAIChatMessage> = logs
            .into_iter()
            .map(|log| OpenAIChatMessage {
                role: log.role.0.into(),
                content: log.message,
            })
            .collect();

        messages.push(user_message);

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

        let api = setting.create_openai_chat();
        let mut stream = api.send_message(api_params).await?;

        let chat_log_repo = self.chat_log_repo.clone();
        let chat_id = payload.chat_id;
        let mut reply = Some(String::new());
        while let Some(content) = stream.next().await {
            match &content {
                StreamContent::Data(data) => match &mut reply {
                    Some(reply) => reply.push_str(data),
                    None => unreachable!(),
                },
                StreamContent::Done => {
                    let reply_tokens = OpenAIChatMessage::calc_tokens(
                        &OpenAIChatRole::Assistant,
                        reply.as_deref().unwrap_or_default(),
                    );
                    let reply_cost = chat_model.calc_cost(reply_tokens);
                    let total_cost = question_cost + reply_cost;
                    let reply_log = NewChatLog {
                        id: Id::random(),
                        chat_id,
                        role: Role::Assistant.into(),
                        message: reply.take().unwrap(),
                        model: model.clone(),
                        tokens: reply_tokens as i32,
                        cost: total_cost,
                    };
                    chat_log_repo.insert(&reply_log).unwrap();
                }
                _ => {}
            }
            sender.send(content).await.expect("send message");
        }

        Ok(())
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

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchChatPayload {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub user_id: Option<Id>,
    pub title: Option<String>,
    pub prompt_id: Option<Id>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChatPayload {
    pub id: Id,
    pub title: Option<String>,
    pub prompt_id: Option<Id>,
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
    pub user_id: Id,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc::channel;

    use crate::{
        models::{chat::ChatConfig, setting::PatchSetting},
        repositories::setting::SettingRepo,
        result::Result,
        services::chat::{ChatService, CreateChatPayload, SendMessagePayload},
        test::establish_connection,
        types::{Id, StreamContent},
    };

    #[tokio::test]
    async fn test_send_message() -> Result<()> {
        let conn = establish_connection();
        let setting_repo = SettingRepo::new(conn.clone());
        let chat_service = ChatService::new(conn);
        let user_id = Id::local();

        dotenvy::dotenv().unwrap();
        let api_key = std::env::var("API_KEY").unwrap();
        let proxy = std::env::var("PROXY").unwrap();

        setting_repo.update(&PatchSetting {
            api_key: Some(api_key),
            proxy: Some(proxy),
            ..Default::default()
        })?;

        let chat_id = chat_service.create_chat(CreateChatPayload {
            title: "test".to_string(),
            prompt_id: None,
            vendor: "openai".to_string(),
            user_id,
            config: ChatConfig::default(),
        })?;

        let (sender, mut receiver) = channel::<StreamContent>(20);

        let handle = tokio::spawn(async move {
            chat_service
                .send_message(
                    SendMessagePayload {
                        chat_id,
                        user_id,
                        message: "hello".to_string(),
                    },
                    sender,
                )
                .await
                .unwrap();
        });

        while let Some(content) = receiver.recv().await {
            println!("{:?}", content);
        }
        handle.await.unwrap();

        Ok(())
    }
}
