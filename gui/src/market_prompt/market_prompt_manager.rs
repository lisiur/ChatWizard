use reqwest::Client;

use super::{MarketPrompt, MarketPromptIndex, PromptMarketRepo};
use crate::result::Result;

pub struct MarketPromptManager {
    repo: PromptMarketRepo,
}

impl MarketPromptManager {
    pub async fn init() -> Result<Self> {
        let repo = PromptMarketRepo::default();
        Ok(Self { repo })
    }

    pub async fn list(&self, client: Client) -> Result<Vec<MarketPromptIndex>> {
        self.repo.fetch_index(client).await
    }

    pub async fn load(&mut self, act: &str, client: Client) -> Result<MarketPrompt> {
        self.repo.fetch_data(act, client).await
    }
}
