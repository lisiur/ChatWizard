use chrono::NaiveDateTime;
use diesel::*;

use crate::result::Result;
use crate::schema::chats;
use crate::{conn::DbConn, types::Id};

#[derive(Insertable)]
#[diesel(table_name = chats)]
pub struct NewChat {
    pub id: Id,
    pub user_id: Id,
    pub title: String,
    pub prompt_id: Option<Id>,
    pub config: String,
    pub cost: f32,
}

#[derive(Queryable, Selectable)]
pub struct Chat {
    pub id: Id,
    pub user_id: Id,
    pub title: String,
    pub prompt_id: Option<Id>,
    pub config: String,
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
    pub config: Option<String>,
    pub cost: Option<f32>,
}
