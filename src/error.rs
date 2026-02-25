use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum BayarcashError {
    #[error("Validation failed: {message}")]
    Validation {
        message: String,
        errors: HashMap<String, Vec<String>>,
    },

    #[error("Resource not found")]
    NotFound,

    #[error("Action failed: {message}")]
    FailedAction {
        message: String,
        details: Option<serde_json::Value>,
    },

    #[error("Rate limit exceeded (resets at {reset_at:?})")]
    RateLimitExceeded { reset_at: Option<u64> },

    #[error("Request timed out")]
    Timeout,

    #[error("API version mismatch: {0} requires v3")]
    ApiVersionMismatch(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, BayarcashError>;
