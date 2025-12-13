use ollm_sdk::{ChatMessage, OllmClient, Model, OllmError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllmClient::new(
        "https://api.ollm.com/v1",
        std::env::var("OLLM_API_KEY")
            .expect("Please set OLLM_API_KEY environment variable"),
    );

    match client
        .chat(
            vec![ChatMessage {
                role: "user".to_string(),
                content: "Hello!".to_string(),
            }],
            Model::NearGLM46.as_str(),
        )
        .await
    {
        Ok(response) => {
            match response.first_content() {
                Ok(content) => println!("Success: {}", content),
                Err(OllmError::MissingChoice) => {
                    eprintln!("Error: API returned no choices in response");
                }
                Err(e) => {
                    eprintln!("Error extracting content: {}", e);
                }
            }
        }
        Err(OllmError::ApiError { status, message }) => {
            eprintln!("API Error ({}): {}", status, message);
        }
        Err(OllmError::Request(e)) => {
            eprintln!("Network error: {}", e);
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
        }
    }

    Ok(())
}

