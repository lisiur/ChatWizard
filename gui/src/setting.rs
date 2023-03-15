use crate::project::Project;
use askai_api::OpenAIApi;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::result::Result;

pub struct Setting {
    project: Project,
    pub opts: Opts,
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
    pub fn init() -> Result<Self> {
        let project = Project::default();
        if !project.setting_dir.exists() {
            fs::create_dir_all(project.setting_dir.as_path())?;
        }
        if !project.setting_path.exists() {
            fs::File::create(project.setting_path.as_path())?;
        }

        let opts_string = fs::read_to_string(project.setting_path.as_path())?;

        let opts: Opts = toml::from_str(&opts_string).unwrap();

        Ok(Setting { project, opts })
    }

    pub fn create_api(&self) -> Result<askai_api::OpenAIApi> {
        let setting = Setting::init()?;
        let api_key = setting.opts.api_key.as_deref().unwrap_or("");

        let mut api = OpenAIApi::new(api_key);
        if let Some(proxy) = &setting.opts.proxy {
            api.set_proxy(proxy);
        }

        Ok(api)
    }

    // pub fn setting_path(&self) -> &PathBuf {
    //     &self.project.setting_path
    // }

    pub fn set_api_key(&mut self, api_key: &str) -> Result<()> {
        self.opts.api_key = Some(api_key.to_string());
        self.save()
    }

    pub fn set_proxy(&mut self, proxy: &str) -> Result<()> {
        if proxy.is_empty() {
            self.clear_proxy()?;
        } else {
            self.opts.proxy = Some(proxy.to_string());
            self.save()?;
        }
        Ok(())
    }

    pub fn clear_proxy(&mut self) -> Result<()> {
        self.opts.proxy = None;
        self.save()
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

    fn save(&self) -> Result<()> {
        let opts_string = toml::to_string(&self.opts).unwrap();
        fs::write(self.project.setting_path.as_path(), opts_string)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Setting::init().is_ok());
    }
}
