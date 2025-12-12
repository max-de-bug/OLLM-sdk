use thiserror::Error;

#[derive(Debug, Error)]
pub enum OllmError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API error (status {status}): {message}")]
    ApiError { status: u16, message: String },

    #[error("Invalid response format")]
    InvalidResponse,

    #[error("Missing choice in response")]
    MissingChoice,
}
