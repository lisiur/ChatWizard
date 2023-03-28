use std::pin::Pin;

use chrono::format::Item;
use futures::{stream, Stream, StreamExt};

use crate::{
    api::client::Client,
    error::{ApiError, Error, NetworkError},
    result::Result,
};

use self::params::{OpenAIChatParams, OpenAIChatRole};

use super::response::OpenAIErrorResponse;

pub mod params;

pub struct OpenAIChat {
    client: Client,
    host: Option<String>,
}

impl OpenAIChat {
    pub fn new(client: Client, host: Option<String>) -> Self {
        Self { client, host }
    }

    async fn send_message(
        &self,
        params: &OpenAIChatParams,
    ) -> Result<Pin<Box<dyn Stream<Item = OpenAIStreamContent> + Send + '_>>> {
        let url = self.host.clone().unwrap_or_default() + "/v1/chat/completions";
        let res = self.client.post(&url, params).await?;

        let stream = res.bytes_stream();

        let mut left_chunk: Option<Vec<u8>> = None;
        let mut left_line: Option<String> = None;
        let stream = stream
            .flat_map(move |chunk| {
                if let Err(err) = chunk {
                    return stream::iter(vec![OpenAIStreamContent::Error(err.into())]);
                }

                let mut vec = chunk.unwrap().to_vec();
                if let Some(left_chunk) = left_chunk.take() {
                    vec = [left_chunk, vec].concat();
                }

                let Ok(data) = String::from_utf8(vec.clone()) else {
                    left_chunk = Some(vec);
                    return stream::iter(vec![]);
                };

                if data.starts_with("{\n    \"error\"") || data.starts_with("{\n  \"error\"") {
                    let res = serde_json::from_str::<OpenAIErrorResponse>(&data).unwrap();
                    return stream::iter(vec![OpenAIStreamContent::Error(res.into())]);
                }

                let chunks = data
                    .lines()
                    .filter_map(|line| {
                        let line = line.trim();
                        if line.is_empty() {
                            None
                        } else if line.starts_with("data: [DONE]") {
                            Some(OpenAIStreamContent::Done)
                        } else if line.starts_with("data: ") && line.ends_with("}]}") {
                            handle_line(line)
                        } else if line.ends_with('}') {
                            let line = left_line.take().unwrap_or_default() + line;
                            match handle_line(&line) {
                                Some(content) => Some(content),
                                None => {
                                    left_line = Some(line);
                                    None
                                }
                            }
                        } else {
                            left_line = Some(left_line.take().unwrap_or_default() + line);
                            None
                        }
                    })
                    .collect::<Vec<OpenAIStreamContent>>();

                stream::iter(chunks)
            })
            .boxed();

        Ok(stream)
    }
}

#[derive(serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum OpenAIStreamContent {
    Error(OpenAIStreamError),
    Data(String),
    Done,
}

#[derive(thiserror::Error, serde::Serialize, Clone, Debug)]
pub enum OpenAIStreamError {
    #[error(transparent)]
    Api(ApiError),

    #[error(transparent)]
    Network(NetworkError),

    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<reqwest::Error> for OpenAIStreamError {
    fn from(err: reqwest::Error) -> Self {
        OpenAIStreamError::Network(if err.is_timeout() {
            NetworkError::Timeout(err.to_string())
        } else {
            NetworkError::Unknown(err.to_string())
        })
    }
}

impl From<OpenAIErrorResponse> for OpenAIStreamError {
    fn from(err: OpenAIErrorResponse) -> Self {
        match err.error.code.as_deref() {
            Some("invalid_api_key") => Self::Api(ApiError::InvalidKey),
            _ => Self::Api(ApiError::Unknown(err.error.message)),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct StreamChunk {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<OpenAIStreamChunkChoice>,
}

#[derive(serde::Deserialize, Debug)]
pub struct OpenAIStreamChunkChoice {
    pub delta: OpenAIStreamChunkChoiceDelta,
    pub index: usize,
    pub finish_reason: Option<OpenAIFinishReason>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OpenAIFinishReason {
    Stop,
    Length,
    ContentFilter,
}

#[derive(serde::Deserialize, Debug)]
pub struct OpenAIStreamChunkChoiceDelta {
    pub role: Option<OpenAIChatRole>,
    pub content: Option<String>,
}

fn handle_line(line: &str) -> Option<OpenAIStreamContent> {
    if !line.starts_with("data:") || !line.ends_with("}]}") {
        return None;
    }
    let json_data = if line.starts_with("data: {") {
        &line[6..]
    } else if line.starts_with("data:{") {
        &line[5..]
    } else {
        return None;
    };
    let json = serde_json::from_str::<StreamChunk>(json_data).unwrap();
    json.choices.get(0).and_then(|choice| {
        choice
            .delta
            .content
            .as_ref()
            .map(|content| OpenAIStreamContent::Data(content.to_string()))
    })
}
