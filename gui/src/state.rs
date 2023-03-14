use std::{collections::HashMap, sync::Arc};

use askai_api::{OpenAIApi, StreamContent, Topic};
use futures::lock::Mutex;
use futures::StreamExt;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::result::Result;
use crate::setting::Setting;

pub struct Chat {
    pub id: Uuid,
    pub name: String,
    pub topic: Arc<Mutex<Topic>>,
}

impl Chat {
    pub fn new(topic: Option<String>) -> Self {
        let topic = Topic::new(topic);
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
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
}

pub struct AppState {
    chats: Mutex<HashMap<Uuid, Arc<Mutex<Chat>>>>,
    setting: Arc<Mutex<Setting>>,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        // Init setting
        let setting = Setting::init()?;

        let state = AppState {
            chats: Mutex::new(HashMap::new()),
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

    pub async fn add_chat(&self, topic: Option<String>) -> Uuid {
        let chat = Chat::new(topic);
        let chat_id = chat.id;
        self.chats
            .lock()
            .await
            .insert(chat.id, Arc::new(Mutex::new(chat)));

        chat_id
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
