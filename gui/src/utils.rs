use std::path::Path;

use tokio::fs;

use crate::result::Result;

pub async fn ensure_directory_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path).await?;
    }

    Ok(())
}

#[allow(unused)]
pub async fn ensure_file_exists(path: &Path, initial: impl Fn() -> String) -> Result<()> {
    if !path.exists() {
        let parent = path.parent().unwrap();
        ensure_directory_exists(parent).await?;

        fs::write(path, initial()).await?;
    }

    Ok(())
}

#[allow(unused)]
pub async fn save_file(path: &Path, data: &[u8]) -> Result<()> {
    fs::write(path, data).await?;

    Ok(())
}

pub fn save_file_sync(path: &Path, data: &[u8]) -> Result<()> {
    std::fs::write(path, data)?;

    Ok(())
}
