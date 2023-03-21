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
pub struct ChatLogs {
    cost: Arc<Mutex<f32>>,
    logs: Arc<Mutex<Vec<ChatLog>>>,
}

impl ChatLogs {
    fn new() -> Self {
        Self {
            cost: Arc::new(Mutex::new(0.0)),
            logs: Arc::new(Mutex::new(vec![])),
        }
    }

    // Add user message
    pub async fn add_user_message(&self, message: &str, model: &str) -> Uuid {
        let message_id = Uuid::new_v4();
        let message = Message {
            role: Role::User,
            content: message.to_string(),
        };
        let log = ChatLog {
            id: message_id,
            model: model.to_string(),
            tokens: message.calc_tokens(),
            consumed_tokens: 0,
            cost: 0.0,
            message,
        };
        self.logs.lock().await.push(log);
        message_id
    }

    // Add assistant message
    async fn add_assistant_message(
        &mut self,
        message: &str,
        model: &str,
        context_tokens: usize,
    ) -> Uuid {
        let message_id = Uuid::new_v4();
        let message = Message {
            role: Role::Assistant,
            content: message.to_string(),
        };
        let tokens = message.calc_tokens();
        let mut consumed_tokens = context_tokens + tokens;
        consumed_tokens += 2; // every reply is primed with <im_start>assistant
        let cost = Self::calc_cost(model, consumed_tokens);

        let log = ChatLog {
            id: message_id,
            model: model.to_string(),
            tokens,
            consumed_tokens,
            cost,
            message,
        };
        log::debug!("Assistant message cost: {}", cost);
        *self.cost.lock().await += cost;
        log::debug!("Total cost: {}", *self.cost.lock().await);
        self.logs.lock().await.push(log);
        message_id
    }

    fn calc_cost(model: &str, tokens: usize) -> f32 {
        let price_per_1k = if model.starts_with("gpt-4") {
            0.12
        } else {
            0.002
        };
        let price_per_token = price_per_1k / 1000.0;

        price_per_token * tokens as f32
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ChatData {
    pub config: ChatConfig,
    pub logs: Vec<ChatLog>,
    pub cost: f32,
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

    pub async fn get_cost(&self) -> f32 {
        *self.logs.cost.lock().await
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
        self.logs.logs.lock().await.clone()
    }

    pub async fn set_logs(&mut self, logs: Vec<ChatLog>) {
        *(self.logs.logs.lock().await) = logs;
    }

    pub async fn set_cost(&mut self, cost: f32) {
        *(self.logs.cost.lock().await) = cost;
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
            self.logs.logs.lock().await.last().unwrap().id
        };

        let params = self.get_chat_params().await;
        let context_tokens = self.calc_backtrack_messages_tokens().await;
        let mut logs = self.logs.clone();
        let model = self.config.model.clone();
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
                                logs.add_assistant_message(&reply, &model, context_tokens)
                                    .await;
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

        let logs = self.logs.logs.lock().await.clone();

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

    async fn calc_backtrack_messages_tokens(&self) -> usize {
        let mut max_backtrack = self.config.max_backtrack;
        if max_backtrack == 0 {
            max_backtrack = usize::MAX;
        }
        let mut tokens = self
            .logs
            .logs
            .lock()
            .await
            .iter()
            .rev()
            .take(max_backtrack)
            .map(|it| it.tokens)
            .sum();

        if let Some(prompt) = &self.prompt {
            tokens += Message {
                role: Role::User,
                content: prompt.clone(),
            }
            .calc_tokens();
        }

        tokens
    }

    async fn backtrack_messages(&self) -> Vec<Message> {
        let mut max_backtrack = self.config.max_backtrack;
        if max_backtrack == 0 {
            max_backtrack = usize::MAX;
        }
        let mut messages: Vec<Message> = self
            .logs
            .logs
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
        self.logs
            .add_user_message(message, &self.config.model)
            .await
    }

    async fn truncate_message_after(&self, id: Uuid) -> Result<()> {
        // find the message index
        let Some(index) = self.logs.logs.lock().await.iter().position(|log| log.id == id) else {
            return Err(Error::NotFound("message not found".to_string()))
        };

        // remove all messages after the message need to resend
        self.logs.logs.lock().await.truncate(index + 1);

        Ok(())
    }

    async fn to_json(&self) -> Result<String> {
        let logs = self.get_logs().await;
        let config = &self.config;
        let cost = *self.logs.cost.lock().await;

        let mut obj = serde_json::Map::new();
        obj.insert("logs".to_string(), serde_json::to_value(logs).unwrap());
        obj.insert("config".to_string(), serde_json::to_value(config).unwrap());
        obj.insert("cost".to_string(), serde_json::to_value(cost).unwrap());

        let json = serde_json::Value::Object(obj);
        let json_string = serde_json::to_string(&json).unwrap();

        Ok(json_string)
    }

    async fn set_by_json(&mut self, json_string: &str) -> Result<()> {
        let chat_data: ChatData = serde_json::from_str(json_string).unwrap();

        self.set_logs(chat_data.logs).await;
        self.set_config(chat_data.config);
        self.set_cost(chat_data.cost).await;

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

    pub async fn save_chat(&mut self, chat_id: Uuid) -> Result<f32> {
        let chat = self.get_chat(chat_id).await?;
        let chat = chat.lock().await;
        let cost = *chat.logs.cost.lock().await;
        self.store.lock().await.save_chat_data(&chat).await?;

        Ok(cost)
    }

    async fn add_chat(&mut self, chat: Chat) {
        self.chats
            .lock()
            .await
            .insert(chat.id, Arc::new(Mutex::new(chat)));
    }

    pub async fn get_chat(&self, chat_id: Uuid) -> Result<Arc<Mutex<Chat>>> {
        if !self.chats.lock().await.contains_key(&chat_id) {
            // load chat
            let chat = self
                .store
                .lock()
                .await
                .load_chat(chat_id, self.prompt_manager.clone())
                .await?;

            // cache chat
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
    tokens: usize,
    model: String,
    consumed_tokens: usize,
    cost: f32,
    message: Message,
}
