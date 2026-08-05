//! TCC Launcher core logic.

use tcc_auth::AuthService;
use tcc_events::EventBus;
use tcc_net::RequestClient;

pub struct LauncherCore {
    pub auth: AuthService,
    pub events: EventBus,
}

impl LauncherCore {
    pub async fn new() -> anyhow::Result<Self> {
        let net = RequestClient::new();
        let events = EventBus::new();
        let auth = AuthService::load(net, events.clone()).await?;
        
        Ok(Self { auth, events })
    }
}