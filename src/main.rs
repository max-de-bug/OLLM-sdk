use ollm_sdk::{ChatMessage, OllmClient, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllmClient::new(
        "https://api.ollm.com/v1",
        "your-api-key",
    );

    let response = client
        .chat(
            vec![ChatMessage {
                role: "user".to_string(),
                content: "Why is the sky blue?".to_string(),
            }],
            Model::NearGLM46.as_str(),
        )
        .await?;

    println!("Response: {}", response.first_content()?);
    Ok(())
}
