#[derive(thiserror::Error, serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum Error {
    #[error("network error: {}", .0.message)]
    Network(NetworkError),

    #[error("api error: {}", .0.message)]
    Api(ApiError),

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("not found: {}", .0)]
    NotFound(String),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct NetworkError {
    pub r#type: String,
    pub message: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ApiErrorResponse {
    error: ApiError,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ApiError {
    pub message: String,
    pub r#type: String,
    pub code: Option<String>,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Network(if err.is_timeout() {
            NetworkError {
                r#type: "timeout".to_string(),
                message: err.to_string(),
            }
        } else {
            NetworkError {
                r#type: "unknown".to_string(),
                message: err.to_string(),
            }
        })
    }
}

impl From<ApiErrorResponse> for Error {
    fn from(err: ApiErrorResponse) -> Self {
        match err.error.code.as_deref() {
            Some("invalid_api_key") => Error::InvalidApiKey,
            _ => Error::Api(err.error),
        }
    }
}
