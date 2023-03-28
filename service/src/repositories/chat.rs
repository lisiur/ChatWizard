use crate::models::chat::NewChat;
use crate::result::Result;
use crate::{conn::DbConn, models::chat::Chat, types::Id};
use diesel::*;

struct ChatRepo(DbConn);

impl ChatRepo {
    pub fn new(conn: DbConn) -> Self {
        Self(conn)
    }

    pub fn count(&self) -> Result<i64> {
        use crate::schema::chats::dsl::*;

        chats
            .count()
            .get_result(&mut *self.0.conn())
            .map_err(Into::into)
    }

    pub fn select_by_id(&self, chat_id: Id) -> Result<Chat> {
        use crate::schema::chats::dsl::*;

        chats
            .filter(id.eq(chat_id))
            .first::<Chat>(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn select_by_user_id(&self, user_id: Id) -> Result<Vec<Chat>> {
        use crate::schema::chats::dsl::*;

        chats
            .filter(user_id.eq(user_id))
            .load::<Chat>(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn insert(&self, chat: &NewChat) -> Result<usize> {
        use crate::schema::chats::dsl::*;

        let size = diesel::insert_into(chats)
            .values(chat)
            .execute(&mut *self.0.conn())?;
        Ok(size)
    }
}
