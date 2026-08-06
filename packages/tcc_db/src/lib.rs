//! TCC Launcher database layer.

use sqlx::SqlitePool;
use directories::ProjectDirs;
use tokio::fs;

pub async fn create_pool() -> sqlx::Result<SqlitePool> {
    let data_dir = ProjectDirs::from("com", "tcc", "launcher")
        .map(|dirs| dirs.data_dir().to_path_buf())
        .ok_or_else(|| sqlx::Error::Configuration("Data directory not found".into()))?;

    fs::create_dir_all(&data_dir).await?;
    let db_path = data_dir.join("launcher.db");

    SqlitePool::connect(&format!("sqlite://{}?mode=rwc", db_path.display())).await
}