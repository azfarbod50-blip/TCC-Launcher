use thiserror::Error;

/// Errors that can occur during authentication.
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("account not found: {0}")]
    AccountNotFound(uuid::Uuid),

    #[error("invalid offline username: {reason}")]
    InvalidOfflineUsername { reason: String },

    #[error("data directory not found")]
    DataDirNotFound,

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Minecraft authentication error: {0}")]
    Minecraft(#[from] MinecraftAuthError),
}

/// Result type for authentication operations.
pub type AuthResult<T> = Result<T, AuthError>;

/// Errors specific to Minecraft authentication (kept for compatibility).
#[derive(Debug, Error)]
pub enum MinecraftAuthError {
    #[error("browser login not found")]
    BrowserLoginNotFound,

    #[error("device code login failed")]
    DeviceCodeLoginFailed,

    #[error("token refresh failed")]
    TokenRefreshFailed,

    #[error("offline account requires Microsoft account")]
    OfflineRequiresMicrosoft,
}

/// Authentication step for progress reporting (kept for compatibility).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinecraftAuthStep {
    BrowserLogin,
    DeviceCode,
    TokenExchange,
    ProfileFetch,
}

/// Guidance for fixing authentication errors (kept for compatibility).
#[derive(Debug, Clone)]
pub struct AuthErrorGuidance {
    pub what_happened: String,
    pub steps_to_fix: Vec<String>,
}

impl MinecraftAuthError {
    pub fn auth_guidance(&self) -> Option<AuthErrorGuidance> {
        match self {
            MinecraftAuthError::BrowserLoginNotFound => Some(AuthErrorGuidance {
                what_happened: "The browser login session was not found.".to_string(),
                steps_to_fix: vec!["Try starting the login process again.".to_string()],
            }),
            MinecraftAuthError::DeviceCodeLoginFailed => Some(AuthErrorGuidance {
                what_happened: "The device code login failed.".to_string(),
                steps_to_fix: vec!["Check your internet connection and try again.".to_string()],
            }),
            MinecraftAuthError::TokenRefreshFailed => Some(AuthErrorGuidance {
                what_happened: "Failed to refresh the access token.".to_string(),
                steps_to_fix: vec!["Try removing and re-adding the account.".to_string()],
            }),
            MinecraftAuthError::OfflineRequiresMicrosoft => Some(AuthErrorGuidance {
                what_happened: "Offline accounts require a Microsoft account for online features.".to_string(),
                steps_to_fix: vec!["Add a Microsoft account to use online features.".to_string()],
            }),
        }
    }
}