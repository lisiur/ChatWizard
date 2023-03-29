use crate::models::chat::NewChat;
use crate::repositories::chat::ChatRepo;
use crate::repositories::user::UserRepo;
use crate::result::Result;
use crate::{database::DbConn, models::chat::ChatConfig, types::Id};

pub struct OpenaiChatService {
    conn: DbConn,
    chat_repo: ChatRepo,
    user_repo: UserRepo,
}

impl OpenaiChatService {
    pub fn new(conn: DbConn) -> Self {
        Self {
            chat_repo: ChatRepo::new(conn.clone()),
            user_repo: UserRepo::new(conn.clone()),
            conn,
        }
    }

    pub async fn create_chat(&self, payload: CreateChatPayload) -> Result<Id> {
        let chat_id = Id::random();

        let new_chat = NewChat {
            id: chat_id,
            user_id: payload.user_id,
            title: payload.title,
            prompt_id: payload.prompt_id,
            config: payload.config.into(),
            cost: 0.0,
        };

        self.chat_repo.insert(&new_chat)?;

        Ok(chat_id)
    }

    pub async fn search_chat(&self, payload: SearchChatPayload) {}

    pub async fn update_chat(&self, payload: UpdateChatPayload) {}

    pub async fn delete_chat(&self, payload: DeleteChatPayload) {}

    pub async fn send_message(&self, payload: SendMessagePayload) {}
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateChatPayload {
    pub user_id: Id,
    pub title: String,
    pub prompt_id: Option<Id>,
    pub config: ChatConfig,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchChatPayload {
    pub user_id: Option<Id>,
    pub title: Option<String>,
    pub prompt_id: Option<Id>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChatPayload {
    pub title: Option<String>,
    pub prompt_id: Option<Id>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteChatPayload {
    pub id: Option<Id>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessagePayload {
    pub chat_id: Id,
    pub user_id: Id,
    pub message: String,
}
