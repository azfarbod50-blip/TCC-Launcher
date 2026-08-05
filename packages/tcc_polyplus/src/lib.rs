//! TCC Launcher PolyPlus integration.

pub struct PolyPlusClient;

impl PolyPlusClient {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn fetch_content(&self) -> Vec<String> {
        Vec::new()
    }
}

impl Default for PolyPlusClient {
    fn default() -> Self {
        Self::new()
    }
}