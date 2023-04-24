use diesel::query_builder::AsQuery;
use diesel::QueryDsl;

use crate::models::plugin::{InstalledPlugin, NewPlugin, PatchPlugin};
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

    pub fn select_by_name(&self, name: &str) -> Result<Plugin> {
        plugins::table
            .as_query()
            .filter(plugins::name.eq(name))
            .first::<Plugin>(&mut *self.0.conn())
            .map_err(Into::into)
    }

    pub fn select_all(&self) -> Result<Vec<InstalledPlugin>> {
        plugins::table
            .as_query()
            .load::<Plugin>(&mut *self.0.conn())
            .map(|plugins| {
                plugins.into_iter().map(|plugin| InstalledPlugin {
                    id: plugin.id,
                    name: plugin.name,
                    version: plugin.version,
                    description: plugin.description,
                    author: plugin.author,
                    config: plugin.config,
                    created_at: plugin.created_at,
                    updated_at: plugin.updated_at,
                })
                .collect()
            })
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
