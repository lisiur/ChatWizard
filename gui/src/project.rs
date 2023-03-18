use std::path::PathBuf;

use directories::ProjectDirs;

pub struct Project {
    pub setting_dir: PathBuf,
    pub setting_path: PathBuf,

    pub chat_data_dir: PathBuf,
    pub chat_metadata_path: PathBuf,

    pub prompt_data_dir: PathBuf,
    pub prompt_metadata_path: PathBuf,
}

impl Project {
    pub fn new() -> Self {
        let config_dir = ProjectDirs::from("com", "lisiur", "askai").unwrap();

        let setting_dir = config_dir.config_dir().to_path_buf();
        let setting_path = setting_dir.join("setting.json");

        let chat_data_dir = config_dir.data_dir().join("chat_data");
        let chat_metadata_path = config_dir.data_dir().join("chat_metadata.json");

        let prompt_data_dir = config_dir.data_dir().join("prompt_data");
        let prompt_metadata_path = config_dir.data_dir().join("prompt_metadata.json");

        Self {
            setting_dir,
            setting_path,
            chat_data_dir,
            chat_metadata_path,
            prompt_data_dir,
            prompt_metadata_path,
        }
    }
}
