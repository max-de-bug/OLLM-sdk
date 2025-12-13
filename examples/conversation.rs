use ollm_sdk::{ChatMessage, OllmClient, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllmClient::new(
        "https://api.ollm.com/v1",
        std::env::var("OLLM_API_KEY")
            .expect("Please set OLLM_API_KEY environment variable"),
    );

    // Create a conversation with multiple messages
    let messages = vec![
        ChatMessage {
            role: "system".to_string(),
            content: "You are a helpful assistant that explains things clearly.".to_string(),
        },
        ChatMessage {
            role: "user".to_string(),
            content: "What is Rust?".to_string(),
        },
    ];

    let response = client
        .chat(messages, Model::NearGLM46.as_str())
        .await?;

    println!("Assistant: {}", response.first_content()?);

    Ok(())
}

