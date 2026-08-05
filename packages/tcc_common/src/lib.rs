//! TCC Launcher common utilities.

pub mod error;

use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error("invalid UUID: {0}")]
    InvalidUuid(#[from] uuid::Error),
    
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, CommonError>;