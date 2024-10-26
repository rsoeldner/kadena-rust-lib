use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Failed to serialize command: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] crate::crypto::CryptoError),
    #[error("Signing error: {0}")]
    SigningError(String),
}
