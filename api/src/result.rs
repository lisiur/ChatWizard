use crate::error::Error;
use axum::response::{Response, IntoResponse};
use reqwest::StatusCode;
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

pub trait IntoResultResponse {
    fn into_response(self) -> Response;
}

impl<T: Serialize> IntoResultResponse for Result<T> {
    fn into_response(self) -> Response {
        let status = match &self {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = match self {
            Ok(data) => serde_json::to_string(&data).unwrap(),
            Err(e) => serde_json::to_string(&e.to_string()).unwrap(),
        };

        (status, body).into_response()
    }
}
