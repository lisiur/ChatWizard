use std::path::PathBuf;

use directories::ProjectDirs;

pub struct Project {
    pub setting_path: PathBuf,
    pub chat_root: PathBuf,
    pub prompt_root: PathBuf,
}

impl Default for Project {
    fn default() -> Self {
        let config_dir = ProjectDirs::from("com", "lisiur", "askai").unwrap();

        let setting_dir = config_dir.config_dir().to_path_buf();
        let setting_path = setting_dir.join("setting.json");

        let chat_root = config_dir.data_dir().join("chats");
        let prompt_root = config_dir.data_dir().join("prompts");

        Self {
            setting_path,
            chat_root,
            prompt_root,
        }
    }
}

impl Project {
    pub fn setting_path() -> PathBuf {
        let config_dir = ProjectDirs::from("com", "lisiur", "askai").unwrap();
        config_dir.config_dir().join("setting.json")
    }

    pub fn chat_root_dir() -> PathBuf {
        let config_dir = ProjectDirs::from("com", "lisiur", "askai").unwrap();
        config_dir.data_dir().join("chats")
    }

    pub fn prompt_root_dir() -> PathBuf {
        let config_dir = ProjectDirs::from("com", "lisiur", "askai").unwrap();
        config_dir.data_dir().join("prompts")
    }
}
