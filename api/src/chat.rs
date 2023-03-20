use futures::{stream, Stream, StreamExt};
use std::pin::Pin;
use uuid::Uuid;

use crate::error::{ApiErrorResponse, Error};
use crate::result::Result;

use crate::{
    types::{FinishReason, Usage},
    OpenAIApi,
};

#[derive(serde::Serialize, Debug)]
pub struct CreateChatRequestParams {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

impl Default for CreateChatRequestParams {
    fn default() -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![],
            stream: true,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ChatLog {
    pub id: Uuid,
    message: Message,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateChatResponseData {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<CreateChatResponseChoices>,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateChatResponseChoices {
    pub message: Message,
    pub finish_reason: FinishReason,
    pub index: Option<usize>,
}

#[derive(serde::Deserialize, Debug)]
pub struct StreamChunk {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<StreamChunkChoice>,
}

#[derive(serde::Deserialize, Debug)]
pub struct StreamChunkChoice {
    pub delta: StreamChunkChoiceDelta,
    pub index: usize,
    pub finish_reason: Option<FinishReason>,
}

#[derive(serde::Deserialize, Debug)]
pub struct StreamChunkChoiceDelta {
    pub role: Option<Role>,
    pub content: Option<String>,
}

#[derive(serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum StreamContent {
    Error(Error),
    Data(String),
    Done,
}

impl OpenAIApi {
    pub async fn create_chat(&self, params: CreateChatRequestParams) -> Result<reqwest::Response> {
        self.client
            .post_stream::<CreateChatResponseData>(
                "https://api.openai.com/v1/chat/completions",
                params,
            )
            .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Chat(Vec<ChatLog>);

impl Chat {
    pub fn new(prompt: Option<String>) -> Self {
        let mut logs = vec![];

        if let Some(prompt) = prompt {
            logs.push(ChatLog {
                id: Uuid::new_v4(),
                message: Message {
                    role: Role::System,
                    content: prompt,
                },
            });
        }

        Self(logs)
    }

    pub fn get_logs(&self) -> &Vec<ChatLog> {
        &self.0
    }

    pub fn from_logs(logs: Vec<ChatLog>) -> Self {
        Self(logs)
    }

    pub fn messages(&self) -> Vec<Message> {
        self.0.iter().map(|log| log.message.clone()).collect()
    }

    pub fn set_prompt(&mut self, prompt: Option<&str>) {
        if let Some(prompt) = prompt {
            if let Some(log) = self.0.first_mut() {
                if matches!(log.message.role, Role::System) {
                    log.message.content = prompt.to_string();
                    return;
                }
            }
            self.0.push(ChatLog {
                id: Uuid::new_v4(),
                message: Message {
                    role: Role::System,
                    content: prompt.to_string(),
                },
            });
        } else {
            self.unset_prompt();
        }
    }

    pub fn unset_prompt(&mut self) {
        if self.0.first().is_some() && matches!(self.0.first().unwrap().message.role, Role::System)
        {
            self.0.remove(0);
        }
    }

    fn limited_messages(&self, limit: usize) -> Vec<Message> {
        let mut messages: Vec<Message> = self
            .0
            .iter()
            .rev()
            .take(limit)
            .rev()
            .map(|log| log.message.clone())
            .collect();

        // gpt-3.5-turbo-0301 does not always pay strong attention to system messages.
        // so we add a user message to the beginning of the message list.
        if let Some(log) = self.0.first() {
            let message = &log.message;
            if matches!(message.role, Role::System) {
                messages.insert(
                    0,
                    Message {
                        role: Role::User,
                        content: message.content.clone(),
                    },
                );
            }
        }

        messages
    }

    // Add user message
    pub fn add_user_message(&mut self, message: &str) -> Uuid {
        let message_id = Uuid::new_v4();
        let log = ChatLog {
            id: message_id,
            message: Message {
                role: Role::User,
                content: message.to_string(),
            },
        };
        self.0.push(log);
        message_id
    }

    // Add assistant message
    fn add_assistant_message(&mut self, message: String) -> Uuid {
        let message_id = Uuid::new_v4();
        let log = ChatLog {
            id: message_id,
            message: Message {
                role: Role::Assistant,
                content: message,
            },
        };
        self.0.push(log);
        message_id
    }

    pub async fn send(
        &mut self,
        api: &OpenAIApi,
    ) -> Result<Pin<Box<dyn Stream<Item = StreamContent> + Send + '_>>> {
        let res = api
            .create_chat(CreateChatRequestParams {
                messages: self.limited_messages(4),
                ..CreateChatRequestParams::default()
            })
            .await?;

        let stream = res.bytes_stream();

        let mut reply = String::new();
        let stream = stream
            .flat_map(move |chunk| {
                if let Err(err) = chunk {
                    return stream::iter(vec![StreamContent::Error(err.into())]);
                }
                let data = String::from_utf8(chunk.unwrap().to_vec()).unwrap();

                log::debug!("data: {}", data);

                if data.starts_with("{\n    \"error\"") || data.starts_with("{\n  \"error\"") {
                    let res = serde_json::from_str::<ApiErrorResponse>(&data).unwrap();
                    return stream::iter(vec![StreamContent::Error(res.into())]);
                }

                let chunks = data
                    .lines()
                    .filter_map(|line| {
                        let line = line.trim();
                        if line.is_empty() {
                            None
                        } else if line.starts_with("data: [DONE]") {
                            self.add_assistant_message(reply.clone());
                            Some(StreamContent::Done)
                        } else {
                            let json_data = &line[6..];
                            let json = serde_json::from_str::<StreamChunk>(json_data).unwrap();
                            json.choices.get(0).and_then(|choice| {
                                choice.delta.content.as_ref().map(|content| {
                                    reply.push_str(content);
                                    StreamContent::Data(content.to_string())
                                })
                            })
                        }
                    })
                    .collect::<Vec<StreamContent>>();

                stream::iter(chunks)
            })
            .boxed();

        Ok(stream)
    }

    pub async fn resend(
        &mut self,
        api: &OpenAIApi,
        message_id: Uuid,
    ) -> Result<Pin<Box<dyn Stream<Item = StreamContent> + Send + '_>>> {
        self.truncate_message_after(message_id)?;

        // send the message again
        self.send(api).await
    }

    fn truncate_message_after(&mut self, id: Uuid) -> Result<()> {
        // find the message index
        let Some(index) = self.0.iter().position(|log| log.id == id) else {
            return Err(Error::NotFound("message not found".to_string()))
        };

        // remove all messages after the message need to resend
        self.0.truncate(index + 1);

        Ok(())
    }

    pub fn discard_message(&mut self, id: Uuid) -> Result<()> {
        // find the message index
        let Some(index) = self.0.iter().position(|log| log.id == id) else {
            return Err(Error::NotFound("message not found".to_string()))
        };

        // remove the message
        self.0.remove(index);

        Ok(())
    }

    pub fn reset(&mut self) {
        self.0.clear();
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat_topic() {
        dotenv::dotenv().unwrap();

        let mut api = OpenAIApi::new(&std::env::var("OPENAI_API").unwrap());
        api.set_proxy(&std::env::var("PROXY").unwrap());

        let mut topic = Chat::new(Some(
            "Repeat what user says, no more other words".to_string(),
        ));

        topic.add_user_message("Hello!");

        assert!(topic.send(&api).await.is_ok());
    }
}
