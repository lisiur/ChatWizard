use futures::{stream, Stream, StreamExt};
use std::pin::Pin;

use crate::error::{ApiErrorResponse, Error};
use crate::result::Result;

use crate::{
    types::{FinishReason, Usage},
    OpenAIApi,
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ChatParams {
    /// ID of the model to use.
    pub model: String,

    /// The messages to generate chat completions for, in the chat format.
    pub messages: Vec<Message>,

    /// What sampling temperature to use, between 0 and 2.
    /// Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    ///
    /// We generally recommend altering this or top_p but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or top_p but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or temperature but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// If set, partial message deltas will be sent, like in ChatGPT.
    /// Tokens will be sent as data-only server-sent events as they become available,
    /// with the stream terminated by a data: [DONE] message.
    pub stream: bool,

    /// Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    /// The maximum number of tokens to generate in the chat completion.
    ///
    /// The total length of input tokens and generated tokens is limited by the model's context length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// Number between -2.0 and 2.0.
    /// Positive values penalize new tokens based on whether they appear in the text so far,
    /// increasing the model's likelihood to talk about new topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// Number between -2.0 and 2.0.
    /// Positive values penalize new tokens based on their existing frequency in the text so far,
    /// decreasing the model's likelihood to repeat the same line verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Default for ChatParams {
    fn default() -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![],
            temperature: None,
            top_p: None,
            n: None,
            stream: true,
            stop: None,
            max_tokens: None,
            presence_penalty: None,
            frequency_penalty: None,
            user: None,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Message {
    pub role: Role,
    pub content: String,
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
    pub async fn create_chat(&self, params: &ChatParams) -> Result<reqwest::Response> {
        self.client
            .post_stream::<CreateChatResponseData>(
                "https://api.openai.com/v1/chat/completions",
                params,
            )
            .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone, Debug)]
pub struct Chat;

impl Chat {
    pub async fn send(
        &mut self,
        api: &OpenAIApi,
        params: &ChatParams,
    ) -> Result<Pin<Box<dyn Stream<Item = StreamContent> + Send + '_>>> {
        let res = api.create_chat(params).await?;

        let stream = res.bytes_stream();

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
                    .collect::<Vec<StreamContent>>();

                stream::iter(chunks)
            })
            .boxed();

        Ok(stream)
    }
}
