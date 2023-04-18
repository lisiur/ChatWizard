use diesel::query_builder::AsQuery;
use diesel::QueryDsl;

use crate::models::plugin::{NewPlugin, PatchPlugin};
use crate::result::Result;
use crate::schema::plugins;
use crate::Id;
use crate::{models::plugin::Plugin, DbConn};
use diesel::prelude::*;

#[derive(Clone)]
pub struct PluginRepo(DbConn);

impl PluginRepo {
    pub fn new(conn: DbConn) -> Self {
        Self(conn)
    }

    pub fn select_by_id(&self, id: Id) -> Result<Plugin> {
        plugins::table
            .as_query()
            .filter(plugins::id.eq(id))
            .first::<Plugin>(&mut *self.0.conn())
            .map_err(Into::into)
    }

    pub fn select_all(&self) -> Result<Vec<Plugin>> {
        plugins::table
            .as_query()
            .load::<Plugin>(&mut *self.0.conn())
            .map_err(Into::into)
    }

    pub fn insert(&self, plugin: NewPlugin) -> Result<usize> {
        let size = diesel::insert_into(plugins::table)
            .values(plugin)
            .execute(&mut *self.0.conn())?;

        Ok(size)
    }

    pub fn update(&self, plugin: PatchPlugin) -> Result<usize> {
        let size = diesel::update(plugins::table)
            .filter(plugins::id.eq(plugin.id))
            .set(plugin)
            .execute(&mut *self.0.conn())?;

        Ok(size)
    }

    pub fn delete_by_id(&self, id: Id) -> Result<usize> {
        let size = diesel::delete(plugins::table)
            .filter(plugins::id.eq(id))
            .execute(&mut *self.0.conn())?;

        Ok(size)
    }
}
