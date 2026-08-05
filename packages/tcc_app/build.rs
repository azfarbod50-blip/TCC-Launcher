// build.rs - Windows resource embedding for TCC Launcher

#[cfg(target_os = "windows")]
fn main() {
    use std::path::Path;
    use winresource::WindowsResource;
    
    let mut res = WindowsResource::new();
    res.set("FileDescription", "TCC Launcher - Offline Minecraft Launcher")
       .set("ProductName", "TCC Launcher")
       .set("OriginalFilename", "tcc-launcher.exe")
       .set("InternalName", "tcc-launcher")
       .set("CompanyName", "TCC Launcher Contributors")
       .set("LegalCopyright", "© 2026 TCC Launcher Contributors")
       .set("LegalTrademarks", "")
       .set("Comments", "Offline Minecraft Launcher")
       .set_icon_with_id("assets/icons/icon.ico", "MAINICON");
    
    if let Err(e) = res.compile() {
        eprintln!("Warning: Failed to embed Windows resources: {}", e);
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {
    // No-op on non-Windows
}