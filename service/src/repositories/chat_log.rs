use crate::database::pagination::{Paginate, PaginatedRecords};
use crate::models::chat_log::{ChatLog, NewChatLog};
use crate::result::Result;
use crate::schema::chat_logs;
use crate::PageQueryParams;
use crate::{database::DbConn, types::Id};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Clone)]
pub struct ChatLogRepo(DbConn);

impl ChatLogRepo {
    pub fn new(conn: DbConn) -> Self {
        Self(conn)
    }

    pub fn select_by_id(&self, id: Id) -> Result<ChatLog> {
        chat_logs::table
            .filter(chat_logs::id.eq(id))
            .first::<ChatLog>(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn select(
        &self,
        params: PageQueryParams<ChatLogQueryParams, ()>,
    ) -> Result<PaginatedRecords<ChatLog>> {
        let mut query = chat_logs::table.into_boxed();

        if let Some(chat_id) = params.query.chat_id {
            query = query.filter(chat_logs::chat_id.eq(chat_id));
        }

        let result = query
            .order(chat_logs::created_at.asc())
            .paginate(params.page)
            .per_page(params.per_page)
            .load_and_count_pages::<ChatLog>(&mut self.0.conn())?;

        Ok(result)
    }

    pub fn delete_since_id(&self, id: Id) -> Result<ChatLog> {
        let target_log = self.select_by_id(id)?;
        let chat_id = target_log.chat_id;
        diesel::delete(chat_logs::table)
            .filter(chat_logs::chat_id.eq(chat_id))
            .filter(chat_logs::created_at.ge(target_log.created_at))
            .execute(&mut *self.0.conn())?;

        Ok(target_log)
    }

    pub fn delete_after_id(&self, id: Id) -> Result<usize> {
        let target_log = self.select_by_id(id)?;
        let chat_id = target_log.chat_id;
        let size = diesel::delete(chat_logs::table)
            .filter(chat_logs::chat_id.eq(chat_id))
            .filter(chat_logs::created_at.gt(target_log.created_at))
            .execute(&mut *self.0.conn())?;

        Ok(size)
    }

    pub fn delete_by_id(&self, id: Id) -> Result<usize> {
        let size = diesel::delete(chat_logs::table)
            .filter(chat_logs::id.eq(id))
            .execute(&mut *self.0.conn())?;

        Ok(size)
    }

    pub fn delete_by_chat_id(&self, chat_id: Id) -> Result<usize> {
        let size = diesel::delete(chat_logs::table)
            .filter(chat_logs::chat_id.eq(chat_id))
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
        let mut records = chat_logs::table
            .filter(chat_logs::chat_id.eq(chat_id))
            .order(chat_logs::created_at.desc())
            .limit(n)
            .load::<ChatLog>(&mut *self.0.conn())?;

        records.reverse();

        Ok(records)
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChatLogQueryParams {
    pub chat_id: Option<Id>,
}
