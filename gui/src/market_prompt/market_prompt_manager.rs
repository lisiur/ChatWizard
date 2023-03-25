use std::collections::HashMap;

use reqwest::Client;
use uuid::Uuid;

use super::{market_prompt_repo::PromptMarketRepo, MarketPrompt, MarketPromptIndex};
use crate::{error::Error, result::Result};

#[derive(Default, Debug)]
pub struct MarketPromptManager {
    repos: HashMap<String, PromptMarketRepo>,
}

impl MarketPromptManager {
    pub async fn set_repos(&mut self, repos: Vec<PromptMarketRepo>) -> Result<()> {
        self.repos = repos.into_iter().map(|r| (r.name.clone(), r)).collect();

        Ok(())
    }

    pub async fn index_list(
        &mut self,
        name: &str,
        client: Client,
    ) -> Result<Vec<MarketPromptIndex>> {
        let repo = self.repos.get(name);
        let repo = repo.ok_or(Error::NotFound("repo".to_string()))?;
        repo.fetch_index(client).await
    }

    pub async fn load_data(
        &mut self,
        name: &str,
        id: Uuid,
        client: Client,
    ) -> Result<MarketPrompt> {
        let repo = self.repos.get(name);
        let repo = repo.ok_or(Error::NotFound("repo".to_string()))?;
        repo.fetch_data(id, client).await
    }
}
