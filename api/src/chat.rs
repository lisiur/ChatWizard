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
    role: Role,
    content: String,
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
pub struct Topic {
    logs: Vec<ChatLog>,
    api: OpenAIApi,
}

impl Topic {
    pub fn new(api: OpenAIApi, topic: Option<String>) -> Self {
        let mut logs = vec![];

        if let Some(topic) = topic {
            logs.push(ChatLog {
                id: Uuid::new_v4(),
                message: Message {
                    role: Role::System,
                    content: topic,
                },
            });
        }

        Self { logs, api }
    }

    fn messages(&self) -> Vec<Message> {
        self.logs.iter().map(|log| log.message.clone()).collect()
    }

    pub async fn send_message(
        &mut self,
        message: &str,
        message_id: Option<Uuid>,
    ) -> Result<Pin<Box<dyn Stream<Item = StreamContent> + Send>>> {
        let message_id = message_id.unwrap_or_else(Uuid::new_v4);
        let log = ChatLog {
            id: message_id,
            message: Message {
                role: Role::User,
                content: message.to_string(),
            },
        };

        self.logs.push(log);

        let res = self
            .api
            .create_chat(CreateChatRequestParams {
                messages: self.messages(),
                ..CreateChatRequestParams::default()
            })
            .await?;

        let stream = res.bytes_stream();

        let stream = stream
            .flat_map(|chunk| {
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
                            Some(StreamContent::Done)
                        } else {
                            let json_data = &line[6..];
                            let json = serde_json::from_str::<StreamChunk>(json_data).unwrap();
                            json.choices.get(0).and_then(|choice| {
                                choice
                                    .delta
                                    .content
                                    .as_ref()
                                    .map(|content| StreamContent::Data(content.to_string()))
                            })
                        }
                    })
                    .collect::<Vec<_>>();

                stream::iter(chunks)
            })
            .boxed();

        Ok(stream)
    }

    pub async fn resend_message(
        &mut self,
        id: Uuid,
    ) -> Result<Pin<Box<dyn Stream<Item = StreamContent> + Send>>> {
        // find the message index
        let Some(index) = self.logs.iter().position(|log| log.id == id) else {
            return Err(Error::NotFound("message not found".to_string()))
        };

        // find the message need to resend
        let message = self.logs[index].message.content.clone();

        // remove all messages after the message need to resend
        self.logs.truncate(index);

        // send the message again
        self.send_message(&message, Some(id)).await
    }

    pub fn truncate_message_from(&mut self, id: Uuid) -> Result<()> {
        // find the message index
        let Some(index) = self.logs.iter().position(|log| log.id == id) else {
            return Err(Error::NotFound("message not found".to_string()))
        };

        // remove all messages after the message need to resend
        self.logs.truncate(index);

        Ok(())
    }

    pub fn discard_message(&mut self, id: Uuid) -> Result<()> {
        // find the message index
        let Some(index) = self.logs.iter().position(|log| log.id == id) else {
            return Err(Error::NotFound("message not found".to_string()))
        };

        // remove the message
        self.logs.remove(index);

        Ok(())
    }

    pub async fn set_proxy(&self, proxy: &str) {
        self.api.set_proxy(proxy).await;
    }

    pub fn reset(&mut self) {
        self.logs.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat_topic() {
        dotenv::dotenv().unwrap();

        let api = OpenAIApi::new(&std::env::var("OPENAI_API").unwrap());
        api.set_proxy(&std::env::var("PROXY").unwrap()).await;

        let mut topic = Topic::new(
            api,
            Some("Repeat what user says, no more other words".to_string()),
        );

        assert!(topic.send_message("Hello!", None).await.is_ok());
    }
}
