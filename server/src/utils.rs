use std::path::Path;

use tokio::fs;

pub async fn ensure_directory_exists(path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path).await?;
    }

    Ok(())
}

#[allow(unused)]
pub async fn ensure_file_exists(path: &Path, initial: impl Fn() -> String) -> anyhow::Result<()> {
    if !path.exists() {
        let parent = path.parent().unwrap();
        ensure_directory_exists(parent).await?;

        fs::write(path, initial()).await?;
    }

    Ok(())
}
