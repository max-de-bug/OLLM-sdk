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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = OllmError::ApiError {
            status: 400,
            message: "Bad request".to_string(),
        };
        assert!(error.to_string().contains("400"));
        assert!(error.to_string().contains("Bad request"));
    }

    #[test]
    fn test_missing_choice_error() {
        let error = OllmError::MissingChoice;
        assert_eq!(error.to_string(), "Missing choice in response");
    }

    #[test]
    fn test_api_error() {
        let error = OllmError::ApiError {
            status: 401,
            message: "Unauthorized".to_string(),
        };
        assert!(error.to_string().contains("401"));
        assert!(error.to_string().contains("Unauthorized"));
    }
}
