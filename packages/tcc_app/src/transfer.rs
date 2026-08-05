//! Transfer module

use std::path::PathBuf;

pub async fn download_file(url: &str, dest: PathBuf) -> anyhow::Result<()> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    std::fs::write(dest, bytes)?;
    Ok(())
}