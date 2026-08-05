//! Events system for TCC Launcher

use freya::prelude::*;
use tcc_events::EventBus;

pub fn use_events() -> EventBus {
    EventBus::new()
}