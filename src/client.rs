use reqwest::Client;

/// Client for interacting with the OLLM API.
///
/// # Example
///
/// ```no_run
/// use ollm_sdk::OllmClient;
///
/// let client = OllmClient::new(
///     "https://api.ollm.com/v1",
///     "your-api-key",
/// );
/// ```
#[derive(Clone)]
pub struct OllmClient {
    pub(crate) base_url: String,
    pub(crate) api_key: String,
    pub(crate) http: Client,
}

impl OllmClient {
    /// Creates a new OLLM client.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the OLLM API (e.g., "https://api.ollm.com/v1")
    /// * `api_key` - Your OLLM API key
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ollm_sdk::OllmClient;
    ///
    /// let client = OllmClient::new(
    ///     "https://api.ollm.com/v1",
    ///     "your-api-key",
    /// );
    /// ```
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: api_key.into(),
            http: Client::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let _client = OllmClient::new("https://api.ollm.com/v1", "test-key");
        // Client should be created successfully - just verify it doesn't panic
    }

    #[test]
    fn test_client_clone() {
        let client = OllmClient::new("https://api.ollm.com/v1", "test-key");
        let _cloned = client.clone();
        // Both should be valid - just verify clone works
    }
}
