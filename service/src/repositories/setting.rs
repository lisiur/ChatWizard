use crate::models::setting::NewSetting;
use crate::result::Result;
use crate::{database::DbConn, models::setting::Setting, types::Id};
use diesel::prelude::*;

pub struct SettingRepo(DbConn);

impl SettingRepo {
    pub fn new(conn: DbConn) -> Self {
        Self(conn)
    }

    pub fn insert(&self, setting: &NewSetting) -> Result<usize> {
        use crate::schema::settings;

        let size = diesel::insert_into(settings::table)
            .values(setting)
            .execute(&mut *self.0.conn())?;
        Ok(size)
    }

    pub fn insert_if_not_exist(&self, setting: &NewSetting) -> Result<usize> {
        use crate::schema::settings;

        let size = diesel::insert_into(settings::table)
            .values(setting)
            .on_conflict(settings::columns::id)
            .do_nothing()
            .execute(&mut *self.0.conn())?;
        Ok(size)
    }

    pub fn select_by_user_id(&self, user_id: Id) -> Result<Setting> {
        use crate::schema::settings;

        settings::table
            .filter(settings::columns::user_id.eq(user_id))
            .first::<Setting>(&mut *self.0.conn())
            .map_err(|e| e.into())
    }
}
