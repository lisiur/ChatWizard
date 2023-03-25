use crate::{
    chat::{ChatConfig, ChatLog},
    project::Project,
    store::{Index, Store},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
pub struct ChatStore {
    root_dir: std::path::PathBuf,
}

impl Default for ChatStore {
    fn default() -> Self {
        Self {
            root_dir: Project::default().chat_root_dir(),
        }
    }
}

impl Store for ChatStore {
    type Index = ChatIndex;
    type Metadata = ChatMetadata;
    type Data = ChatData;

    fn root_dir(&self) -> std::path::PathBuf {
        self.root_dir.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatIndex {
    pub id: Uuid,
    pub title: String,
}

impl Index for ChatIndex {
    fn id(&self) -> uuid::Uuid {
        self.id
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatMetadata {
    pub title: String,
    pub config: ChatConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatData {
    pub cost: f32,
    pub logs: Vec<ChatLog>,
}
