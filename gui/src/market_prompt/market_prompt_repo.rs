use std::collections::HashMap;

use futures::lock::Mutex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{MarketPrompt, MarketPromptIndex};
use crate::result::Result;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PromptMarketRepo {
    pub name: String,
    url: String,
    #[serde(skip)]
    index_list: Mutex<Option<Vec<MarketPromptIndex>>>,
    #[serde(skip)]
    etag: Mutex<Option<String>>,
    #[serde(skip)]
    prompts: Mutex<HashMap<Uuid, MarketPrompt>>,
}

impl PromptMarketRepo {
    pub fn partial_clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            url: self.url.clone(),
            ..Default::default()
        }
    }
    pub async fn list(client: Client) -> Result<Vec<Self>> {
        let res = client
            .get("https://raw.githubusercontent.com/lisiur/askai-prompts-repos/main/repos.json")
            .send()
            .await?;

        let data: Vec<Self> = res.json().await?;

        Ok(data)
    }

    fn index_url(&self) -> String {
        self.url.clone() + "metadata.json"
    }

    fn data_url(&self, id: Uuid) -> String {
        self.url.clone() + "data/" + &id.to_string() + ".json"
    }

    pub async fn fetch_index(&self, client: Client) -> Result<Vec<MarketPromptIndex>> {
        let url = self.index_url();

        let mut cached_index_list = self.index_list.lock().await;
        if cached_index_list.is_some() {
            return Ok(cached_index_list.clone().unwrap());
        }

        // fetch from remote
        let res = client.get(url).send().await?;

        let etag = res
            .headers()
            .get("ETag")
            .map(|h| h.to_str().unwrap().to_string());

        let index_list = res.json::<Vec<MarketPromptIndex>>().await?;

        *self.etag.lock().await = etag;
        *cached_index_list = Some(index_list.clone());

        Ok(index_list)
    }

    pub async fn fetch_data(&self, id: Uuid, client: Client) -> Result<MarketPrompt> {
        let mut prompts = self.prompts.lock().await;
        if prompts.contains_key(&id) {
            return Ok(prompts.get(&id).unwrap().clone());
        }

        let url = self.data_url(id);
        let res = client.get(&url).send().await?;
        let market_prompt = res.json::<MarketPrompt>().await?;
        prompts.insert(id, market_prompt.clone());

        Ok(market_prompt)
    }
}
