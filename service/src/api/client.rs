use crate::result::Result;
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct Client {
    headers: Option<reqwest::header::HeaderMap>,
    proxy: Option<reqwest::Proxy>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            headers: None,
            proxy: None,
        }
    }

    pub fn headers(&mut self, headers: Option<reqwest::header::HeaderMap>) -> &mut Self {
        self.headers = headers;
        self
    }

    pub fn proxy(&mut self, proxy: Option<reqwest::Proxy>) -> &mut Self {
        self.proxy = proxy;
        self
    }

    fn build(&self) -> reqwest::Client {
        let mut client_builder = reqwest::Client::builder();

        // set headers
        if let Some(headers) = self.headers.clone() {
            client_builder = client_builder.default_headers(headers);
        }

        // set proxy
        if let Some(proxy) = self.proxy.clone() {
            client_builder = client_builder.no_proxy().proxy(proxy);
        }

        client_builder.build().unwrap()
    }

    pub async fn get(&self, url: &str) -> Result<reqwest::Response> {
        let client = self.build();

        let request = client.get(url);

        request.send().await.map_err(Into::into)
    }

    pub async fn post(&self, url: &str, data: impl serde::Serialize) -> Result<reqwest::Response> {
        let client = self.build();

        let data = serde_json::to_value(data).unwrap();

        let request = client.post(url).json(&data);

        request.send().await.map_err(Into::into)
    }
}
