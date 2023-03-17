use futures::lock::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::fs;

use crate::{
    result::Result,
    utils::{ensure_directory_exists, ensure_file_exists},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Prompt {
    pub act: String,
    pub prompt: String,
}

pub struct PromptManager {
    prompts: Mutex<HashMap<String, Prompt>>,
    store: PromptStore,
}

impl PromptManager {
    pub async fn init(meta_path: &Path, data_path: &Path) -> Result<Self> {
        let store = PromptStore::init(meta_path, data_path).await?;
        Ok(Self {
            prompts: Mutex::new(HashMap::new()),
            store,
        })
    }

    pub fn all_prompt_meta(&self) -> &Vec<PromptMeta> {
        &self.store.meta_list
    }

    pub async fn add_prompt(&mut self, prompt: &Prompt) -> Result<()> {
        self.store.add_prompt(prompt).await?;

        self.prompts
            .lock()
            .await
            .insert(prompt.act.clone(), prompt.clone());

        Ok(())
    }

    pub async fn update_prompt(&mut self, prompt: &Prompt) -> Result<()> {
        self.store.update_prompt(prompt).await?;

        self.prompts
            .lock()
            .await
            .insert(prompt.act.clone(), prompt.clone());

        Ok(())
    }

    pub async fn get_prompt(&mut self, act: &str) -> Result<Option<Prompt>> {
        if self.prompts.lock().await.contains_key(act) {
            let prompt = self.prompts.lock().await.get(act).cloned();

            Ok(prompt)
        } else {
            let prompt = self.store.load_prompt(act).await?;
            if let Some(prompt) = &prompt {
                self.prompts
                    .lock()
                    .await
                    .insert(prompt.act.clone(), prompt.clone());
            }

            Ok(prompt)
        }
    }

    pub async fn delete_prompt(&mut self, act: &str) -> Result<()> {
        self.prompts.lock().await.remove(act);
        self.store.delete_prompt(act).await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PromptMeta {
    pub act: String,
}

struct PromptStore {
    meta_path: PathBuf,
    data_dir: PathBuf,
    meta_list: Vec<PromptMeta>,
}

impl PromptStore {
    pub async fn init(meta_path: &Path, data_dir: &Path) -> Result<Self> {
        ensure_file_exists(meta_path, || json!([]).to_string()).await?;
        ensure_directory_exists(data_dir).await?;

        let content = fs::read_to_string(meta_path).await?;
        let meta_list: Vec<PromptMeta> = serde_json::from_str(&content).unwrap();

        Ok(Self {
            meta_path: meta_path.to_path_buf(),
            data_dir: data_dir.to_path_buf(),
            meta_list,
        })
    }

    pub async fn load_prompt(&mut self, act: &str) -> Result<Option<Prompt>> {
        let meta = self.meta_list.iter().find(|m| m.act == act);
        if let Some(meta) = meta {
            let data_path = self.prompt_data_path(&meta.act);
            if data_path.exists() {
                let content = fs::read_to_string(&self.prompt_data_path(act)).await?;
                let prompt: Prompt = serde_json::from_str(&content).unwrap();
                Ok(Some(prompt))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn add_prompt(&mut self, prompt: &Prompt) -> Result<()> {
        let act = &prompt.act;

        let meta = PromptMeta {
            act: act.to_string(),
        };

        // update meta
        self.meta_list.push(meta.clone());
        self.save_meta().await?;

        // save prompt data
        self.save_data(prompt).await?;

        Ok(())
    }

    pub async fn update_prompt(&mut self, prompt: &Prompt) -> Result<()> {
        // save prompt data
        self.save_data(prompt).await?;

        Ok(())
    }

    pub async fn delete_prompt(&mut self, act: &str) -> Result<()> {
        // delete prompt data
        fs::remove_file(&self.prompt_data_path(act)).await?;

        // update meta
        self.meta_list.retain(|m| m.act != act);
        self.save_meta().await?;

        Ok(())
    }

    pub async fn save_meta(&self) -> Result<()> {
        let content = serde_json::to_string(&self.meta_list).unwrap();

        fs::write(&self.meta_path, content).await?;

        Ok(())
    }

    pub async fn save_data(&self, prompt: &Prompt) -> Result<()> {
        fs::write(
            &self.prompt_data_path(&prompt.act),
            serde_json::to_string(prompt).unwrap(),
        )
        .await?;

        Ok(())
    }

    fn prompt_data_path(&self, act: &str) -> PathBuf {
        self.data_dir.join(act).join(".json")
    }
}
