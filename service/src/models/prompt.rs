use chrono::NaiveDateTime;
use diesel::*;
use serde::Serialize;

use crate::schema::prompts;
use crate::types::Id;

#[derive(Queryable, Serialize)]
pub struct Prompt {
    pub id: Id,
    pub name: String,
    pub content: String,
    pub user_id: Id,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize)]
#[diesel(table_name = prompts)]
pub struct PromptIndex {
    pub id: Id,
    pub name: String,
    #[serde(skip_serializing)]
    pub content: String,
    pub user_id: Id,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = prompts)]
pub struct NewPrompt {
    pub id: Id,
    pub name: String,
    pub content: String,
    pub user_id: Id,
}

#[derive(AsChangeset, Default, Debug)]
#[diesel(table_name = prompts)]
pub struct PatchPrompt {
    pub id: Id,
    pub name: Option<String>,
    pub content: Option<String>,
}
