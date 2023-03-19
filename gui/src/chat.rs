use std::collections::HashMap;
use std::path::PathBuf;
use std::{path::Path, sync::Arc};

use askai_api::{Logs, OpenAIApi, Role, StreamContent};
use futures::{lock::Mutex, StreamExt};
use serde_json::json;
use tokio::fs;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::error::Error;
use crate::prompt::{Prompt, PromptManager};
use crate::result::Result;
use crate::utils::{ensure_directory_exists, ensure_file_exists};

pub struct Chat {
    pub id: Uuid,
    pub title: String,
    pub prompt: Option<String>,
    logs: Arc<Mutex<Logs>>,
}

impl Chat {
    pub fn new(prompt: Option<String>, title: &str) -> Self {
        let logs = Logs::new(prompt.clone());
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            prompt,
            logs: Arc::new(Mutex::new(logs)),
        }
    }

    pub fn new_with_id(id: Uuid, prompt: Option<String>, title: &str) -> Self {
        let logs = Logs::new(prompt.clone());
        Self {
            id,
            title: title.to_string(),
            prompt,
            logs: Arc::new(Mutex::new(logs)),
        }
    }

    pub async fn get_logs(&self) -> Logs {
        (*self.logs.lock().await).clone()
    }

    pub async fn send_message(
        &self,
        sender: Sender<StreamContent>,
        message: &str,
        api: OpenAIApi,
    ) -> Uuid {
        let mut topic = self.logs.lock().await;
        let message_id = topic.add_user_message(message);

        let topic = self.logs.clone();
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
        let topic = self.logs.clone();

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

    pub async fn set_logs(&self, logs: Logs) {
        *self.logs.lock().await = logs;
    }

    pub async fn topic_json_string(&self) -> String {
        self.logs.lock().await.to_json_string()
    }

    pub async fn save_as_markdown(&self, path: &Path) -> Result<()> {
        let markdown = self.to_markdown().await?;

        fs::write(path, markdown).await?;

        Ok(())
    }

    async fn to_markdown(&self) -> Result<String> {
        let mut markdown = String::new();

        if !self.title.is_empty() {
            markdown.push_str(&format!("# {}\n\n", self.title));
        }

        let messages = self.logs.lock().await.messages();

        for message in messages {
            match message.role {
                Role::User => {
                    markdown.push_str(&format!("## {}\n", message.content));
                }
                Role::Assistant => {
                    markdown.push_str(&format!("{}\n", message.content));
                }
                _ => {}
            }
        }

        Ok(markdown)
    }
}

pub struct ChatManager {
    chats: Mutex<HashMap<Uuid, Arc<Mutex<Chat>>>>,
    store: Mutex<ChatStore>,
    prompt_manager: Arc<Mutex<PromptManager>>,
}

impl ChatManager {
    pub async fn init(
        metadata_path: &Path,
        data_dir: &Path,
        prompt_manager: Arc<Mutex<PromptManager>>,
    ) -> Result<Self> {
        let store = ChatStore::init(metadata_path, data_dir).await?;
        let chats = HashMap::new();

        Ok(Self {
            chats: Mutex::new(chats),
            store: Mutex::new(store),
            prompt_manager,
        })
    }

    pub async fn all_chat_meta(&self) -> Vec<ChatMetadata> {
        let store = self.store.lock().await;
        store.metadata.clone()
    }

    pub async fn create_chat(&mut self, act: Option<String>, title: &str) -> Result<Uuid> {
        let prompt = match act {
            Some(act) => {
                let mut prompt_manager = self.prompt_manager.lock().await;
                prompt_manager.get_prompt(&act).await?
            }
            None => None,
        };

        let chat = self.store.lock().await.create_chat(prompt, title).await?;
        let chat_id = chat.id;
        self.add_chat(chat).await;

        Ok(chat_id)
    }

    pub async fn delete_chat(&mut self, chat_id: Uuid) -> Result<()> {
        self.store.lock().await.delete_chat(chat_id).await?;
        self.chats.lock().await.remove(&chat_id);

        Ok(())
    }

    pub async fn save_chat(&mut self, chat_id: Uuid) -> Result<()> {
        let chat = self.get_chat(chat_id).await?;
        let chat = chat.lock().await;
        self.store.lock().await.save_chat(&chat).await?;

        Ok(())
    }

