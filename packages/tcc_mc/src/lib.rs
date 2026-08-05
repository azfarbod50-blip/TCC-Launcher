//! TCC Launcher Minecraft utilities.

use std::path::PathBuf;

pub struct MinecraftVersion {
    pub id: String,
    pub path: PathBuf,
}

impl MinecraftVersion {
    pub fn new(id: String, path: PathBuf) -> Self {
        Self { id, path }
    }
}

pub async fn get_minecraft_versions() -> Vec<MinecraftVersion> {
    Vec::new()
}