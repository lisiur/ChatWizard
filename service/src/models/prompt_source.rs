use crate::schema::prompt_sources;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

use crate::Id;

#[derive(Queryable, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PromptSource {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub url: String,
    pub r#type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = prompt_sources)]
pub struct NewPromptSource {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub url: String,
    #[diesel(column_name = "type_")]
    pub r#type: String,
}
