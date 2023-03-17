use std::path::{Path, PathBuf};

use crate::utils::ensure_file_exists;
use askai_api::OpenAIApi;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::result::Result;

pub struct Setting {
    setting_path: PathBuf,
    opts: Opts,
}

#[derive(Serialize, Deserialize)]
pub struct Opts {
    pub api_key: Option<String>,
    pub org_id: Option<String>,
    pub proxy: Option<String>,
    pub theme: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Setting {
    pub async fn init(setting_path: &Path) -> Result<Self> {
        ensure_file_exists(setting_path, String::new).await?;

        let opts_string = fs::read_to_string(setting_path).await?;

        let opts: Opts = toml::from_str(&opts_string).unwrap();

        Ok(Setting {
            setting_path: setting_path.to_path_buf(),
            opts,
        })
    }

    pub async fn create_api(&self) -> Result<askai_api::OpenAIApi> {
        let api_key = self.opts.api_key.as_deref().unwrap_or("");

        let mut api = OpenAIApi::new(api_key);
        if let Some(proxy) = &self.opts.proxy {
            api.set_proxy(proxy);
        }

        Ok(api)
    }

    // pub fn setting_path(&self) -> &PathBuf {
    //     &self.project.setting_path
    // }

    pub fn has_api_key(&self) -> bool {
        self.opts.api_key.is_some()
    }

    pub async fn set_api_key(&mut self, api_key: &str) -> Result<()> {
        self.opts.api_key = Some(api_key.to_string());
        self.save().await
    }

    pub fn get_proxy(&mut self) -> &Option<String> {
        &self.opts.proxy
    }

    pub async fn set_proxy(&mut self, proxy: &str) -> Result<()> {
        if proxy.is_empty() {
            self.clear_proxy().await?;
        } else {
            self.opts.proxy = Some(proxy.to_string());
            self.save().await?;
        }
        Ok(())
    }

    pub async fn clear_proxy(&mut self) -> Result<()> {
        self.opts.proxy = None;
        self.save().await
    }

    // pub fn get_theme(&self) -> Theme {
    //     match self.opts.theme.as_deref() {
    //         Some("light") => Theme::Light,
    //         Some("dark") => Theme::Dark,
    //         Some("system") => Theme::System,
    //         _ => Theme::System,
    //     }
    // }

    // pub fn set_theme(&mut self, theme: &str) -> Result<()> {
    //     self.opts.theme = Some(theme.to_string());
    //     self.save()
    // }

    async fn save(&self) -> Result<()> {
        let opts_string = toml::to_string(&self.opts).unwrap();
        fs::write(self.setting_path.as_path(), opts_string).await?;
        Ok(())
    }
}
