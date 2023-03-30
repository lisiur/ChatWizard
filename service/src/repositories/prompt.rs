use diesel::query_builder::AsQuery;
use diesel::*;

use crate::database::pagination::*;
use crate::database::DbConn;
use crate::models::prompt::NewPrompt;
use crate::models::prompt::PatchPrompt;
use crate::models::prompt::Prompt;
use crate::models::prompt::PromptIndex;
use crate::result::Result;
use crate::schema::prompts;
use crate::types::Id;
use crate::types::PageQueryParams;

#[derive(Clone)]
pub struct PromptRepo(DbConn);

impl PromptRepo {
    pub fn new(conn: DbConn) -> Self {
        Self(conn)
    }

    pub fn select_index(
        &self,
        params: PageQueryParams<(), ()>,
    ) -> Result<PaginatedRecords<PromptIndex>> {
        let records = prompts::table
            .as_query()
            .filter(prompts::user_id.eq(params.user_id))
            .paginate(params.page)
            .per_page(params.per_page)
            .load_and_count_pages::<PromptIndex>(&mut self.0.conn())?;

        Ok(records)
    }

    pub fn select_by_id(&self, prompt_id: Id) -> Result<Prompt> {
        log::debug!("select prompt by id: {:?}", prompt_id);
        prompts::table
            .filter(prompts::id.eq(prompt_id))
            .first::<Prompt>(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn select_by_user_id(&self, user_id: Id) -> Result<Vec<Prompt>> {
        prompts::table
            .filter(prompts::user_id.eq(user_id))
            .load::<Prompt>(&mut *self.0.conn())
            .map_err(|e| e.into())
    }

    pub fn insert(&self, prompt: &NewPrompt) -> Result<usize> {
        let size = diesel::insert_into(prompts::table)
            .values(prompt)
            .execute(&mut *self.0.conn())?;
        Ok(size)
    }

    pub fn update(&self, prompt: &PatchPrompt) -> Result<usize> {
        log::debug!("update prompt: {:?}", prompt);

        let size = diesel::update(prompts::table)
            .filter(prompts::id.eq(prompt.id))
            .set(prompt)
            .execute(&mut *self.0.conn())?;
        Ok(size)
    }

    pub fn delete_by_id(&self, prompt_id: Id) -> Result<usize> {
        let size = diesel::delete(prompts::table)
            .filter(prompts::id.eq(prompt_id))
            .execute(&mut *self.0.conn())?;
        Ok(size)
    }
}
