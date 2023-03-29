use crate::models::chat_model::NewChatModel;
use crate::result::Result;
use crate::schema::chat_models;
use crate::{database::DbConn, models::chat_model::ChatModel, types::Id};
use diesel::*;

pub struct ChatModelRepo(DbConn);

impl ChatModelRepo {
    pub fn new(conn: DbConn) -> Self {
        Self(conn)
    }

    pub fn select_by_id(&self, id: Id) -> Result<ChatModel> {
        chat_models::table
            .filter(chat_models::id.eq(id))
            .first::<ChatModel>(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn select_by_name(&self, name: &str) -> Result<ChatModel> {
        chat_models::table
            .filter(chat_models::name.eq(name))
            .first::<ChatModel>(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn insert_or_update(&self, chat_model: &NewChatModel) -> Result<usize> {
        use crate::schema::chat_models::dsl::*;

        let size = diesel::insert_into(chat_models)
            .values(chat_model)
            .on_conflict(id)
            .do_update()
            .set(chat_model)
            .execute(&mut *self.0.conn())?;
        Ok(size)
    }
}
