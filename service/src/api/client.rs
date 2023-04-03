use std::time::Duration;

use crate::result::Result;

#[derive(Default, Clone)]
pub struct Client {
    headers: Option<reqwest::header::HeaderMap>,
    proxy: Option<reqwest::Proxy>,
    timeout: Option<Duration>,
}

impl Client {
    pub fn new(timeout: Option<Duration>) -> Self {
        Self {
            timeout,
            ..Default::default()
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

        // set timeout
        if let Some(duration) = self.timeout {
            client_builder = client_builder.timeout(duration);
        }

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
