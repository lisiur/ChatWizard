use std::collections::HashMap;

use crate::{error::Error, result::Result, store::Store};
use futures::lock::Mutex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    prompt_store::{PromptData, PromptIndex, PromptMetadata, PromptStore},
    Prompt,
};

pub struct PromptManager {
    index_list: Vec<PromptIndex>,
    loaded_prompts: Mutex<HashMap<Uuid, Prompt>>,
    store: PromptStore,
}

impl PromptManager {
    pub async fn init() -> Result<Self> {
        let store = PromptStore::init().await?;

        let index_list = store.read_index_list().await?;

        Ok(Self {
            index_list,
            loaded_prompts: Mutex::new(HashMap::new()),
            store,
        })
    }

    pub fn list(&self) -> Vec<PromptIndex> {
        self.index_list.clone()
    }

    pub async fn create(&mut self, act: &str, prompt: &str) -> Result<Uuid> {
        let id = Uuid::new_v4();

        let prompt = Prompt {
            id,
            act: act.to_string(),
            prompt: prompt.to_string(),
        };

        let index = prompt.as_index();
        let metadata = prompt.as_metadata();
        let data = prompt.as_data();

        // save to filesystem
        self.store.write_metadata(&id, &metadata).await?;
        self.store.write_data(&id, &data).await?;

        // update index list
        self.index_list.insert(0, index);
        self.store.write_index(&self.index_list).await?;

        // created prompt inserts to loaded prompts map directly
        self.loaded_prompts.lock().await.insert(id, prompt);

        Ok(id)
    }

    pub async fn update(&mut self, prompt: &PromptUpdatePayload) -> Result<()> {
        let id = prompt.id;
        let mut updated_act = None;

        if let Some(act) = &prompt.act {
            let index = self
                .index_list
                .iter_mut()
                .find(|item| item.id == id)
                .ok_or(Error::NotFound("prompt".to_string()))?;

            index.act = act.to_string();

            self.store.write_index(&self.index_list).await?;

            updated_act = Some(act);
        }

        if let Some(act) = updated_act {
            let metadata = PromptMetadata {
                act: act.to_string(),
            };

            self.store.write_metadata(&id, &metadata).await?;
        }

        if let Some(prompt) = &prompt.prompt {
            let data = PromptData {
                prompt: prompt.to_string(),
            };

            self.store.write_data(&id, &data).await?;

            // update cache
            self.loaded_prompts.lock().await.remove(&id);
            self.load(id).await?;
        }

        Ok(())
    }

    pub async fn load(&mut self, id: Uuid) -> Result<Option<Prompt>> {
        if !self.loaded_prompts.lock().await.contains_key(&id) {
            let index = self.index_list.iter().find(|item| item.id == id);
            if index.is_none() {
                return Ok(None);
            }

            let index = index.unwrap();

            let (_metadata, data) = self.store.read(&id).await?;

            let prompt = Prompt {
                id,
                act: index.act.to_string(),
                prompt: data.prompt,
            };

            self.loaded_prompts.lock().await.insert(id, prompt);
        }

        let prompt = self.loaded_prompts.lock().await.get(&id).cloned();

        Ok(prompt)
    }

    pub async fn delete(&mut self, id: Uuid) -> Result<()> {
        self.store.delete(&id).await?;

        self.index_list.retain(|item| item.id != id);

        self.store.write_index(&self.index_list).await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PromptUpdatePayload {
    pub id: Uuid,
    pub act: Option<String>,
    pub prompt: Option<String>,
}
