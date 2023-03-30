use diesel::prelude::*;

use crate::models::prompt_source::NewPromptSource;
use crate::result::Result;
use crate::schema::prompt_sources;
use crate::Id;
use crate::{models::prompt_source::PromptSource, DbConn};

pub struct PromptSourceRepo(DbConn);

impl PromptSourceRepo {
    pub fn new(conn: DbConn) -> Self {
        Self(conn)
    }

    pub fn select(&self) -> Result<Vec<PromptSource>> {
        let prompt_sources = prompt_sources::table.load::<PromptSource>(&mut *self.0.conn())?;

        Ok(prompt_sources)
    }

    pub fn select_by_id(&self, id: Id) -> Result<PromptSource> {
        prompt_sources::table
            .filter(prompt_sources::id.eq(id))
            .first::<PromptSource>(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn insert_or_update(&self, prompt_source: &NewPromptSource) -> Result<usize> {
        use crate::schema::prompt_sources::dsl::*;

        let size = diesel::insert_into(prompt_sources)
            .values(prompt_source)
            .on_conflict(id)
            .do_update()
            .set(prompt_source)
            .execute(&mut *self.0.conn())?;
        Ok(size)
    }
}
