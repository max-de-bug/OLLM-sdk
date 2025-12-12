use reqwest::Client;

#[derive(Clone)]
pub struct OllmClient {
    pub base_url: String,
    pub api_key: String,
    pub http: Client,
}

impl OllmClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: api_key.into(),
            http: Client::new(),
        }
    }
}
