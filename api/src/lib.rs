mod chat;
mod client;
mod completion;
mod error;
mod image;
mod result;
mod types;

pub use chat::{StreamContent, Topic};
use client::{Client, ClientOpts};
pub use error::Error;
pub use result::Result;

pub struct OpenAIApi {
    client: Client,
}

impl OpenAIApi {
    pub fn new(api_key: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {api_key}").parse().unwrap(),
        );

        let client = Client::new(ClientOpts {
            headers: Some(headers),
            proxy: None,
        });

        Self { client }
    }

    pub async fn set_proxy(&self, proxy: &str) {
        if proxy.is_empty() {
            self.client.clear_proxy().await;
        } else {
            self.client
                .set_proxy(reqwest::Proxy::all(proxy).unwrap())
                .await;
        }
    }

    pub async fn clear_proxy(&self) {
        self.client.clear_proxy().await;
    }
}
