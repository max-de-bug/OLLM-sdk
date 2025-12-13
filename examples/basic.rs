use ollm_sdk::{ChatMessage, OllmClient, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = OllmClient::new(
        "https://api.ollm.com/v1",
        std::env::var("OLLM_API_KEY")
            .expect("Please set OLLM_API_KEY environment variable"),
    );

    // Send a simple chat message
    let response = client
        .chat(
            vec![ChatMessage {
                role: "user".to_string(),
                content: "Why is the sky blue?".to_string(),
            }],
            Model::NearGLM46.as_str(),
        )
        .await?;

    // Print the response
    println!("Response: {}", response.first_content()?);
    
    // Access metadata
    if let Some(usage) = &response.usage {
        println!(
            "Tokens used: {}",
            usage.total_tokens.unwrap_or(0)
        );
    }

    Ok(())
}

