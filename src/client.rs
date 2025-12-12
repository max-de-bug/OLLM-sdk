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
    pub base_url: String,
    pub api_key: String,
    pub http: Client,
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
