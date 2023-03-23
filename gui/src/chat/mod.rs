pub mod chat_config;
pub mod chat_manager;
pub mod chat_store;

use std::{path::Path, sync::Arc};

use askai_api::{Chat as ChatApi, ChatParams, Message, OpenAIApi, Role, StreamContent};
use futures::{lock::Mutex, StreamExt};
use tokio::fs;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::error::Error;
use crate::prompt::prompt_store::PromptStore;
use crate::result::Result;
use crate::store::Store;

use self::chat_config::ChatConfig;
use self::chat_store::{ChatData, ChatIndex, ChatMetadata};

#[derive(Clone, Debug)]
pub struct ChatLogs {
    cost: f32,
    logs: Vec<ChatLog>,
}

impl ChatLogs {
    fn new(cost: f32, logs: Vec<ChatLog>) -> Self {
        Self { cost, logs }
    }

    // Add user message
    pub fn add_user_message(&mut self, message: &str, model: &str) -> Uuid {
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
        self.logs.push(log);
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
        let cost = calc_cost(model, consumed_tokens);

        let log = ChatLog {
            id: message_id,
            model: model.to_string(),
            tokens,
            consumed_tokens,
            cost,
            message,
        };
        self.cost += cost;
        self.logs.push(log);
        message_id
    }
}

pub struct Chat {
    pub index: ChatIndex,
    pub metadata: ChatMetadata,

    prompt: Option<String>,
    logs: Arc<Mutex<ChatLogs>>,
}

impl Chat {
    pub async fn init(index: ChatIndex, metadata: ChatMetadata, data: ChatData) -> Result<Self> {
        let prompt = if let Some(prompt_id) = metadata.config.prompt_id {
            let prompt_store = PromptStore::init().await?;
            let prompt_data = prompt_store.read_data(&prompt_id).await?;

            Some(prompt_data.prompt)
        } else {
            None
        };

        let logs = ChatLogs::new(data.cost, data.logs);

        Ok(Self {
            index,
            metadata,

            prompt,
            logs: Arc::new(Mutex::new(logs)),
        })
    }

    pub fn as_metadata(&self) -> ChatMetadata {
        self.metadata.clone()
    }

    // TODO: return ref
    pub async fn as_data(&self) -> ChatData {
        let logs = self.logs.lock().await;
        ChatData {
            cost: logs.cost,
            logs: logs.logs.clone(),
        }
    }

    pub async fn get_cost(&self) -> f32 {
        self.logs.lock().await.cost
    }

    pub async fn send_message(
        &self,
        sender: Sender<StreamContent>,
        message: &str,
        api: OpenAIApi,
    ) -> Result<Uuid> {
        let message_id = self.add_user_message(message).await;

        let params = self.chat_params().await;
        let context_tokens = self.calc_backtrack_messages_tokens().await;
        let logs = self.logs.clone();
        let model = self.metadata.config.model.clone();
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
                                logs.lock()
                                    .await
                                    .add_assistant_message(&reply, &model, context_tokens)
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
        let message = self.truncate_message_from(message_id).await?;

        self.send_message(sender, &message.content, api).await
    }

    pub async fn save_as_markdown(&self, path: &Path) -> Result<()> {
        let markdown = self.to_markdown().await?;

        fs::write(path, markdown).await?;

        Ok(())
    }

    async fn to_markdown(&self) -> Result<String> {
        let mut markdown = String::new();

        if !self.index.title.is_empty() {
            markdown.push_str(&format!("# {}\n\n", self.index.title));
        }

        let logs = self.logs.lock().await.logs.clone();

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
        let mut max_backtrack = self.metadata.config.max_backtrack;
        if max_backtrack == 0 {
            max_backtrack = usize::MAX;
        }
        let mut tokens = self
            .logs
            .lock()
            .await
            .logs
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
        let mut max_backtrack = self.metadata.config.max_backtrack;
        if max_backtrack == 0 {
            max_backtrack = usize::MAX;
        }
        let mut messages: Vec<Message> = self
            .logs
            .lock()
            .await
            .logs
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
            .lock()
            .await
            .add_user_message(message, &self.metadata.config.model)
    }

    async fn truncate_message_from(&self, id: Uuid) -> Result<Message> {
        let mut logs = self.logs.lock().await;

        // find the message index
        let Some(index) = logs.logs.iter().position(|log| log.id == id) else {
            return Err(Error::NotFound("message".to_string()))
        };

        let message = logs.logs[index].message.clone();

        // remove all messages after the message need to resend
        logs.logs.truncate(index);

        Ok(message)
    }

    async fn chat_params(&self) -> ChatParams {
        ChatParams {
            stream: true,
            model: self.metadata.config.model.clone(),
            messages: self.backtrack_messages().await,
            temperature: self.metadata.config.temperature,
            top_p: self.metadata.config.top_p,
            n: self.metadata.config.n,
            stop: self.metadata.config.stop.clone(),
            max_tokens: self.metadata.config.max_tokens,
            presence_penalty: self.metadata.config.presence_penalty,
            frequency_penalty: self.metadata.config.frequency_penalty,
            user: self.metadata.config.user.clone(),
        }
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

fn calc_cost(model: &str, tokens: usize) -> f32 {
    let price_per_1k = if model.starts_with("gpt-4") {
        0.12
    } else {
        0.002
    };
    let price_per_token = price_per_1k / 1000.0;

    price_per_token * tokens as f32
}
