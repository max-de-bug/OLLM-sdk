use crate::client::OllmClient;
use crate::error::OllmError;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub choices: Vec<ChatChoice>,
}

#[derive(Deserialize, Debug)]
pub struct ChatChoice {
    pub message: ChatMessageResponse,
}

#[derive(Deserialize, Debug)]
pub struct ChatMessageResponse {
    pub role: String,
    pub content: String,
}

impl ChatResponse {
    /// Helper to get the first choice
    pub fn first_content(&self) -> Result<&str, OllmError> {
        self.choices
            .first()
            .map(|c| c.message.content.as_str())
            .ok_or(OllmError::MissingChoice)
    }
}

impl OllmClient {
    pub async fn chat(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<ChatResponse, OllmError> {
        let url = format!("{}/chat/completions", self.base_url);

        let body = ChatRequest {
            model: model.to_string(),
            messages,
        };

        let res = self
            .http
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?
            .json::<ChatResponse>()
            .await?;

        Ok(res)
    }
}