    async fn add_chat(&mut self, chat: Chat) {
        self.chats
            .lock()
            .await
            .insert(chat.id, Arc::new(Mutex::new(chat)));
    }

    pub async fn get_chat(&self, chat_id: Uuid) -> Result<Arc<Mutex<Chat>>> {
        if !self.chats.lock().await.contains_key(&chat_id) {
            let chat = self
                .store
                .lock()
                .await
                .load_chat(chat_id, self.prompt_manager.clone())
                .await?;

            self.chats
                .lock()
                .await
                .insert(chat_id, Arc::new(Mutex::new(chat)));
        }

        let chats = self.chats.lock().await;
        let chat = chats
            .get(&chat_id)
            .ok_or(Error::NotFound("chat".to_string()))?;

        Ok(chat.clone())
    }
}

struct ChatStore {
    data_dir: PathBuf,
    metadata_path: PathBuf,
    metadata: Vec<ChatMetadata>,
}

impl ChatStore {
    pub async fn init(metadata_path: &Path, data_dir: &Path) -> Result<Self> {
        ensure_directory_exists(data_dir).await?;
        ensure_file_exists(metadata_path, || json!([]).to_string()).await?;

        let metadata_content = fs::read_to_string(metadata_path).await?;
        let metadata: Vec<ChatMetadata> = serde_json::from_str(&metadata_content).unwrap();

        Ok(Self {
            data_dir: data_dir.to_path_buf(),
            metadata_path: metadata_path.to_path_buf(),
            metadata,
        })
    }

    pub async fn create_chat(&mut self, prompt: Option<Prompt>, title: &str) -> Result<Chat> {
        let chat = Chat::new(prompt.as_ref().map(|p| p.prompt.clone()), title);

        self.metadata.insert(
            0,
            ChatMetadata {
                id: chat.id,
                title: chat.title.clone(),
                act: prompt.map(|p| p.act),
            },
        );

        // Update metadata
        self.save_metadata().await?;
        // Update chat data
        self.save_chat(&chat).await.unwrap();

        Ok(chat)
    }

    pub async fn save_chat(&self, chat: &Chat) -> Result<()> {
        let path = self.chat_save_path(chat.id);
        let chat_data = chat.topic_json_string().await;

        // Update chat data
        fs::write(&path, chat_data).await?;

        Ok(())
    }

    async fn save_metadata(&self) -> Result<()> {
        let metadata_string = serde_json::to_string(&self.metadata).unwrap();
        fs::write(&self.metadata_path, metadata_string).await?;

        Ok(())
    }

    pub async fn load_chat(
        &self,
        chat_id: Uuid,
        prompt_manager: Arc<Mutex<PromptManager>>,
    ) -> Result<Chat> {
        let chat_metadata = self.chat_metadata(chat_id)?;
        let chat_data_path = self.chat_save_path(chat_metadata.id);
        let topic_json_string = fs::read_to_string(&chat_data_path).await?;
        let logs: Logs = serde_json::from_str(&topic_json_string).unwrap();

        let prompt = match &chat_metadata.act {
            Some(act) => prompt_manager.lock().await.get_prompt(act).await?,
            None => None,
        };
        let title = &chat_metadata.title;

        let chat = Chat::new_with_id(chat_metadata.id, prompt.map(|it| it.prompt), title);
        chat.set_logs(logs).await;

        Ok(chat)
    }

    pub async fn delete_chat(&mut self, chat_id: Uuid) -> Result<()> {
        let chat_metadata = self.chat_metadata(chat_id)?;
        let chat_data_path = self.chat_save_path(chat_metadata.id);
        fs::remove_file(&chat_data_path).await?;

        self.metadata.retain(|chat| chat.id != chat_id);
        self.save_metadata().await?;

        Ok(())
    }

    fn chat_metadata(&self, chat_id: Uuid) -> Result<&ChatMetadata> {
        let chat_metadata = self
            .metadata
            .iter()
            .find(|chat| chat.id == chat_id)
            .unwrap();

        Ok(chat_metadata)
    }

    fn chat_save_path(&self, id: Uuid) -> PathBuf {
        self.data_dir.join(format!("{}.json", id))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ChatMetadata {
    id: Uuid,
    title: String,
    act: Option<String>,
}