#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Api(#[from] ApiError),

    #[error(transparent)]
    Sql(#[from] diesel::result::Error),

    #[error(transparent)]
    Migration(#[from] diesel_migrations::MigrationError),

    #[error(transparent)]
    Network(NetworkError),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error("unknown error: {0}")]
    Unknown(String),
}

#[derive(thiserror::Error, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug)]
pub enum ApiError {
    #[error("invalid api key")]
    InvalidKey,
    #[error("unknown error: {0}")]
    Unknown(String),
}

#[derive(thiserror::Error, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug)]
pub enum NetworkError {
    #[error("timeout: {0}")]
    Timeout(String),
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Network(if err.is_timeout() {
            NetworkError::Timeout(err.to_string())
        } else {
            NetworkError::Unknown(err.to_string())
        })
    }
}

impl From<crate::api::openai::response::OpenAIErrorResponse> for Error {
    fn from(err: crate::api::openai::response::OpenAIErrorResponse) -> Self {
        match err.error.code.as_deref() {
            Some("invalid_api_key") => Error::Api(ApiError::InvalidKey),
            _ => Error::Api(ApiError::Unknown(err.error.message)),
        }
    }
}

#[derive(thiserror::Error, serde::Serialize, Clone, Debug)]
pub enum StreamError {
    #[error(transparent)]
    Api(ApiError),

    #[error(transparent)]
    Network(NetworkError),

    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<reqwest::Error> for StreamError {
    fn from(err: reqwest::Error) -> Self {
        StreamError::Network(if err.is_timeout() {
            NetworkError::Timeout(err.to_string())
        } else {
            NetworkError::Unknown(err.to_string())
        })
    }
}
