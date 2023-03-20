use crate::{error::ApiErrorResponse, result::Result};
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct ClientOpts {
    pub headers: Option<reqwest::header::HeaderMap>,
    pub proxy: Option<reqwest::Proxy>,
}

pub struct Client {
    opt: ClientOpts,
    client: reqwest::Client,
}

impl Client {
    pub fn new(opt: ClientOpts) -> Self {
        let client = Self::init_client(&opt);
        Self { opt, client }
    }

    fn init_client(opt: &ClientOpts) -> reqwest::Client {
        let mut client_builder = reqwest::Client::builder();

        // set headers
        if let Some(headers) = opt.headers.clone() {
            client_builder = client_builder.default_headers(headers);
        }

        // set proxy
        if let Some(proxy) = opt.proxy.clone() {
            client_builder = client_builder.no_proxy().proxy(proxy);
        }

        client_builder.build().unwrap()
    }

    pub fn set_proxy(&mut self, proxy: reqwest::Proxy) {
        let client = Self::init_client(&ClientOpts {
            headers: self.opt.headers.clone(),
            proxy: Some(proxy),
        });
        self.client = client;
    }

    pub fn clear_proxy(&mut self) {
        let client = Self::init_client(&ClientOpts {
            headers: self.opt.headers.clone(),
            proxy: None,
        });
        self.client = client;
    }

    pub async fn get(
        &self,
        url: &str,
        _params: Option<serde_json::Value>,
    ) -> Result<reqwest::Response> {
        let request = self.client.get(url);

        request.send().await.map_err(Into::into)
    }

    pub async fn post(&self, url: &str, data: serde_json::Value) -> Result<reqwest::Response> {
        let request = self.client.post(url).json(&data);

        request.send().await.map_err(Into::into)
    }

    pub async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let res_data = self
            .get(url, None)
            .await?
            .json::<serde_json::Value>()
            .await?;

        log::debug!("response data: {}", res_data);

        let res_data = serde_json::from_value::<OpenAIApiResponse<T>>(res_data).unwrap();
        match res_data {
            OpenAIApiResponse::Ok(data) => Ok(data),
            OpenAIApiResponse::Err(err) => Err(err.into()),
        }
    }

    pub async fn post_json<T: DeserializeOwned>(
        &self,
        url: &str,
        data: impl serde::Serialize,
    ) -> Result<T> {
        let data = serde_json::to_value(data).unwrap();

        log::debug!("request params: {}", data);

        let res_data = self
            .post(url, data)
            .await?
            .json::<serde_json::Value>()
            .await?;

        log::debug!("response data: {}", res_data);

        let res_data = serde_json::from_value::<OpenAIApiResponse<T>>(res_data).unwrap();
        match res_data {
            OpenAIApiResponse::Ok(data) => Ok(data),
            OpenAIApiResponse::Err(err) => Err(err.into()),
        }
    }

    pub async fn post_stream<T: DeserializeOwned>(
        &self,
        url: &str,
        data: impl serde::Serialize,
    ) -> Result<reqwest::Response> {
        let data = serde_json::to_value(data).unwrap();

        log::debug!("request params: {}", data);

        self.post(url, data).await
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum OpenAIApiResponse<T> {
    Ok(T),
    Err(ApiErrorResponse),
}
