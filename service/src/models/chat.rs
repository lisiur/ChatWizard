use std::backtrace;

use chrono::NaiveDateTime;
use diesel::deserialize::FromSql;
use diesel::serialize::ToSql;
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use diesel::{prelude::*, AsExpression, FromSqlRow};

use crate::result::Result;
use crate::schema::chats;
use crate::types::JsonWrapper;
use crate::{database::DbConn, types::Id};

#[derive(Insertable)]
#[diesel(table_name = chats)]
pub struct NewChat {
    pub id: Id,
    pub user_id: Id,
    pub title: String,
    pub prompt_id: Option<Id>,
    pub config: JsonWrapper<ChatConfig>,
    pub cost: f32,
}

#[derive(Queryable, Selectable, Identifiable)]
pub struct Chat {
    pub id: Id,
    pub user_id: Id,
    pub title: String,
    pub prompt_id: Option<Id>,
    pub config: JsonWrapper<ChatConfig>,
    pub cost: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = chats)]
pub struct PatchChat {
    pub id: Id,
    pub title: Option<String>,
    pub prompt_id: Option<Id>,
    pub config: Option<JsonWrapper<ChatConfig>>,
    pub cost: Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ChatConfig {
    backtrack: usize,
    params: ChatParams,
}

impl Default for ChatConfig {
    fn default() -> Self {
        Self {
            backtrack: 3,
            params: ChatParams::OpenAI(OpenAIChatParams {
                model: "gpt-3.5-turbo".to_string(),
                temperature: None,
                stop: None,
                presence_penalty: None,
                frequency_penalty: None,
            }),
        }
    }
}

impl From<ChatConfig> for JsonWrapper<ChatConfig> {
    fn from(val: ChatConfig) -> Self {
        JsonWrapper(val)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ChatParams {
    #[serde(rename = "openai")]
    OpenAI(OpenAIChatParams),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenAIChatParams {
    pub model: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
}
