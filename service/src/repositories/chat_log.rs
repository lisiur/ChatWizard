use crate::database::pagination::{Paginate, PaginatedRecords};
use crate::models::chat_log::{ChatLog, NewChatLog, PatchChatLog};
use crate::result::Result;
use crate::schema::chat_logs;
use crate::{database::DbConn, types::Id};
use crate::{CursorDirection, CursorQueryParams, CursorQueryResult, PageQueryParams};
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

    pub fn select_by_cursor(
        &self,
        params: CursorQueryParams<ChatLogQueryParams, ()>,
    ) -> Result<CursorQueryResult<ChatLog>> {
        let cursor_created_at = params
            .cursor
            .map(|id| {
                chat_logs::table
                    .filter(chat_logs::id.eq(id))
                    .select(chat_logs::created_at)
                    .first::<chrono::NaiveDateTime>(&mut *self.0.conn())
                    .unwrap()
            })
            .unwrap_or_else(|| match params.direction {
                CursorDirection::Forward => chrono::NaiveDateTime::parse_from_str(
                    "2000-01-01 00:00:00",
                    "%Y-%m-%d %H:%M:%S",
                )
                .unwrap(),
                CursorDirection::Backward => chrono::NaiveDateTime::parse_from_str(
                    "2999-12-31 23:59:59",
                    "%Y-%m-%d %H:%M:%S",
                )
                .unwrap(),
            });

        let mut query = chat_logs::table.into_boxed();

        if let Some(chat_id) = params.query.chat_id {
            query = query.filter(chat_logs::chat_id.eq(chat_id));
        }

        match params.direction {
            CursorDirection::Forward => {
                query = query
                    .filter(chat_logs::created_at.ge(cursor_created_at))
                    .order(chat_logs::created_at.asc());
            }
            CursorDirection::Backward => {
                query = query
                    .filter(chat_logs::created_at.le(cursor_created_at))
                    .order(chat_logs::created_at.desc());
            }
        };

        let mut items = query
            .limit(params.size + 1)
            .load::<ChatLog>(&mut *self.0.conn())?;

        // let mut items = match params.direction {
        //     CursorDirection::Forward => items,
        //     CursorDirection::Backward => {
        //         items.reverse();
        //         items
        //     }
        // };

        let next_cursor = if items.len() > params.size as usize {
            let next_cursor = items.last().unwrap().id;
            items.pop();
            Some(next_cursor)
        } else {
            None
        };

        let result = CursorQueryResult {
            records: items,
            next_cursor,
        };

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

    pub fn update(&self, chat_log: &PatchChatLog) -> Result<()> {
        diesel::update(chat_logs::table)
            .filter(chat_logs::id.eq(chat_log.id))
            .set(chat_log)
            .execute(&mut *self.0.conn())?;

        Ok(())
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
