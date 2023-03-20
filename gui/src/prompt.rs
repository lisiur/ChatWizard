use futures::lock::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::fs;
use uuid::Uuid;

use crate::{
    result::Result,
    utils::{ensure_directory_exists, ensure_file_exists},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Prompt {
    pub id: Uuid,
    pub act: String,
    pub prompt: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PromptUpdatePayload {
    pub id: Uuid,
    pub act: Option<String>,
    pub prompt: Option<String>,
}

pub struct PromptManager {
    prompts: Mutex<HashMap<Uuid, Prompt>>,
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

    pub async fn add_prompt(&mut self, act: &str, prompt: &str) -> Result<Uuid> {
        let uuid = Uuid::new_v4();

        let prompt = Prompt {
            id: uuid,
            act: act.to_string(),
            prompt: prompt.to_string(),
        };

        self.store.add_prompt(&prompt).await?;

        self.prompts.lock().await.insert(uuid, prompt.clone());

        Ok(uuid)
    }

    pub async fn update_prompt(&mut self, prompt: &PromptUpdatePayload) -> Result<()> {
        self.store.update_prompt(prompt).await?;

        if let Some(p) = self.prompts.lock().await.get_mut(&prompt.id) {
            if let Some(act) = &prompt.act {
                p.act = act.clone();
            }
            if let Some(prompt) = &prompt.prompt {
                p.prompt = prompt.clone();
            }
        }

        Ok(())
    }

    pub async fn get_prompt(&mut self, id: Uuid) -> Result<Option<Prompt>> {
        if self.prompts.lock().await.contains_key(&id) {
            let prompt = self.prompts.lock().await.get(&id).cloned();

            Ok(prompt)
        } else {
            let prompt = self.store.load_prompt(id).await?;
            if let Some(prompt) = &prompt {
                self.prompts.lock().await.insert(id, prompt.clone());
            }

            Ok(prompt)
        }
    }

    pub async fn delete_prompt(&mut self, id: Uuid) -> Result<()> {
        self.prompts.lock().await.remove(&id);
        self.store.delete_prompt(id).await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PromptMeta {
    pub id: Uuid,
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

    pub async fn load_prompt(&mut self, id: Uuid) -> Result<Option<Prompt>> {
        let meta = self.meta_list.iter().find(|m| m.id == id);
        if let Some(meta) = meta {
            let data_path = self.prompt_data_path(meta.id);
            if data_path.exists() {
                let content = fs::read_to_string(&self.prompt_data_path(id)).await?;
                let prompt = Prompt {
                    id: meta.id,
                    act: meta.act.clone(),
                    prompt: content,
                };
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

        // save prompt data
        self.save_data(prompt.id, &prompt.prompt).await?;
        let meta = PromptMeta {
            id: prompt.id,
            act: act.to_string(),
        };

        // update meta
        self.meta_list.push(meta.clone());
        self.save_meta().await?;

        Ok(())
    }

    pub async fn update_prompt(&mut self, payload: &PromptUpdatePayload) -> Result<()> {
        let id = payload.id;

        // update meta
        if let Some(act) = &payload.act {
            let meta = self.meta_list.iter_mut().find(|m| m.id == id);
            if let Some(meta) = meta {
                meta.act = act.to_string();
            }
            self.save_meta().await?;
        }

        // save prompt data
        if let Some(prompt) = &payload.prompt {
            self.save_data(id, prompt).await?;
        }

        Ok(())
    }

    pub async fn delete_prompt(&mut self, id: Uuid) -> Result<()> {
        // delete prompt data
        fs::remove_file(&self.prompt_data_path(id)).await?;

        // update meta
        self.meta_list.retain(|m| m.id != id);
        self.save_meta().await?;

        Ok(())
    }

    pub async fn save_meta(&self) -> Result<()> {
        let content = serde_json::to_string(&self.meta_list).unwrap();

        fs::write(&self.meta_path, content).await?;

        Ok(())
    }

    pub async fn save_data(&self, id: Uuid, prompt: &str) -> Result<()> {
        log::debug!("save prompt data: {:?}", prompt);

        let save_path = self.prompt_data_path(id);
        log::debug!("save prompt path: {:?}", &save_path);

        fs::write(&save_path, prompt).await?;

        Ok(())
    }

    fn prompt_data_path(&self, id: Uuid) -> PathBuf {
        self.data_dir.join(format!("{id}.txt"))
    }
}
