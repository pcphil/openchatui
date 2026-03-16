pub mod anthropic;
pub mod google;
pub mod ollama;
pub mod openai;

use crate::models::{AttachmentData, ChatMessage, Model};
use async_trait::async_trait;
use futures::stream::BoxStream;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("API error: {0}")]
    Api(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Connection error: {0}")]
    Connection(String),
}

pub type ProviderResult<T> = Result<T, ProviderError>;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn list_models(&self) -> ProviderResult<Vec<Model>>;
    async fn stream_completion(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
        attachments: Vec<AttachmentData>,
    ) -> ProviderResult<BoxStream<'static, ProviderResult<String>>>;
    async fn test_connection(&self) -> ProviderResult<bool>;
    fn provider_name(&self) -> &str;
}

pub struct ProviderRegistry {
    providers: HashMap<String, Box<dyn LlmProvider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            providers: HashMap::new(),
        };
        // Always register Ollama (no API key needed)
        registry.register("ollama", Box::new(ollama::OllamaProvider::new()));
        registry
    }

    pub fn register(&mut self, name: &str, provider: Box<dyn LlmProvider>) {
        self.providers.insert(name.to_string(), provider);
    }

    pub fn get(&self, name: &str) -> Option<&dyn LlmProvider> {
        self.providers.get(name).map(|p| p.as_ref())
    }

    pub fn configure_provider(&mut self, name: &str, api_key: &str) {
        match name {
            "openai" => {
                self.register(
                    "openai",
                    Box::new(openai::OpenAiProvider::new(api_key.to_string())),
                );
            }
            "anthropic" => {
                self.register(
                    "anthropic",
                    Box::new(anthropic::AnthropicProvider::new(api_key.to_string())),
                );
            }
            "google" => {
                self.register(
                    "google",
                    Box::new(google::GoogleProvider::new(api_key.to_string())),
                );
            }
            _ => {}
        }
    }

    pub fn providers(&self) -> Vec<&str> {
        self.providers.keys().map(|k| k.as_str()).collect()
    }
}
