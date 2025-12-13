# OLLM SDK

A production-ready, type-safe Rust SDK for the [OLLM API](https://api.ollm.com) with compile-time error checking and zero-cost abstractions.

## Features

- ✅ **Type-safe API** - Compile-time error detection for all API calls
- ✅ **Async/Await** - Built on tokio for high-performance async operations
- ✅ **Structured Error Handling** - Explicit error types with `thiserror`
- ✅ **Developer-Friendly** - Helper methods and clean API design
- ✅ **Zero Boilerplate** - 85% less code than raw HTTP requests

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ollm_sdk = "0.1.3"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
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
```

## Usage Examples

### Basic Chat Completion

```rust
use ollm_sdk::{ChatMessage, OllmClient, Model};

let client = OllmClient::new("https://api.ollm.com/v1", "your-api-key");

let response = client
    .chat(
        vec![ChatMessage {
            role: "user".to_string(),
            content: "Hello!".to_string(),
        }],
        Model::NearGLM46.as_str(),
    )
    .await?;

println!("{}", response.first_content()?);
```

### Multiple Messages (Conversation)

```rust
let messages = vec![
    ChatMessage {
        role: "system".to_string(),
        content: "You are a helpful assistant.".to_string(),
    },
    ChatMessage {
        role: "user".to_string(),
        content: "What is Rust?".to_string(),
    },
];

let response = client.chat(messages, Model::NearGLM46.as_str()).await?;
```

### Error Handling

```rust
match client.chat(messages, Model::NearGLM46.as_str()).await {
    Ok(response) => {
        println!("Success: {}", response.first_content()?);
    }
    Err(ollm_sdk::OllmError::ApiError { status, message }) => {
        eprintln!("API Error {}: {}", status, message);
    }
    Err(e) => {
        eprintln!("Request failed: {}", e);
    }
}
```

### Access Full Response

```rust
let response = client.chat(messages, model).await?;

// Get all choices
for choice in &response.choices {
    println!("Role: {}", choice.message.role);
    println!("Content: {}", choice.message.content);
}

// Access metadata
if let Some(usage) = &response.usage {
    println!("Tokens used: {}", usage.total_tokens.unwrap_or(0));
}
```

## Available Models

```rust
use ollm_sdk::Model;

Model::NearGLM46.as_str()    // "near/GLM-4.6"
```

## API Reference

### `OllmClient`

The main client for making API requests.

```rust
let client = OllmClient::new(base_url: impl Into<String>, api_key: impl Into<String>);
```

### `chat()`

Send a chat completion request.

```rust
pub async fn chat(
    &self,
    messages: Vec<ChatMessage>,
    model: &str,
) -> Result<ChatResponse, OllmError>
```

### `ChatResponse`

The response from a chat completion request.

```rust
pub struct ChatResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: Option<Usage>,
}

impl ChatResponse {
    pub fn first_content(&self) -> Result<&str, OllmError>;
}
```

## Error Handling

The SDK uses structured error types:

```rust
pub enum OllmError {
    Request(reqwest::Error),           // HTTP request failed
    ApiError { status: u16, message: String },  // API returned error
    InvalidResponse,                   // Response format invalid
    MissingChoice,                     // No choices in response
}
```

## Comparison with Raw HTTP

**Without SDK (30+ lines):**

```rust
let response = client
    .post("https://api.ollm.com/v1/chat/completions")
    .header("Authorization", "Bearer your-api-key")
    .header("Content-Type", "application/json")
    .json(&json!({...}))
    .send()
    .await?;
// ... manual error handling, JSON parsing, etc.
```

**With SDK (3 lines):**

```rust
let response = client.chat(messages, model).await?;
println!("{}", response.first_content()?);
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
