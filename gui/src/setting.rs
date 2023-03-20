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
        let api_key = self.settings.api_key.as_deref().unwrap_or("");

        let mut api = OpenAIApi::new(api_key);
        if let Some(proxy) = &self.settings.proxy {
            api.set_proxy(proxy);
        }

        Ok(api)
    }

    // pub fn setting_path(&self) -> &PathBuf {
    //     &self.project.setting_path
    // }

    pub fn has_api_key(&self) -> bool {
        self.settings.api_key.is_some()
    }

    pub async fn set_api_key(&mut self, api_key: &str) -> Result<()> {
        self.settings.api_key = Some(api_key.to_string());
        self.save().await
    }

    pub fn get_proxy(&mut self) -> &Option<String> {
        &self.settings.proxy
    }

    pub async fn set_proxy(&mut self, proxy: &str) -> Result<()> {
        if proxy.is_empty() {
            self.clear_proxy().await?;
        } else {
            self.settings.proxy = Some(proxy.to_string());
            self.save().await?;
        }
        Ok(())
    }

    pub async fn clear_proxy(&mut self) -> Result<()> {
        self.settings.proxy = None;
        self.save().await
    }

    pub fn get_theme(&self) -> Theme {
        self.settings.theme.clone()
    }

    pub async fn set_theme(&mut self, theme: Theme) -> Result<()> {
        self.settings.theme = theme;
        self.save().await?;

        Ok(())
    }

    pub fn get_locale(&self) -> String {
        self.settings.locale.clone()
    }

    pub async fn set_locale(&mut self, locale: &str) -> Result<()> {
        self.settings.locale = locale.to_string();
        self.save().await?;

        Ok(())
    }

    async fn save(&self) -> Result<()> {
        let opts_string = serde_json::to_string(&self.settings).unwrap();
        fs::write(self.setting_path.as_path(), opts_string).await?;
        Ok(())
    }
}
