use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::Duration;

use crate::api::client::Client;
use crate::api::openai::chat::OpenAIChatApi;
use crate::schema::settings;
use crate::types::{Id, TextWrapper};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Setting {
    pub id: Id,
    pub user_id: Id,
    pub language: String,
    pub theme: TextWrapper<Theme>,
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub forward_url: Option<String>,
    pub forward_api_key: bool,
}

impl Setting {
    pub fn api_key(&self) -> Option<&str> {
        self.api_key
            .as_deref()
            .and_then(|inner| if inner.is_empty() { None } else { Some(inner) })
    }

    pub fn proxy(&self) -> Option<&str> {
        self.proxy
            .as_deref()
            .and_then(|inner| if inner.is_empty() { None } else { Some(inner) })
    }

    pub fn forward_url(&self) -> Option<&str> {
        self.forward_url
            .as_deref()
            .and_then(|inner| if inner.is_empty() { None } else { Some(inner) })
    }

    pub fn create_client(&self, timeout: Option<Duration>) -> Client {
        let proxy = self.proxy().map(|item| reqwest::Proxy::all(item).unwrap());

        let mut client = Client::new(timeout);
        client.proxy(proxy);

        client
    }

    pub fn create_openai_chat(&self) -> OpenAIChatApi {
        let mut headers = reqwest::header::HeaderMap::new();

        if let Some(api_key) = self.api_key() {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {api_key}").parse().unwrap(),
            );
        }
        if self.forward_url().is_some() && !self.forward_api_key {
            headers.remove(reqwest::header::AUTHORIZATION);
        }

        let proxy = self.proxy().map(|item| reqwest::Proxy::all(item).unwrap());

        let mut client = Client::new(None);
        client.headers(Some(headers));
        client.proxy(proxy);

        let host = self.forward_url().unwrap_or("https://api.openai.com");

        OpenAIChatApi::new(client, host)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Theme {
    System,
    Light,
    Dark,
}

impl AsRef<str> for Theme {
    fn as_ref(&self) -> &str {
        match self {
            Theme::System => "system",
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

impl FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "system" => Ok(Theme::System),
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            _ => Err("Invalid theme".into()),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = settings)]
pub struct NewSetting {
    pub id: Id,
    pub user_id: Id,
    pub language: String,
    pub theme: TextWrapper<Theme>,
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub forward_url: Option<String>,
    pub forward_api_key: bool,
}

#[derive(AsChangeset, Deserialize, Default)]
#[diesel(table_name = settings)]
#[serde(rename_all = "camelCase")]
pub struct PatchSetting {
    pub user_id: Id,
    pub language: Option<String>,
    pub theme: Option<TextWrapper<Theme>>,
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub forward_url: Option<String>,
    pub forward_api_key: Option<bool>,
}
