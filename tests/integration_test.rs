use ollm_sdk::{ChatMessage, OllmClient, Model};

/// Integration tests for real API calls
/// Set OLLM_API_KEY in .env file or environment variable
/// Run with: cargo test --test integration_test -- --ignored

fn setup_client() -> OllmClient {
    let _ = dotenv::dotenv();
    let api_key = std::env::var("OLLM_API_KEY")
        .expect("OLLM_API_KEY not set. Create .env file or set environment variable");
    OllmClient::new("https://api.ollm.com/v1", &api_key)
}

#[tokio::test]
#[ignore]
async fn test_near_glm46() {
    let client = setup_client();
    let response = client
        .chat(
            vec![ChatMessage {
                role: "user".to_string(),
                content: "Say hello".to_string(),
            }],
            Model::NearGLM46.as_str(),
        )
        .await
        .expect("API request failed");
    
    assert!(!response.choices.is_empty());
    println!("✅ {}: {}", response.model, response.first_content().unwrap());
}

#[tokio::test]
#[ignore]
async fn test_near_deepseek() {
    let client = setup_client();
    let response = client
        .chat(
            vec![ChatMessage {
                role: "user".to_string(),
                content: "Say hello".to_string(),
            }],
            Model::NearDeepSeekV31.as_str(),
        )
        .await
        .expect("API request failed");
    
    assert!(!response.choices.is_empty());
    println!("✅ {}: {}", response.model, response.first_content().unwrap());
}

#[tokio::test]
#[ignore]
async fn test_phala_glm46() {
    let client = setup_client();
    let response = client
        .chat(
            vec![ChatMessage {
                role: "user".to_string(),
                content: "Say hello".to_string(),
            }],
            Model::PhalaGLM46.as_str(),
        )
        .await
        .expect("API request failed");
    
    assert!(!response.choices.is_empty());
    println!("✅ {}: {}", response.model, response.first_content().unwrap());
}

#[tokio::test]
#[ignore]
async fn test_all_models() {
    let client = setup_client();
    let models = vec![
        // NEAR Provider Models
        Model::NearDeepSeekV31,
        Model::NearGLM46,
        Model::NearGptOss120b,
        Model::NearQwen330BA3BInstruct2507,
        // Phala Provider Models
        Model::PhalaDeepSeekChatV30324,
        Model::PhalaDeepSeekChatV31,
        Model::PhalaDeepSeekR10528,
        Model::PhalaGemma327bIt,
        Model::PhalaGLM46,
        Model::PhalaGptOss120b,
    ];
    
    println!("Testing {} models...\n", models.len());
    
    for model in models {
        match client
            .chat(
                vec![ChatMessage {
                    role: "user".to_string(),
                    content: "Hi".to_string(),
                }],
                model.as_str(),
            )
            .await
        {
            Ok(resp) => println!("✅ {}: {}", model.as_str(), resp.first_content().unwrap_or("")),
            Err(e) => println!("❌ {}: {}", model.as_str(), e),
        }
    }
}

