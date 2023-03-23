mod chat;
mod client;
mod completion;
mod error;
mod image;
mod models;
mod result;
mod types;

pub use chat::{Chat, ChatParams, Message, Role, StreamContent};
use client::{Client, ClientOpts};
use error::ApiErrorResponse;
pub use error::Error;
pub use result::Result;

pub struct OpenAIApi {
    client: Client,
    host: String,
}

impl OpenAIApi {
    pub fn new(api_key: Option<&str>) -> Self {
        Self {
            client: Self::create_client(api_key, None),
            host: "https://api.openai.com".to_string(),
        }
    }

    fn create_client(api_key: Option<&str>, proxy: Option<&str>) -> Client {
        let mut headers = reqwest::header::HeaderMap::new();

        if let Some(api_key) = api_key {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {api_key}").parse().unwrap(),
            );
        }

        let proxy = proxy.map(|item| reqwest::Proxy::all(item).unwrap());

        Client::new(ClientOpts {
            headers: Some(headers),
            proxy,
        })
    }

    pub fn set_host(&mut self, host: &str) {
        self.host = host.to_string();
    }

    pub fn set_proxy(&mut self, proxy: &str) {
        if proxy.is_empty() {
            self.client.clear_proxy();
        } else {
            self.client.set_proxy(reqwest::Proxy::all(proxy).unwrap());
        }
    }

    pub async fn clear_proxy(&mut self) {
        self.client.clear_proxy();
    }

    pub async fn check_api_key(&self, api_key: &str) -> Result<()> {
        let client = Self::create_client(Some(api_key), None);
        let response = client
            .get(&format!("{}/v1/models", self.host), None)
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error: ApiErrorResponse = response.json().await?;
            Err(error.into())
        }
    }
}
