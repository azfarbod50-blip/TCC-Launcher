use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The kind of Minecraft account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountKind {
    /// Offline/local account (no Microsoft authentication)
    Offline,
}

/// A Minecraft account usable for launching the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftAccount {
    /// Unique identifier for this account.
    pub id: Uuid,
    /// The player's username.
    pub username: String,
    /// Access token (empty for offline accounts).
    pub access_token: String,
    /// Refresh token (empty for offline accounts).
    pub refresh_token: String,
    /// When the access token expires.
    pub expires: DateTime<Utc>,
    /// The kind of account.
    pub kind: AccountKind,
}

impl MinecraftAccount {
    /// Returns true if this is an offline account (always true in TCC).
    pub fn is_offline(&self) -> bool {
        matches!(self.kind, AccountKind::Offline)
    }

    /// Returns true if the access token has expired.
    pub fn is_expired(&self) -> bool {
        self.expires <= Utc::now()
    }
}