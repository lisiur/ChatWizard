use std::path::PathBuf;

use directories::ProjectDirs;

pub struct Project {
    pub setting_dir: PathBuf,
    pub setting_path: PathBuf,
    pub data_dir: PathBuf,
    pub metadata_path: PathBuf,
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}

impl Project {
    pub fn new() -> Self {
        let config_dir = ProjectDirs::from("com", "lisiur", "askai").unwrap();

        let setting_dir = config_dir.config_dir().to_path_buf();
        let setting_path = setting_dir.join("setting.toml");

        let data_dir = config_dir.data_dir().to_path_buf();
        let metadata_path = data_dir.join("metadata.json");

        Self {
            setting_dir,
            setting_path,
            data_dir,
            metadata_path,
        }
    }
}
