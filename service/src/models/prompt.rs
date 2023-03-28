use chrono::NaiveDateTime;
use diesel::*;

use crate::schema::prompts;
use crate::types::Id;

#[derive(Queryable)]
struct Prompt {
    pub id: Id,
    pub name: String,
    pub content: String,
    pub user_id: Id,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = prompts)]
struct NewPrompt {
    pub id: Id,
    pub name: String,
    pub content: String,
    pub user_id: Id,
}

#[derive(AsChangeset)]
#[diesel(table_name = prompts)]
pub struct PatchPrompt {
    pub id: Id,
    pub name: Option<String>,
    pub content: Option<String>,
}
