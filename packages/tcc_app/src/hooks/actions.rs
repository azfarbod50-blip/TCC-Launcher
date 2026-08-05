//! Actions for TCC Launcher

use freya::prelude::*;
use freya::query::Mutation;
use crate::routes::Route;
use tcc_auth::MinecraftAccount;
use uuid::Uuid;

pub struct Actions {
    pub router: freya::router::RouterContext,
    // Add more actions as needed
}

impl Actions {
    pub fn new() -> Self {
        Self {
            router: freya::router::RouterContext::get(),
        }
    }

    pub fn navigate(&self, route: Route) {
        let _ = self.router.push(route);
    }

    pub fn close_account_switcher(&self) {
        // Implementation for closing account switcher
    }
}

impl Default for Actions {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NotificationBuilder {
    title: String,
    body: String,
}

impl NotificationBuilder {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
        }
    }
}

pub struct PumpSignal;