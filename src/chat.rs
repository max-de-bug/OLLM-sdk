use crate::client::OllmClient;
use crate::error::OllmError;
use serde::{Deserialize, Serialize};
use reqwest::Response;

/// A message in a chat conversation.
#[derive(Serialize, Clone, Debug)]
pub struct ChatMessage {
    /// The role of the message sender (e.g., "user", "assistant", "system")
    pub role: String,
    /// The content of the message
    pub content: String,
}

#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: Option<Usage>,
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
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
    /// Gets the content of the first choice in the response.
    ///
    /// # Returns
    ///
    /// Returns `Ok(&str)` with the content if a choice exists, or `Err(OllmError::MissingChoice)` if no choices are present.
    pub fn first_content(&self) -> Result<&str, OllmError> {
        self.choices
            .first()
            .map(|c| c.message.content.as_str())
            .ok_or(OllmError::MissingChoice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_content_with_choices() {
        let response = ChatResponse {
            id: None,
            object: None,
            created: None,
            model: "near/GLM-4.6".to_string(),
            choices: vec![ChatChoice {
                message: ChatMessageResponse {
                    role: "assistant".to_string(),
                    content: "Hello, world!".to_string(),
                },
            }],
            usage: None,
        };

        assert_eq!(response.first_content().unwrap(), "Hello, world!");
    }

    #[test]
    fn test_first_content_without_choices() {
        let response = ChatResponse {
            id: None,
            object: None,
            created: None,
            model: "near/GLM-4.6".to_string(),
            choices: vec![],
            usage: None,
        };

        assert!(matches!(
            response.first_content(),
            Err(OllmError::MissingChoice)
        ));
    }

    #[test]
    fn test_chat_message_serialization() {
        let message = ChatMessage {
            role: "user".to_string(),
            content: "Test message".to_string(),
        };

        let json = serde_json::to_string(&message).unwrap();
        assert!(json.contains("user"));
        assert!(json.contains("Test message"));
    }
}

impl OllmClient {
    /// Sends a chat completion request to the OLLM API.
    ///
    /// # Arguments
    ///
    /// * `messages` - A vector of chat messages in the conversation
    /// * `model` - The model to use (e.g., "near/GLM-4.6")
    ///
    /// # Returns
    ///
    /// Returns `Ok(ChatResponse)` on success, or `Err(OllmError)` on failure.
    pub async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
    ) -> Result<ChatResponse, OllmError> {
        let url = format!("{}/chat/completions", self.base_url);

        let body = ChatRequest {
            model: model.to_string(),
            messages,
        };

        let response: Response = self
            .http
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let error_text: String = response
                .text()
                .await
                .unwrap_or_else(|e| format!("Failed to read error response: {}", e));
            return Err(OllmError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let res: ChatResponse = response.json().await?;
        Ok(res)
    }
}
