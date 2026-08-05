//! System utilities.

use std::path::PathBuf;

pub fn get_os() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    }
}

pub fn get_arch() -> &'static str {
    if cfg!(target_arch = "x86_64") {
        "x64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        "unknown"
    }
}

pub fn get_cache_dir() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "tcc", "launcher")
        .map(|dirs| dirs.cache_dir().to_path_buf())
}