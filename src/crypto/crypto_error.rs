use thiserror::Error;

/// Custom error type for cryptographic operations
#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Hex decoding error: {0}")]
    HexError(#[from] hex::FromHexError),
    #[error("Base64 decoding error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("Ed25519 error: {0}")]
    Ed25519Error(#[from] ed25519_dalek::SignatureError),
    #[error("Invalid seed length")]
    InvalidSeedLength,
}
