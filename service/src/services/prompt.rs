use crate::database::pagination::PaginatedRecords;
use crate::models::prompt::{NewPrompt, PatchPrompt, Prompt, PromptIndex};
use crate::result::Result;
use crate::types::{Id, PageQueryParams};
use crate::{
    database::DbConn,
    repositories::{chat::ChatRepo, prompt::PromptRepo},
};

pub struct PromptService {
    #[allow(unused)]
    conn: DbConn,
    chat_repo: ChatRepo,
    prompt_repo: PromptRepo,
}

impl PromptService {
    pub fn new(conn: DbConn) -> Self {
        Self {
            chat_repo: ChatRepo::new(conn.clone()),
            prompt_repo: PromptRepo::new(conn.clone()),
            conn,
        }
    }

    pub fn search_prompts(
        &self,
        payload: SearchPromptPayload,
    ) -> Result<PaginatedRecords<PromptIndex>> {
        let records = self.prompt_repo.select_index(payload.into())?;
        Ok(records)
    }

    pub fn get_prompt(&self, prompt_id: Id) -> Result<Prompt> {
        log::debug!("get prompt: {:?}", prompt_id);
        let prompt = self.prompt_repo.select_by_id(prompt_id)?;
        Ok(prompt)
    }

    pub fn create_prompt(&self, payload: CreatePromptPayload) -> Result<Id> {
        let id = Id::random();
        let prompt = NewPrompt {
            id,
            name: payload.name,
            content: payload.content,
            user_id: payload.user_id,
        };
        self.prompt_repo.insert(&prompt)?;

        Ok(id)
    }

    pub fn update_prompt(&self, payload: UpdatePromptPayload) -> Result<()> {
        let prompt = PatchPrompt {
            id: payload.id,
            name: payload.name,
            content: payload.content,
        };

        self.prompt_repo.update(&prompt)?;

        Ok(())
    }

    pub fn delete_prompt(&self, prompt_id: Id) -> Result<()> {
        self.prompt_repo.delete_by_id(prompt_id)?;
        self.chat_repo.update_deleted_prompt(prompt_id)?;

        Ok(())
    }
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchPromptPayload {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub user_id: Option<Id>,
    pub title: Option<String>,
    pub prompt_id: Option<Id>,
}

impl<T: Default, U: Default> From<SearchPromptPayload> for PageQueryParams<T, U> {
    fn from(value: SearchPromptPayload) -> Self {
        let mut params = PageQueryParams::<T, U>::default();

        if let Some(page) = value.page {
            params.page = page;
        }
        if let Some(per_page) = value.per_page {
            params.per_page = per_page;
        }

        params
    }
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreatePromptPayload {
    pub name: String,
    pub content: String,
    pub user_id: Id,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePromptPayload {
    pub id: Id,
    pub name: Option<String>,
    pub content: Option<String>,
}
