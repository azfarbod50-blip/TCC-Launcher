use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use directories::ProjectDirs;
use sqlx::SqlitePool;
use tokio::sync::Mutex;

use crate::data::{AccountKind, MinecraftAccount};
use crate::error::{AuthError, AuthResult};

/// SQLite-backed credentials store for offline accounts.
///
/// Accounts are persisted to `auth.db` in the launcher data directory.
pub struct CredentialsStore {
    pool: SqlitePool,
    default_user: Mutex<Option<Uuid>>,
}

impl CredentialsStore {
    /// Creates a new credentials store, initializing the database if needed.
    pub async fn new() -> AuthResult<Self> {
        let data_dir = Self::data_dir()?;
        tokio::fs::create_dir_all(&data_dir).await?;

        let db_path = data_dir.join("auth.db");
        let pool = SqlitePool::connect(&format!("sqlite://{}?mode=rwc", db_path.display())).await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        let default_user = Self::load_default_user(&pool).await?;

        Ok(Self {
            pool,
            default_user: Mutex::new(default_user),
        })
    }

    /// Gets the launcher data directory.
    fn data_dir() -> AuthResult<PathBuf> {
        ProjectDirs::from("com", "tcc", "launcher")
            .map(|dirs| dirs.data_dir().to_path_buf())
            .ok_or(AuthError::DataDirNotFound)
    }

    /// Loads the default user ID from the database.
    async fn load_default_user(pool: &SqlitePool) -> AuthResult<Option<Uuid>> {
        let row = sqlx::query!(
            "SELECT user_id FROM default_user LIMIT 1"
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| Uuid::from_bytes(r.user_id)))
    }

    /// Lists all accounts in the store.
    pub async fn list_accounts(&self) -> Vec<MinecraftAccount> {
        let rows = sqlx::query!(
            "SELECT id, username, access_token, refresh_token, expires, kind FROM accounts ORDER BY username"
        )
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        rows.into_iter()
            .map(|row| MinecraftAccount {
                id: Uuid::from_bytes(row.id),
                username: row.username,
                access_token: row.access_token.unwrap_or_default(),
                refresh_token: row.refresh_token.unwrap_or_default(),
                expires: chrono::DateTime::from_timestamp(row.expires, 0).unwrap_or_else(|| Utc::now()),
                kind: match row.kind.as_str() {
                    "offline" => AccountKind::Offline,
                    _ => AccountKind::Offline,
                },
            })
            .collect()
    }

    /// Gets a specific account by ID.
    pub async fn get_account(&self, id: Uuid) -> Option<MinecraftAccount> {
        let row = sqlx::query!(
            "SELECT id, username, access_token, refresh_token, expires, kind FROM accounts WHERE id = ?",
            id.as_bytes()
        )
        .fetch_optional(&self.pool)
        .await
        .ok()??;

        Some(MinecraftAccount {
            id: Uuid::from_bytes(row.id),
            username: row.username,
            access_token: row.access_token.unwrap_or_default(),
            refresh_token: row.refresh_token.unwrap_or_default(),
            expires: chrono::DateTime::from_timestamp(row.expires, 0).unwrap_or_else(|| Utc::now()),
            kind: match row.kind.as_str() {
                "offline" => AccountKind::Offline,
                _ => AccountKind::Offline,
            },
        })
    }

    /// Adds an offline account and saves it.
    pub async fn add_offline_account_and_save(&self, username: String) -> AuthResult<MinecraftAccount> {
        let account = super::offline::offline_account(username);
        self.add_account(&account).await?;
        Ok(account)
    }

    /// Adds an account to the database.
    async fn add_account(&self, account: &MinecraftAccount) -> AuthResult<()> {
        let kind_str = match account.kind {
            AccountKind::Offline => "offline",
            _ => "offline",
        };

        sqlx::query!(
            "INSERT INTO accounts (id, username, access_token, refresh_token, expires, kind) VALUES (?, ?, ?, ?, ?, ?)",
            account.id.as_bytes(),
            account.username,
            account.access_token,
            account.refresh_token,
            account.expires.timestamp(),
            kind_str
        )
        .execute(&self.pool)
        .await?;

        // If this is the first account, set it as default
        let count = sqlx::query!("SELECT COUNT(*) as cnt FROM accounts")
            .fetch_one(&self.pool)
            .await?
            .cnt;

        if count == 1 {
            self.set_default_user(Some(account.id)).await?;
        }

        Ok(())
    }

    /// Removes an account.
    pub async fn remove_account(&self, id: Uuid) -> AuthResult<()> {
        sqlx::query!("DELETE FROM accounts WHERE id = ?", id.as_bytes())
            .execute(&self.pool)
            .await?;

        // If we removed the default user, clear it
        let mut default = self.default_user.lock().await;
        if *default == Some(id) {
            *default = None;
            sqlx::query!("DELETE FROM default_user")
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    /// Sets the default user.
    pub async fn set_default_user(&self, id: Option<Uuid>) -> AuthResult<()> {
        let mut default = self.default_user.lock().await;
        *default = id;

        if let Some(id) = id {
            sqlx::query!(
                "INSERT OR REPLACE INTO default_user (user_id) VALUES (?)",
                id.as_bytes()
            )
            .execute(&self.pool)
            .await?;
        } else {
            sqlx::query!("DELETE FROM default_user")
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    /// Gets the default account.
    pub async fn default_account(&self) -> AuthResult<Option<MinecraftAccount>> {
        let default = self.default_user.lock().await;
        if let Some(id) = *default {
            return Ok(self.get_account(id).await);
        }
        Ok(None)
    }

    /// Resolves the default account ID.
    pub async fn resolve_default_id(&self) -> AuthResult<Option<Uuid>> {
        let default = self.default_user.lock().await;
        Ok(*default)
    }

    /// Commits an account (for compatibility with auth service interface).
    pub async fn commit_account(&self, account: MinecraftAccount, _events: &tcc_events::EventBus) -> AuthResult<()> {
        self.add_account(&account).await
    }

    /// Commits a refreshed account (no-op for offline accounts).
    pub async fn commit_refreshed_account(&self, account: MinecraftAccount) -> AuthResult<()> {
        self.add_account(&account).await
    }
}