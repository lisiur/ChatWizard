#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum OpenAIResponse<T> {
    Ok(T),
    Err(OpenAIErrorResponse),
}

#[derive(serde::Deserialize, Debug)]
pub struct OpenAIErrorResponse {
    pub error: OpenAIResponseError,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct OpenAIResponseError {
    pub message: String,
    pub r#type: String,
    pub code: Option<String>,
}
