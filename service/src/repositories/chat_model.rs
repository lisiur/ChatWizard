use crate::models::chat_model::{NewChatModel, PatchChatModel};
use crate::result::Result;
use crate::schema::chat_models;
use crate::{database::DbConn, models::chat_model::ChatModel, types::Id};
use diesel::*;

#[derive(Clone)]
pub struct ChatModelRepo(DbConn);

impl ChatModelRepo {
    pub fn new(conn: DbConn) -> Self {
        Self(conn)
    }

    pub fn select(&self) -> Result<Vec<ChatModel>> {
        let chat_models = chat_models::table.load::<ChatModel>(&mut *self.0.conn())?;

        Ok(chat_models)
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

    pub fn insert(&self, chat_model: &NewChatModel) -> Result<usize> {
        let size = diesel::insert_into(chat_models::table)
            .values(chat_model)
            .execute(&mut *self.0.conn())?;
        Ok(size)
    }

    pub fn update(&self, chat_model: &PatchChatModel) -> Result<()> {
        diesel::update(chat_models::table)
            .filter(chat_models::id.eq(chat_model.id))
            .set(chat_model)
            .execute(&mut *self.0.conn())?;

        Ok(())
    }

    pub fn delete(&self, id: Id) -> Result<()> {
        diesel::delete(chat_models::table.filter(chat_models::id.eq(id)))
            .execute(&mut *self.0.conn())?;

        Ok(())
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
