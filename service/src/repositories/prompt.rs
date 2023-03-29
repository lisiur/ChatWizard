use diesel::query_builder::AsQuery;

use crate::database::pagination::*;
use crate::database::DbConn;
use crate::models::prompt::Prompt;
use crate::result::Result;
use crate::schema::prompts;

pub struct PromptRepo(DbConn);

impl PromptRepo {
    pub fn select(&self, page: i64, per_page: i64) -> Result<PaginatedRecords<Prompt>> {
        let records = prompts::table
            .as_query()
            .paginate(page)
            .per_page(per_page)
            .load_and_count_pages::<Prompt>(&mut self.0.conn())?;

        Ok(records)
    }
}
