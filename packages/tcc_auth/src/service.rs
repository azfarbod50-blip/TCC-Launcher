use std::sync::Arc;
use std::collections::HashMap;

use tcc_events::EventBus;
use tcc_net::RequestClient;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::data::{MinecraftAccount};
use crate::error::{AuthError, AuthResult};
use crate::offline::{validate_offline_username};
use crate::store::CredentialsStore;

/// Owns everything the authentication flows need.
///
/// Constructed once by the composition layer and passed down, so nothing here
/// reaches for a global. No database; accounts live in `auth.db` beside the
/// launcher data.
pub struct AuthService {
    store: Mutex<CredentialsStore>,
    /// Serialises token renewal per account.
    refresh_guards: std::sync::Mutex<HashMap<Uuid, Arc<Mutex<()>>>>,
    net: RequestClient,
    events: EventBus,
}

impl AuthService {
    /// Loads the credentials store from disk. A missing or unreadable file
    /// yields an empty store rather than failing: a corrupt `auth.db` should
    /// mean "sign in again", not "the launcher will not start".
    pub async fn load(net: RequestClient, events: EventBus) -> AuthResult<Self> {
        Ok(Self::with_store(CredentialsStore::new().await?, net, events))
    }

    #[must_use]
    pub fn with_store(store: CredentialsStore, net: RequestClient, events: EventBus) -> Self {
        Self {
            store: Mutex::new(store),
            refresh_guards: std::sync::Mutex::new(HashMap::new()),
            net,
            events,
        }
    }

    // --- account management ------------------------------------------------

    #[tracing::instrument(skip(self), fields(username = %username))]
    pub async fn add_offline_account(&self, username: String) -> AuthResult<MinecraftAccount> {
        validate_offline_username(&username)?;
        self.store
            .lock()
            .await
            .add_offline_account_and_save(username)
            .await
    }

    pub async fn list_accounts(&self) -> Vec<MinecraftAccount> {
        self.store.lock().await.list_accounts().await
    }

    pub async fn get_account(&self, id: Uuid) -> Option<MinecraftAccount> {
        self.store.lock().await.get_account(id).await
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn default_account(&self) -> AuthResult<Option<MinecraftAccount>> {
        self.store.lock().await.default_account().await
    }

    #[tracing::instrument(level = "debug", skip(self), fields(?id))]
    pub async fn set_default_account(&self, id: Option<Uuid>) -> AuthResult<()> {
        self.store.lock().await.set_default_user(id).await
    }

    #[tracing::instrument(skip(self), fields(%id))]
    pub async fn remove_account(&self, id: Uuid) -> AuthResult<()> {
        self.store.lock().await.remove_account(id).await?;
        Ok(())
    }

    // --- token renewal -----------------------------------------------------

    fn refresh_guard(&self, id: Uuid) -> Arc<Mutex<()>> {
        Arc::clone(
            self.refresh_guards
                .lock()
                .expect("refresh guard registry poisoned")
                .entry(id)
                .or_default(),
        )
    }

    /// Clones an account out of the store, so no caller holds the store lock
    /// past this call.
    async fn account_snapshot(&self, id: Uuid) -> AuthResult<MinecraftAccount> {
        self.store
            .lock()
            .await
            .get_account(id)
            .await
            .ok_or(AuthError::AccountNotFound(id))
    }

    /// Renews `id`'s access token if it has lapsed, returning a usable account.
    ///
    /// For offline accounts, this is a no-op since they don't have tokens.
    #[tracing::instrument(level = "debug", skip(self), fields(%id))]
    async fn renew_token(&self, id: Uuid, _force: bool) -> AuthResult<MinecraftAccount> {
        let existing = self.account_snapshot(id).await?;
        // Offline accounts don't expire
        Ok(existing)
    }

    #[tracing::instrument(level = "debug", skip(self), fields(%id))]
    pub async fn refresh_account(&self, id: Uuid) -> AuthResult<MinecraftAccount> {
        self.renew_token(id, true).await
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn refresh_all_accounts(&self) -> AuthResult<Vec<MinecraftAccount>> {
        let accounts = self.store.lock().await.list_accounts().await;
        Ok(accounts)
    }

    /// An account with a token good enough to launch with, renewing if needed.
    #[tracing::instrument(level = "debug", skip(self), fields(%id))]
    pub async fn account_for_launch(&self, id: Uuid) -> AuthResult<MinecraftAccount> {
        let account = self.renew_token(id, false).await?;
        Ok(account)
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn default_account_for_launch(&self) -> AuthResult<Option<MinecraftAccount>> {
        let Some(id) = self.store.lock().await.resolve_default_id().await? else {
            return Ok(None);
        };
        Ok(Some(self.account_for_launch(id).await?))
    }
}