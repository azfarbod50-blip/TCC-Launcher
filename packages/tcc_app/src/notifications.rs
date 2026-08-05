//! Notifications system

use freya::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Notification {
    pub id: uuid::Uuid,
    pub title: String,
    pub body: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct NotificationCenter {
    notifications: VecDeque<Notification>,
    center_open: bool,
}

impl NotificationCenter {
    pub fn new() -> Self {
        Self {
            notifications: VecDeque::new(),
            center_open: false,
        }
    }

    pub fn add(&mut self, notification: Notification) {
        self.notifications.push_back(notification);
    }

    pub fn snapshot(&self, _inbox: &(), _center_open: bool, _prompt_view: ()) -> NotificationSnapshot {
        NotificationSnapshot {
            notifications: self.notifications.clone(),
            center_open: self.center_open,
            prompt_view: None,
        }
    }
}

#[derive(Clone)]
pub struct NotificationSnapshot {
    pub notifications: VecDeque<Notification>,
    pub center_open: bool,
    pub prompt_view: Option<()>,
}

impl Default for NotificationCenter {
    fn default() -> Self {
        Self::new()
    }
}