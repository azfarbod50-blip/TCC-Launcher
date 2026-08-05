//! TCC Launcher Java management.

use std::path::PathBuf;

pub struct JavaRuntime {
    pub path: PathBuf,
    pub version: String,
}

impl JavaRuntime {
    pub fn new(path: PathBuf, version: String) -> Self {
        Self { path, version }
    }
}

pub async fn detect_java_runtimes() -> Vec<JavaRuntime> {
    Vec::new()
}