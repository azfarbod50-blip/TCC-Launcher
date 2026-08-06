//! TCC Launcher authentication and credentials store (offline only).
//!
//! The whole surface is [`AuthService`], which owns the credentials store and
//! the per-account refresh guards. Construct one in the composition layer and
//! pass it down; nothing in here reaches for a global or a database. Accounts
//! are persisted to `auth.db`.

mod data;
mod error;
mod offline;
mod service;
mod store;

pub use data::{AccountKind, MinecraftAccount};
pub use error::{AuthError, AuthResult, MinecraftAuthError, MinecraftAuthStep, AuthErrorGuidance};
pub use offline::{offline_account, offline_uuid, validate_offline_username};
pub use service::{AuthService};
pub use store::CredentialsStore;