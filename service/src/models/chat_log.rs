use std::str::FromStr;

use chrono::NaiveDateTime;
use diesel::*;
use serde::Serialize;

use crate::api::openai::chat::params::OpenAIChatRole;
use crate::schema::chat_logs;
use crate::types::{Id, TextWrapper};

#[derive(Queryable, Serialize)]
pub struct ChatLog {
    pub id: Id,
    pub chat_id: Id,
    pub role: TextWrapper<Role>,
    pub message: String,
    pub model: String,
    pub tokens: i32,
    pub cost: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub finished: bool,
}

#[derive(Hash, PartialEq, Eq, Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    System,
    Assistant,
    User,
}

impl AsRef<str> for Role {
    fn as_ref(&self) -> &str {
        match self {
            Role::System => "system",
            Role::Assistant => "assistant",
            Role::User => "user",
        }
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "system" => Ok(Role::System),
            "assistant" => Ok(Role::Assistant),
            "user" => Ok(Role::User),
            _ => Err("Invalid role".into()),
        }
    }
}

impl From<Role> for OpenAIChatRole {
    fn from(role: Role) -> Self {
        match role {
            Role::System => OpenAIChatRole::System,
            Role::Assistant => OpenAIChatRole::Assistant,
            Role::User => OpenAIChatRole::User,
        }
    }
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = chat_logs)]
pub struct PatchChatLog {
    pub id: Id,
    pub chat_id: Option<Id>,
    pub role: Option<TextWrapper<Role>>,
    pub message: Option<String>,
    pub model: Option<String>,
    pub tokens: Option<i32>,
    pub cost: Option<f32>,
    pub finished: Option<bool>,
}

#[derive(Insertable)]
#[diesel(table_name = chat_logs)]
pub struct NewChatLog {
    pub id: Id,
    pub chat_id: Id,
    pub role: TextWrapper<Role>,
    pub message: String,
    pub model: String,
    pub tokens: i32,
    pub cost: f32,
    pub finished: bool,
}
