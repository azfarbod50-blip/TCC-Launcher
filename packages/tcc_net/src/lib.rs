//! TCC Launcher networking.

use reqwest::Client;

#[derive(Clone)]
pub struct RequestClient {
    client: Client,
}

impl RequestClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn http(&self) -> &Client {
        &self.client
    }
}

impl Default for RequestClient {
    fn default() -> Self {
        Self::new()
    }
}