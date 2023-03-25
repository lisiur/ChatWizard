pub mod market_prompt_manager;
pub mod market_prompt_repo;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarketPrompt {
    pub act: String,
    pub prompt: String,
    pub author: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarketPromptIndex {
    id: String,
    act: String,
}
