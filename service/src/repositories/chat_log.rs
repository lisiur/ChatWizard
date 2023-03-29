use crate::models::chat_log::{ChatLog, NewChatLog};
use crate::result::Result;
use crate::schema::chat_logs;
use crate::{database::DbConn, types::Id};
use diesel::prelude::*;

#[derive(Clone)]
pub struct ChatLogRepo(DbConn);

impl ChatLogRepo {
    pub fn new(conn: DbConn) -> Self {
        Self(conn)
    }

    pub fn delete_by_id(&self, id: Id) -> Result<usize> {
        let size = diesel::delete(chat_logs::table)
            .filter(chat_logs::id.eq(id))
            .execute(&mut *self.0.conn())?;

        Ok(size)
    }

    pub fn insert(&self, chat_log: &NewChatLog) -> Result<usize> {
        let size = diesel::insert_into(chat_logs::table)
            .values(chat_log)
            .execute(&mut *self.0.conn())?;

        Ok(size)
    }

    pub fn select_last_n(&self, n: i64, chat_id: Id) -> Result<Vec<ChatLog>> {
        let records = chat_logs::table
            .filter(chat_logs::chat_id.eq(chat_id))
            .order(chat_logs::created_at.desc())
            .limit(n)
            .load::<ChatLog>(&mut *self.0.conn())?;

        Ok(records)
    }
}
