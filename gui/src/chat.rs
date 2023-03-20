use std::collections::HashMap;
use std::path::PathBuf;
use std::{path::Path, sync::Arc};

use askai_api::{Chat as ChatApi, ChatParams, Message, OpenAIApi, Role, StreamContent};
use futures::{lock::Mutex, StreamExt};
use serde_json::json;
use tokio::fs;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::error::Error;
use crate::prompt::{Prompt, PromptManager};
use crate::result::Result;
use crate::utils::{ensure_directory_exists, ensure_file_exists};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatConfig {
    model: String,

    max_backtrack: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

impl Default for ChatConfig {
    fn default() -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            max_backtrack: 4,
            temperature: Some(1.0),
            top_p: None,
            n: Some(1),
            stop: None,
            max_tokens: None,
            presence_penalty: Some(0.0),
            frequency_penalty: Some(0.0),
            user: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChatLogs(Arc<Mutex<Vec<ChatLog>>>);

impl ChatLogs {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(vec![])))
    }

    // Add user message
    pub async fn add_user_message(&self, message: &str) -> Uuid {
        let message_id = Uuid::new_v4();
        let log = ChatLog {
            id: message_id,
            message: Message {
                role: Role::User,
                content: message.to_string(),
            },
        };
        self.0.lock().await.push(log);
        message_id
    }

    // Add assistant message
    async fn add_assistant_message(&self, message: String) -> Uuid {
        let message_id = Uuid::new_v4();
        let log = ChatLog {
            id: message_id,
            message: Message {
                role: Role::Assistant,
                content: message,
            },
        };
        self.0.lock().await.push(log);
        message_id
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ChatData {
    pub config: ChatConfig,
    pub logs: Vec<ChatLog>,
}

pub struct Chat {
    pub id: Uuid,
    pub title: String,
    pub prompt: Option<String>,
    pub logs: ChatLogs,
    pub config: ChatConfig,
}

impl Chat {
    pub fn new(prompt: Option<String>, title: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            prompt,
            logs: ChatLogs::new(),
            config: ChatConfig::default(),
        }
    }

    pub fn new_with_id(id: Uuid, prompt: Option<String>, title: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            prompt,
            logs: ChatLogs::new(),
            config: ChatConfig::default(),
        }
    }

    pub async fn get_chat_params(&self) -> ChatParams {
        ChatParams {
            stream: true,
            model: self.config.model.clone(),
            messages: self.backtrack_messages().await,
            temperature: self.config.temperature,
            top_p: self.config.top_p,
            n: self.config.n,
            stop: self.config.stop.clone(),
            max_tokens: self.config.max_tokens,
            presence_penalty: self.config.presence_penalty,
            frequency_penalty: self.config.frequency_penalty,
            user: self.config.user.clone(),
        }
    }

    pub fn set_config(&mut self, config: ChatConfig) {
        self.config = config;
    }

    pub async fn get_logs(&self) -> Vec<ChatLog> {
        self.logs.0.lock().await.clone()
    }

    pub async fn set_logs(&mut self, logs: Vec<ChatLog>) {
        *(self.logs.0.lock().await) = logs;
    }

    pub async fn send_message(
        &self,
        sender: Sender<StreamContent>,
        message: Option<&str>,
        api: OpenAIApi,
    ) -> Result<Uuid> {
        let message_id = if let Some(message) = message {
            self.add_user_message(message).await
        } else {
            // Only resend can get here
            // So we can always get the last message id
            self.logs.0.lock().await.last().unwrap().id
        };

        let params = self.get_chat_params().await;
        let logs = self.logs.clone();
        tokio::spawn(async move {
            let mut reply = String::new();
            match ChatApi.send(&api, &params).await {
                Ok(mut stream) => {
                    while let Some(content) = stream.next().await {
                        match &content {
                            StreamContent::Data(data) => {
                                reply.push_str(data);
                            }
                            StreamContent::Done => {
                                logs.add_assistant_message(reply).await;
                                break;
                            }
                            _ => {}
                        }
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

        Ok(message_id)
    }

    pub async fn resend_message(
        &self,
        sender: Sender<StreamContent>,
        message_id: Uuid,
        api: OpenAIApi,
    ) -> Result<Uuid> {
        self.truncate_message_after(message_id).await?;

        self.send_message(sender, None, api).await
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

        let logs = self.logs.0.lock().await.clone();

        for log in logs {
            match log.message.role {
                Role::User => {
                    markdown.push_str(&format!("## {}\n", log.message.content));
                }
                Role::Assistant => {
                    markdown.push_str(&format!("{}\n", log.message.content));
                }
                _ => {}
            }
        }

        Ok(markdown)
    }

    async fn backtrack_messages(&self) -> Vec<Message> {
        let mut max_backtrack = self.config.max_backtrack;
        if max_backtrack == 0 {
            max_backtrack = usize::MAX;
        }
        let mut messages: Vec<Message> = self
            .logs
            .0
            .lock()
            .await
            .iter()
            .rev()
            .take(max_backtrack)
            .rev()
            .map(|log| log.message.clone())
            .collect();

        // gpt-3.5-turbo-0301 does not always pay strong attention to system messages.
        // so we add a user message to the beginning of the message list.
        if let Some(prompt) = &self.prompt {
            messages.insert(
                0,
                Message {
                    role: Role::User,
                    content: prompt.clone(),
                },
            );
        }

        messages
    }

    // Add user message
    pub async fn add_user_message(&self, message: &str) -> Uuid {
        self.logs.add_user_message(message).await
    }

    async fn truncate_message_after(&self, id: Uuid) -> Result<()> {
        // find the message index
        let Some(index) = self.logs.0.lock().await.iter().position(|log| log.id == id) else {
            return Err(Error::NotFound("message not found".to_string()))
        };

        // remove all messages after the message need to resend
        self.logs.0.lock().await.truncate(index + 1);

        Ok(())
    }

    async fn to_json(&self) -> Result<String> {
        let logs = self.get_logs().await;
        let config = &self.config;

        let mut obj = serde_json::Map::new();
        obj.insert("logs".to_string(), serde_json::to_value(logs).unwrap());
        obj.insert("config".to_string(), serde_json::to_value(config).unwrap());

        let json = serde_json::Value::Object(obj);
        let json_string = serde_json::to_string(&json).unwrap();

        Ok(json_string)
    }

    async fn set_by_json(&mut self, json_string: &str) -> Result<()> {
        let chat_data: ChatData = serde_json::from_str(json_string).unwrap();

        self.set_logs(chat_data.logs).await;
        self.set_config(chat_data.config);

        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ChatMetadata {
    id: Uuid,
    title: String,
    prompt_id: Option<Uuid>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ChatUpdatePayload {
    pub id: Uuid,
    pub title: Option<String>,
    pub prompt_id: Option<Uuid>,
    pub config: Option<ChatConfig>,
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

    pub async fn create_chat(&mut self, prompt_id: Option<Uuid>, title: &str) -> Result<Uuid> {
        let prompt = match prompt_id {
            Some(id) => {
                let mut prompt_manager = self.prompt_manager.lock().await;
                prompt_manager.get_prompt(id).await?
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

    pub async fn update_chat(&mut self, payload: ChatUpdatePayload) -> Result<()> {
        let mut store = self.store.lock().await;
        store.update_chat(&payload).await?;

        let chat = self.get_chat(payload.id).await?;
        let mut chat = chat.lock().await;

        if let Some(title) = payload.title {
            chat.title = title;
        }

        if let Some(prompt_id) = payload.prompt_id {
            let prompt = match prompt_id {
                id if id.is_nil() => None,
                id => {
                    let mut prompt_manager = self.prompt_manager.lock().await;
                    prompt_manager
                        .get_prompt(id)
                        .await?
                        .map(|prompt| prompt.prompt)
                }
            };
            chat.prompt = prompt;
        }

        if let Some(config) = payload.config {
            chat.set_config(config);
            store.save_chat_data(&chat).await?;
        }

        Ok(())
    }

    pub async fn save_chat(&mut self, chat_id: Uuid) -> Result<()> {
        let chat = self.get_chat(chat_id).await?;
        let chat = chat.lock().await;
        self.store.lock().await.save_chat_data(&chat).await?;

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
                prompt_id: prompt.map(|p| p.id),
            },
        );

        // Update metadata
        self.save_metadata().await?;
        // Update chat data
        self.save_chat_data(&chat).await.unwrap();

        Ok(chat)
    }

    pub async fn save_chat_data(&self, chat: &Chat) -> Result<()> {
        let path = self.chat_save_path(chat.id);
        let chat_data_json = chat.to_json().await?;

        // Update chat data
        fs::write(&path, chat_data_json).await?;

        Ok(())
    }

    pub async fn update_chat(&mut self, payload: &ChatUpdatePayload) -> Result<()> {
        let chat_metadata = self.chat_metadata_mut(payload.id)?;

        if let Some(title) = &payload.title {
            chat_metadata.title = title.to_string();
        }

        if let Some(prompt_id) = payload.prompt_id {
            chat_metadata.prompt_id = Some(prompt_id);
        }

        self.save_metadata().await?;

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
        let chat_json_string = fs::read_to_string(&chat_data_path).await?;

        let prompt = match &chat_metadata.prompt_id {
            Some(id) => prompt_manager.lock().await.get_prompt(*id).await?,
            None => None,
        };
        let title = &chat_metadata.title;

        let mut chat = Chat::new_with_id(chat_metadata.id, prompt.map(|it| it.prompt), title);
        chat.set_by_json(&chat_json_string).await?;

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

    fn chat_metadata_mut(&mut self, chat_id: Uuid) -> Result<&mut ChatMetadata> {
        let chat_metadata = self
            .metadata
            .iter_mut()
            .find(|chat| chat.id == chat_id)
            .unwrap();

        Ok(chat_metadata)
    }

    fn chat_save_path(&self, id: Uuid) -> PathBuf {
        self.data_dir.join(format!("{}.json", id))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ChatLog {
    pub id: Uuid,
    message: Message,
}
