use std::{collections::HashMap, sync::Arc};

use askai_api::{OpenAIApi, StreamContent, Topic};
use futures::lock::Mutex;
use futures::StreamExt;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::result::Result;
use crate::setting::Setting;
use crate::store::Store;

pub struct Chat {
    pub id: Uuid,
    pub title: String,
    pub topic: Arc<Mutex<Topic>>,
}

impl Chat {
    pub fn new(topic: Option<String>, title: &str) -> Self {
        let topic = Topic::new(topic);
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            topic: Arc::new(Mutex::new(topic)),
        }
    }

    pub async fn send_message(
        &self,
        sender: Sender<StreamContent>,
        message: &str,
        api: OpenAIApi,
    ) -> Uuid {
        let mut topic = self.topic.lock().await;
        let message_id = topic.add_user_message(message);

        let topic = self.topic.clone();
        tokio::spawn(async move {
            let mut topic = topic.lock().await;
            match topic.send(&api).await {
                Ok(mut stream) => {
                    while let Some(content) = stream.next().await {
                        sender.send(content).await.expect("send message");
                    }
                }
                Err(err) => {
                    sender
                        .send(StreamContent::Error(err))
                        .await
                        .expect("send error");
                }
            };
        });

        message_id
    }

    pub async fn resend_message(
        &self,
        sender: Sender<StreamContent>,
        message_id: Uuid,
        api: OpenAIApi,
    ) -> Result<()> {
        let topic = self.topic.clone();

        tokio::spawn(async move {
            let mut topic = topic.lock().await;
            match topic.resend(&api, message_id).await {
                Ok(mut stream) => {
                    while let Some(content) = stream.next().await {
                        sender.send(content).await.expect("send message");
                    }
                }
                Err(err) => {
                    sender
                        .send(StreamContent::Error(err))
                        .await
                        .expect("send error");
                }
            };
        });

        Ok(())
    }

    pub async fn reset(&self) {
        self.topic.lock().await.reset();
    }

    pub fn from_topic(id: Uuid, title: &str, topic: Topic) -> Self {
        Self {
            id,
            title: title.to_string(),
            topic: Arc::new(Mutex::new(topic)),
        }
    }

    pub async fn topic_json_string(&self) -> String {
        self.topic.lock().await.to_json_string()
    }
}

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
