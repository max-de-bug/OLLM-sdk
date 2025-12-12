use thiserror::Error;

#[derive(Debug, Error)]
pub enum OllmError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Invalid response format")]
    InvalidResponse,

    #[error("Missing choice in response")]
    MissingChoice,
}
