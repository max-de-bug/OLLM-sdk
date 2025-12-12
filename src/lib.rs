pub mod chat;
pub mod client;
pub mod error;
pub mod models;

pub use chat::{ChatMessage, ChatResponse};
pub use client::OllmClient;
pub use error::OllmError;
