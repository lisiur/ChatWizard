pub mod market_prompt_manager;

use std::collections::HashMap;

use crate::result::Result;
use futures::lock::Mutex;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarketPrompt {
    pub act: String,
    pub prompt: String,
    pub author: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarketPromptIndex {
    act: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PromptMarketRepo {
    name: String,
    url: String,
    #[serde(skip)]
    index_list: Mutex<Option<Vec<MarketPromptIndex>>>,
    #[serde(skip)]
    etag: Mutex<Option<String>>,
    #[serde(skip)]
    prompts: Mutex<HashMap<String, MarketPrompt>>,
}

impl Default for PromptMarketRepo {
    fn default() -> Self {
        Self {
            name: "Github".to_string(),
            url: "https://raw.githubusercontent.com/lisiur/askai/main/prompts/en/".to_string(),
            index_list: Mutex::new(None),
            etag: Mutex::new(None),
            prompts: Mutex::new(HashMap::new()),
        }
    }
}

impl PromptMarketRepo {
    fn index_url(&self) -> String {
        self.url.clone() + "index.json"
    }

    fn data_url(&self, act: &str) -> String {
        self.url.clone() + "data/" + act + ".json"
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

    pub async fn fetch_data(&self, act: &str, client: Client) -> Result<MarketPrompt> {
        let mut prompts = self.prompts.lock().await;
        if prompts.contains_key(act) {
            return Ok(prompts.get(act).unwrap().clone());
        }

        let url = self.data_url(act);
        let res = client.get(&url).send().await?;
        let market_prompt = res.json::<MarketPrompt>().await?;
        prompts.insert(act.to_string(), market_prompt.clone());

        Ok(market_prompt)
    }
}
