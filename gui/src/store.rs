use std::path::{Path, PathBuf};

use json::json;
use tokio::fs;
use uuid::Uuid;

use crate::{
    result::Result,
    utils::{ensure_directory_exists, ensure_file_exists},
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json as json;

async fn read_file<T>(path: &Path) -> Result<T>
where
    T: DeserializeOwned + Send + Sync,
{
    let content = fs::read_to_string(path).await?;

    let value = json::from_str(&content).unwrap();

    Ok(value)
}

async fn write_file<T>(path: &Path, value: &T) -> Result<()>
where
    T: Serialize + Send + Sync,
{
    let content = json::to_string(value).unwrap();

    fs::write(path, content).await?;

    Ok(())
}

pub trait Index {
    fn id(&self) -> Uuid;
}

#[async_trait::async_trait]
pub trait Store: Send + Sync + Sized + Default {
    type Index: Index + Serialize + DeserializeOwned + Send + Sync;
    type Metadata: Serialize + DeserializeOwned + Send + Sync;
    type Data: Serialize + DeserializeOwned + Send + Sync;

    fn root_dir(&self) -> PathBuf;

    async fn init() -> Result<Self> {
        let store = Self::default();

        ensure_file_exists(&store.index_path(), || json!([]).to_string()).await?;
        ensure_directory_exists(&store.metadata_dir()).await?;
        ensure_directory_exists(&store.data_dir()).await?;

        Ok(store)
    }

    async fn read_index_list(&self) -> Result<Vec<Self::Index>> {
        read_file(&self.index_path()).await
    }

    async fn read_metadata(&self, id: &Uuid) -> Result<Self::Metadata> {
        read_file(&self.metadata_path(id)).await
    }

    async fn read_data(&self, id: &Uuid) -> Result<Self::Data> {
        read_file(&self.data_path(id)).await
    }

    async fn read(&self, id: &Uuid) -> Result<(Self::Metadata, Self::Data)> {
        let metadata = self.read_metadata(id).await?;
        let data = self.read_data(id).await?;

        Ok((metadata, data))
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        let metadata_path = self.metadata_path(id);
        let data_path = self.data_path(id);

        fs::remove_file(&metadata_path).await?;
        fs::remove_file(&data_path).await?;

        Ok(())
    }

    fn index_path(&self) -> PathBuf {
        Self::root_dir(self).join("index.json")
    }

    fn metadata_dir(&self) -> PathBuf {
        Self::root_dir(self).join("metadata")
    }

    fn data_dir(&self) -> PathBuf {
        Self::root_dir(self).join("data")
    }

    fn metadata_path(&self, id: &Uuid) -> PathBuf {
        Self::metadata_dir(self).join(format!("{}.json", id))
    }

    fn data_path(&self, id: &Uuid) -> PathBuf {
        self.data_dir().join(format!("{}.json", id))
    }

    async fn write_index(&self, index_list: &Vec<Self::Index>) -> Result<()> {
        write_file(&self.index_path(), index_list).await
    }

    async fn write_metadata(&self, id: &Uuid, metadata: &Self::Metadata) -> Result<()> {
        write_file(&self.metadata_path(id), metadata).await
    }

    async fn write_data(&self, id: &Uuid, data: &Self::Data) -> Result<()> {
        write_file(&self.data_path(id), data).await
    }
}
