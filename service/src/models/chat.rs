use chrono::NaiveDateTime;
use diesel::*;

use crate::result::Result;
use crate::schema::chats;
use crate::{conn::DbConn, types::Id};

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

struct ChatRepo(DbConn);

impl ChatRepo {
    pub fn new(conn: DbConn) -> Self {
        Self(conn)
    }

    pub fn select(&self, chat_id: Id) -> Result<Chat> {
        use crate::schema::chats::dsl::*;
        chats
            .filter(id.eq(chat_id))
            .first::<Chat>(&mut *self.0.conn())
            .map_err(|e| e.into())
    }
}
