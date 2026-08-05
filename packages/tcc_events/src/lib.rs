//! TCC Launcher event system.

use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

/// Event bus for communication between components.
#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<Event>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1024);
        Self { sender }
    }

    pub fn send(&self, event: Event) {
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }

    pub fn progress(&self, id: Uuid, label: &str, current: u64, total: u64) {
        self.send(Event::Progress {
            id,
            label: label.to_string(),
            current,
            total,
        });
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Progress {
        id: Uuid,
        label: String,
        current: u64,
        total: u64,
    },
    // Add more events as needed
}