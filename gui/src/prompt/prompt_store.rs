use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    project::Project,
    store::{Index, Store},
};

pub struct PromptStore {
    root_dir: std::path::PathBuf,
}

impl Default for PromptStore {
    fn default() -> Self {
        Self {
            root_dir: Project::prompt_root_dir(),
        }
    }
}

impl Store for PromptStore {
    type Index = PromptIndex;

    type Metadata = PromptMetadata;

    type Data = PromptData;

    fn root_dir(&self) -> std::path::PathBuf {
        self.root_dir.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PromptIndex {
    pub id: Uuid,
    pub act: String,
}

impl Index for PromptIndex {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PromptMetadata {
    pub act: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PromptData {
    pub prompt: String,
}
