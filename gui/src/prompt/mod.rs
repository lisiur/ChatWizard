pub mod prompt_manager;
pub mod prompt_store;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::prompt_store::{PromptData, PromptIndex, PromptMetadata};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Prompt {
    pub id: Uuid,
    pub act: String,
    pub prompt: String,
}

impl Prompt {
    pub fn as_index(&self) -> PromptIndex {
        PromptIndex {
            id: self.id,
            act: self.act.to_string(),
        }
    }

    pub fn as_metadata(&self) -> PromptMetadata {
        PromptMetadata {
            act: self.act.to_string(),
        }
    }

    pub fn as_data(&self) -> PromptData {
        PromptData {
            prompt: self.prompt.to_string(),
        }
    }
}
