use std::{collections::HashMap, sync::Arc};

use futures::lock::Mutex;
use uuid::Uuid;

use crate::result::Result;
use crate::store::Store;
use crate::{chat::Chat, error::Error};

use super::{
    chat_config::ChatConfig,
    chat_store::{ChatData, ChatIndex, ChatMetadata, ChatStore},
};

pub struct ChatManager {
    index_list: Vec<ChatIndex>,
    loaded_chat: Mutex<HashMap<Uuid, Arc<Mutex<Chat>>>>,
    store: ChatStore,
}

impl ChatManager {
    pub async fn init() -> Result<Self> {
        let store = ChatStore::init().await?;
        let chats = HashMap::new();

        let index_list = store.read_index_list().await?;

        Ok(Self {
            index_list,
            loaded_chat: Mutex::new(chats),
            store,
        })
    }

    pub fn list(&self) -> Vec<ChatIndex> {
        self.index_list.clone()
    }

    pub async fn create(&mut self, title: &str, prompt_id: Option<Uuid>) -> Result<Uuid> {
        let id = Uuid::new_v4();

        let index = ChatIndex {
            id,
            title: title.to_string(),
        };

        let metadata = ChatMetadata {
            title: title.to_string(),
            config: ChatConfig {
                prompt_id,
                ..Default::default()
            },
        };

        let data = ChatData {
            cost: 0.0,
            logs: Vec::new(),
        };

        self.index_list.insert(0, index.clone());
        self.store.write_index(&self.index_list).await?;

        self.store.write_metadata(&id, &metadata).await?;
        self.store.write_data(&id, &data).await?;

        let chat = Chat::init(index, metadata, data).await?;

        self.loaded_chat
            .lock()
            .await
            .insert(id, Arc::new(Mutex::new(chat)));

        Ok(id)
    }

    pub async fn delete(&mut self, chat_id: Uuid) -> Result<()> {
        self.index_list.retain(|index| index.id != chat_id);
        self.store.write_index(&self.index_list).await?;

        self.store.delete(&chat_id).await?;

        self.loaded_chat.lock().await.remove(&chat_id);

        Ok(())
    }

    pub async fn update(&mut self, payload: &ChatUpdatePayload) -> Result<()> {
        let id = payload.id;
        let mut updated_title = None;

        if let Some(title) = &payload.title {
            if let Some(index) = self.index_list.iter_mut().find(|index| index.id == id) {
                index.title = title.to_string();
                updated_title = Some(title.to_string());
            }
        }

        if let Some(config) = &payload.config {
            let mut metadata = self.store.read_metadata(&id).await?;
            metadata.config = config.clone();
            metadata.title = updated_title.unwrap_or(metadata.title);

            self.store.write_metadata(&id, &metadata).await?;
        }

        self.update_loaded(payload).await?;

        Ok(())
    }

    async fn update_loaded(&mut self, payload: &ChatUpdatePayload) -> Result<()> {
        let id = payload.id;

        if let Some(chat) = self.loaded_chat.lock().await.get_mut(&id) {
            let mut chat = chat.lock().await;

            if let Some(title) = &payload.title {
                chat.index.title = title.to_string();
            }

            if let Some(config) = &payload.config {
                chat.metadata.config = config.clone();
            }
        }

        Ok(())
    }

    pub async fn load(&self, chat_id: Uuid) -> Result<Arc<Mutex<Chat>>> {
        if !self.loaded_chat.lock().await.contains_key(&chat_id) {
            // load chat
            let metadata = self.store.read_metadata(&chat_id).await?;
            let data = self.store.read_data(&chat_id).await?;
            let index = self
                .index_list
                .iter()
                .find(|index| index.id == chat_id)
                .ok_or(Error::NotFound("chat".to_string()))?
                .clone();

            let chat = Chat::init(index, metadata, data).await?;

            // cache chat
            self.loaded_chat
                .lock()
                .await
                .insert(chat_id, Arc::new(Mutex::new(chat)));
        }

        let chats = self.loaded_chat.lock().await;
        let chat = chats
            .get(&chat_id)
            .ok_or(Error::NotFound("chat".to_string()))?;

        Ok(chat.clone())
    }

    pub async fn save_data(&self, chat_id: Uuid) -> Result<()> {
        let chat = self.load(chat_id).await?;
        let chat = chat.lock().await;

        let chat_data = chat.as_data().await;

        self.store.write_data(&chat_id, &chat_data).await?;

        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ChatUpdatePayload {
    pub id: Uuid,
    pub title: Option<String>,
    pub config: Option<ChatConfig>,
}
