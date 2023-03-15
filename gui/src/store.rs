use askai_api::Topic;
use std::path::PathBuf;
use tokio::fs;

use serde_json::json;
use uuid::Uuid;

use crate::project::Project;
use crate::result::Result;
use crate::state::Chat;

pub struct Store {
    chat_data_dir: PathBuf,
    metadata_path: PathBuf,
    metadata: Metadata,
}

impl Store {
    pub async fn init() -> Result<Self> {
        let project = Project::default();
        if !project.data_dir.exists() {
            fs::create_dir_all(project.data_dir.as_path()).await?;
        }
        if !project.metadata_path.exists() {
            fs::File::create(project.metadata_path.as_path()).await?;
            fs::write(
                project.metadata_path.as_path(),
                json!({
                    "chats": []
                })
                .to_string(),
            )
            .await?;
        }
        let chat_data_dir = project.data_dir.join("data");
        if !chat_data_dir.exists() {
            fs::create_dir_all(chat_data_dir.as_path()).await?;
        }

        let metadata_content = fs::read_to_string(project.metadata_path.as_path()).await?;
        let metadata: Metadata = serde_json::from_str(&metadata_content).unwrap();

        Ok(Self {
            chat_data_dir,
            metadata_path: project.metadata_path,
            metadata,
        })
    }

    pub async fn create_chat(&mut self, topic: Option<String>, title: &str) -> Result<Chat> {
        let chat = Chat::new(topic, title);
        self.metadata
            .chats
            .insert(0, ChatMetadata::from_chat(&chat));

        // Update metadata
        self.write_metadata().await?;
        // Update chat data
        self.save_chat(&chat).await.unwrap();

        Ok(chat)
    }

    pub async fn save_chat(&self, chat: &Chat) -> Result<()> {
        let path = self.chat_save_path(chat.id, &chat.title);
        let chat_data = chat.topic_json_string().await;

        // Update chat data
        fs::write(&path, chat_data).await?;

        Ok(())
    }

    async fn write_metadata(&self) -> Result<()> {
        let metadata_string = serde_json::to_string(&self.metadata).unwrap();
        fs::write(&self.metadata_path, metadata_string).await?;

        Ok(())
    }

    pub async fn all_chats(&self) -> Result<Vec<ChatMetadata>> {
        let metadata_content = fs::read_to_string(&self.metadata_path).await?;
        let metadata: Metadata = serde_json::from_str(&metadata_content).unwrap();

        Ok(metadata.chats)
    }

    pub async fn read_chat(&self, chat_id: Uuid) -> Result<Chat> {
        let chat_metadata = self.chat_metadata(chat_id)?;
        let chat_data_path = self.chat_save_path(chat_metadata.id, &chat_metadata.title);
        let topic_json_string = fs::read_to_string(&chat_data_path).await?;
        let topic: Topic = serde_json::from_str(&topic_json_string).unwrap();
        let chat = Chat::from_topic(chat_metadata.id, &chat_metadata.title, topic);

        Ok(chat)
    }

    pub async fn delete_chat(&mut self, chat_id: Uuid) -> Result<()> {
        let chat_metadata = self.chat_metadata(chat_id)?;
        let chat_data_path = self.chat_save_path(chat_metadata.id, &chat_metadata.title);
        fs::remove_file(&chat_data_path).await?;

        self.metadata.chats.retain(|chat| chat.id != chat_id);
        self.write_metadata().await?;

        Ok(())
    }

    fn chat_metadata(&self, chat_id: Uuid) -> Result<&ChatMetadata> {
        let chat_metadata = self
            .metadata
            .chats
            .iter()
            .find(|chat| chat.id == chat_id)
            .unwrap();

        Ok(chat_metadata)
    }

    fn chat_save_path(&self, id: Uuid, title: &str) -> PathBuf {
        self.chat_data_dir.join(format!("{}_{}.json", id, title))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Metadata {
    chats: Vec<ChatMetadata>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ChatMetadata {
    id: Uuid,
    title: String,
}

impl ChatMetadata {
    pub fn from_chat(chat: &Chat) -> Self {
        Self {
            id: chat.id,
            title: chat.title.clone(),
        }
    }
}
