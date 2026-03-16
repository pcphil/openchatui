use super::{LlmProvider, ProviderError, ProviderResult};
use crate::models::{AttachmentData, ChatMessage, Model};
use async_trait::async_trait;
use futures::stream::{BoxStream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OllamaProvider {
    client: Client,
    base_url: String,
}

#[derive(Deserialize)]
struct OllamaModelList {
    models: Option<Vec<OllamaModel>>,
}

#[derive(Deserialize)]
struct OllamaModel {
    name: String,
}

#[derive(Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<OllamaChatMessage>,
    stream: bool,
}

#[derive(Serialize)]
struct OllamaChatMessage {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct OllamaChatResponse {
    message: Option<OllamaResponseMessage>,
    done: Option<bool>,
}

#[derive(Deserialize)]
struct OllamaResponseMessage {
    content: String,
}

impl OllamaProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "http://localhost:11434".to_string(),
        }
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    async fn list_models(&self) -> ProviderResult<Vec<Model>> {
        let resp = self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map_err(|e| ProviderError::Connection(format!("Cannot connect to Ollama: {}", e)))?;

        let list: OllamaModelList = resp.json().await?;
        let models = list
            .models
            .unwrap_or_default()
            .into_iter()
            .map(|m| {
                let name = m.name.clone();
                Model {
                    id: format!("ollama:{}", name),
                    name: name.clone(),
                    provider: "ollama".to_string(),
                    supports_vision: name.contains("llava") || name.contains("vision"),
                    supports_streaming: true,
                }
            })
            .collect();
        Ok(models)
    }

    async fn stream_completion(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
        attachments: Vec<AttachmentData>,
    ) -> ProviderResult<BoxStream<'static, ProviderResult<String>>> {
        let ollama_messages: Vec<OllamaChatMessage> = messages
            .into_iter()
            .enumerate()
            .map(|(i, m)| {
                // Attach images to the last user message
                let images = if m.role == "user" && i == 0 {
                    let imgs: Vec<String> = attachments
                        .iter()
                        .filter(|a| a.mime_type.starts_with("image/"))
                        .map(|a| a.data.clone())
                        .collect();
                    if imgs.is_empty() {
                        None
                    } else {
                        Some(imgs)
                    }
                } else {
                    None
                };
                OllamaChatMessage {
                    role: m.role,
                    content: m.content,
                    images,
                }
            })
            .collect();

        let request = OllamaChatRequest {
            model: model.to_string(),
            messages: ollama_messages,
            stream: true,
        };

        let resp = self
            .client
            .post(format!("{}/api/chat", self.base_url))
            .json(&request)
            .send()
            .await?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(ProviderError::Api(format!("Ollama error: {}", text)));
        }

        let byte_stream = resp.bytes_stream();

        let text_stream = byte_stream.filter_map(|chunk| async move {
            match chunk {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    // Ollama returns NDJSON - one JSON object per line
                    let mut tokens = String::new();
                    for line in text.lines() {
                        if line.trim().is_empty() {
                            continue;
                        }
                        if let Ok(resp) = serde_json::from_str::<OllamaChatResponse>(line) {
                            if let Some(msg) = resp.message {
                                tokens.push_str(&msg.content);
                            }
                            if resp.done == Some(true) {
                                break;
                            }
                        }
                    }
                    if tokens.is_empty() {
                        None
                    } else {
                        Some(Ok(tokens))
                    }
                }
                Err(e) => Some(Err(ProviderError::Http(e))),
            }
        });

        Ok(Box::pin(text_stream))
    }

    async fn test_connection(&self) -> ProviderResult<bool> {
        let resp = self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await;
        match resp {
            Ok(r) => Ok(r.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    fn provider_name(&self) -> &str {
        "ollama"
    }
}
