use serde::ser::SerializeMap;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Api(#[from] ApiError),

    #[error(transparent)]
    Database(#[from] diesel::result::Error),

    #[error(transparent)]
    Migration(#[from] diesel_migrations::MigrationError),

    #[error(transparent)]
    Network(NetworkError),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    ParseCsv(#[from] csv::Error),

    #[error("unknown error: {0}")]
    Unknown(String),
}

impl serde::ser::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Error::Api(err) => err.serialize(serializer),
            Error::Database(err) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "database")?;
                map.serialize_entry("message", &err.to_string())?;
                map.end()
            }
            Error::Migration(err) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "migration")?;
                map.serialize_entry("message", &err.to_string())?;
                map.end()
            }
            Error::Network(err) => err.serialize(serializer),
            Error::Serde(err) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "serde")?;
                map.serialize_entry("message", &err.to_string())?;
                map.end()
            }
            Error::ParseCsv(err) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "parse_csv")?;
                map.serialize_entry("message", &err.to_string())?;
                map.end()
            }
            Error::Unknown(err) => err.serialize(serializer),
        }
    }
}

#[derive(thiserror::Error, serde::Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "type", content = "message")]
pub enum ApiError {
    #[error("invalid api key")]
    InvalidKey,
    #[error("unknown error: {0}")]
    Unknown(String),
}

#[derive(thiserror::Error, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "type", content = "message")]
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
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "error")]
pub enum StreamError {
    #[error(transparent)]
    Api(ApiError),

    #[error(transparent)]
    Network(NetworkError),

    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<Error> for StreamError {
    fn from(err: Error) -> Self {
        match err {
            Error::Api(err) => StreamError::Api(err),
            Error::Network(err) => StreamError::Network(err),
            Error::Unknown(err) => StreamError::Unknown(err),
            _ => StreamError::Unknown(err.to_string()),
        }
    }
}
