use chrono::NaiveDateTime;
use diesel::deserialize::FromSql;
use diesel::serialize::ToSql;
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use diesel::*;

use crate::schema::chat_logs;
use crate::types::Id;

#[derive(Queryable)]
pub struct ChatLog {
    pub id: Id,
    pub chat_id: Id,
    pub role: Role,
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

impl FromSql<Text, Sqlite> for Role {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Sqlite>) -> diesel::deserialize::Result<Self> {
        let bytes = <Vec<u8>>::from_sql(bytes)?;
        let string = String::from_utf8(bytes)?;
        match string.as_ref() {
            "system" => Ok(Role::System),
            "assistant" => Ok(Role::Assistant),
            "user" => Ok(Role::User),
            _ => Err("Invalid role".into()),
        }
    }
}

impl ToSql<Text, Sqlite> for Role {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Sqlite>,
    ) -> diesel::serialize::Result {
        <str as ToSql<Text, Sqlite>>::to_sql(self.as_ref(), out)
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
