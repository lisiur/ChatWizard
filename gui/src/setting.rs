use std::path::{Path, PathBuf};

use crate::utils::ensure_file_exists;
use askai_api::OpenAIApi;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::fs;

use crate::result::Result;

#[derive(Debug)]
pub struct Setting {
    setting_path: PathBuf,
    pub settings: Settings,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub api_key: Option<String>,
    pub org_id: Option<String>,
    pub proxy: Option<String>,
    #[serde(default)]
    pub theme: Theme,
    #[serde(default = "default_locale")]
    pub locale: String,
    pub forward_url: Option<String>,
    pub forward_api_key: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SettingsUpdatePayload {
    pub api_key: Option<String>,
    pub org_id: Option<String>,
    pub proxy: Option<String>,
    pub theme: Option<Theme>,
    pub locale: Option<String>,
    pub forward_url: Option<String>,
    pub forward_api_key: Option<bool>,
}

fn default_locale() -> String {
    "enUS".to_string()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Default for Theme {
    fn default() -> Self {
        Self::System
    }
}

impl Setting {
    pub async fn init(setting_path: &Path) -> Result<Self> {
        ensure_file_exists(setting_path, || json!({}).to_string()).await?;

        let settings_string = fs::read_to_string(setting_path).await?;
        dbg!(&settings_string);

        let settings = serde_json::from_str::<Settings>(&settings_string).unwrap();
        dbg!(&settings);

        Ok(Setting {
            setting_path: setting_path.to_path_buf(),
            settings,
        })
    }

    pub async fn create_api(&self) -> Result<askai_api::OpenAIApi> {
        let mut api_key = self.settings.api_key.as_deref();
        let forward_url = self.settings.forward_url.as_deref();
        let forward_api_key = self.settings.forward_api_key.unwrap_or(false);

        if forward_url.is_some() && !forward_api_key {
            api_key = None;
        }

        let mut api = OpenAIApi::new(api_key);

        if let Some(host) = forward_url {
            api.set_host(host);
        }

        if let Some(proxy) = &self.settings.proxy {
            api.set_proxy(proxy);
        }

        Ok(api)
    }

    pub async fn update(&mut self, payload: &SettingsUpdatePayload) -> Result<()> {
        if let Some(api_key) = &payload.api_key {
            if api_key.is_empty() {
                self.settings.api_key = None;
            } else {
                self.settings.api_key = Some(api_key.to_string());
            }
        }

        if let Some(org_id) = &payload.org_id {
            self.settings.org_id = Some(org_id.to_string());
        }

        if let Some(proxy) = &payload.proxy {
            if proxy.is_empty() {
                self.settings.proxy = None;
            } else {
                self.settings.proxy = Some(proxy.to_string());
            }
        }

        if let Some(theme) = &payload.theme {
            self.settings.theme = theme.clone();
        }

        if let Some(locale) = &payload.locale {
            self.settings.locale = locale.to_string();
        }

        if let Some(forward_url) = &payload.forward_url {
            if forward_url.is_empty() {
                self.settings.forward_url = None;
            } else {
                self.settings.forward_url = Some(forward_url.to_string());
            }
        }

        if let Some(forward_api_key) = &payload.forward_api_key {
            self.settings.forward_api_key = Some(*forward_api_key);
        }

        self.save().await?;

        Ok(())
    }

    pub fn has_api_key(&self) -> bool {
        self.settings.api_key.is_some()
    }

    pub fn get_proxy(&mut self) -> &Option<String> {
        &self.settings.proxy
    }

    pub fn get_theme(&self) -> Theme {
        self.settings.theme.clone()
    }

    pub fn get_locale(&self) -> String {
        self.settings.locale.clone()
    }

    async fn save(&self) -> Result<()> {
        let opts_string = serde_json::to_string(&self.settings).unwrap();
        fs::write(self.setting_path.as_path(), opts_string).await?;
        Ok(())
    }
}
