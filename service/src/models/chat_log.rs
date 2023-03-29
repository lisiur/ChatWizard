use std::str::FromStr;

use chrono::NaiveDateTime;
use diesel::deserialize::FromSql;
use diesel::serialize::ToSql;
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use diesel::*;

use crate::schema::chat_logs;
use crate::types::{Id, TextWrapper};

#[derive(Queryable)]
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
}

#[derive(AsExpression, Hash, PartialEq, Eq, Clone, Debug)]
#[diesel(sql_type = Text)]
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

#[derive(AsChangeset)]
#[diesel(table_name = chat_logs)]
pub struct PatchChatLog {
    pub id: Id,
    pub chat_id: Option<Id>,
    pub role: Option<Role>,
    pub message: Option<String>,
    pub model: Option<String>,
    pub tokens: Option<i32>,
    pub cost: Option<f32>,
}
