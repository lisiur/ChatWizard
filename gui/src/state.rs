use std::{collections::HashMap, sync::Arc};

use askai_api::OpenAIApi;
use futures::lock::Mutex;

use crate::chat::Chat;
use crate::result::Result;
use crate::setting::Setting;
use crate::store::Store;
use uuid::Uuid;

pub struct AppState {
    pub store: Arc<Mutex<Store>>,
    pub chats: Arc<Mutex<HashMap<Uuid, Arc<Mutex<Chat>>>>>,
    pub setting: Arc<Mutex<Setting>>,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        // Init setting
        let setting = Setting::init()?;

        let store = Store::init().await?;

        let state = AppState {
            store: Arc::new(Mutex::new(store)),
            chats: Arc::new(Mutex::new(HashMap::new())),
            setting: Arc::new(Mutex::new(setting)),
        };

        Ok(state)
    }

    pub async fn get_chat(&self, chat_id: Uuid) -> Arc<Mutex<Chat>> {
        self.chats
            .lock()
            .await
            .get(&chat_id)
            .expect("chat id not found")
            .clone()
    }

    pub async fn create_chat(&self, topic: Option<String>, title: &str) -> Result<Uuid> {
        let mut store = self.store.lock().await;
        let chat = store.create_chat(topic, title).await?;
        let chat_id = chat.id;
        self.add_chat(chat).await;

        Ok(chat_id)
    }

    pub async fn delete_chat(&self, chat_id: Uuid) -> Result<()> {
        let mut store = self.store.lock().await;
        store.delete_chat(chat_id).await?;
        self.chats.lock().await.remove(&chat_id);

        Ok(())
    }

    pub async fn save_chat(&self, chat_id: Uuid) -> Result<()> {
        let chat = self.get_chat(chat_id).await;
        let chat = chat.lock().await;
        self.store.lock().await.save_chat(&chat).await
    }

    async fn add_chat(&self, chat: Chat) {
        self.chats
            .lock()
            .await
            .insert(chat.id, Arc::new(Mutex::new(chat)));
    }

    pub async fn read_chat(&self, chat_id: Uuid) -> Result<Arc<Mutex<Chat>>> {
        if self.chats.lock().await.contains_key(&chat_id) {
            return Ok(self.get_chat(chat_id).await);
        }
        let store = self.store.lock().await;
        let chat = store.read_chat(chat_id).await?;
        let chat = Arc::new(Mutex::new(chat));

        self.chats.lock().await.insert(chat_id, chat.clone());

        Ok(chat)
    }

    pub async fn create_api(&self) -> Result<OpenAIApi> {
        self.setting.lock().await.create_api()
    }

    pub async fn set_api_key(&self, api_key: &str) -> Result<()> {
        let mut setting = self.setting.lock().await;
        setting.set_api_key(api_key)
    }

    pub async fn set_proxy(&self, proxy: &str) -> Result<()> {
        let mut setting = self.setting.lock().await;
        setting.set_proxy(proxy)
    }

    pub async fn get_proxy(&self) -> Result<Option<String>> {
        let setting = self.setting.lock().await;
        Ok(setting.opts.proxy.clone())
    }

    pub async fn has_api_key(&self) -> Result<bool> {
        let setting = self.setting.lock().await;
        Ok(setting.opts.api_key.is_some())
    }
}
