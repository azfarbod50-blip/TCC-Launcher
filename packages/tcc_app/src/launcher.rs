//! Launcher module

use tcc_core::LauncherCore;

pub async fn create_launcher() -> anyhow::Result<LauncherCore> {
    LauncherCore::new().await
}

pub fn state() -> Option<&'static LauncherCore> {
    // Global state access - would be set during initialization
    None
}