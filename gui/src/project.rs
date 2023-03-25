use std::path::PathBuf;

use directories::ProjectDirs;

pub struct Project {
    config_dir: PathBuf,
    data_dir: PathBuf,
}

impl Default for Project {
    fn default() -> Self {
        let project_dirs = ProjectDirs::from("com", "lisiur", "askai").unwrap();
        Self {
            config_dir: project_dirs.config_dir().to_path_buf(),
            data_dir: project_dirs.data_dir().to_path_buf(),
        }
    }
}

impl Project {
    pub fn setting_path(&self) -> PathBuf {
        self.config_dir.join("setting.json")
    }

    pub fn chat_root_dir(&self) -> PathBuf {
        self.data_dir.join("chats")
    }

    pub fn prompt_root_dir(&self) -> PathBuf {
        self.data_dir.join("prompts")
    }
}
