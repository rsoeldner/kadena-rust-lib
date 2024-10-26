use thiserror::Error;

/// Errors that can occur during fetch operations
#[derive(Debug, Error)]
pub enum FetchError {
    /// Network-related errors
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    /// JSON serialization/deserialization errors
    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    /// API-specific errors
    #[error("API error: {0}")]
    ApiError(String),
}
