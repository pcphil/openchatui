use super::{LlmProvider, ProviderError, ProviderResult};
use crate::models::{AttachmentData, ChatMessage, Model};
use async_trait::async_trait;
use futures::stream::{BoxStream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct AnthropicProvider {
    client: Client,
    api_key: String,
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
    stream: bool,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: serde_json::Value,
}

#[derive(Deserialize)]
struct AnthropicStreamEvent {
    #[serde(rename = "type")]
    event_type: String,
    delta: Option<AnthropicDelta>,
}

#[derive(Deserialize)]
struct AnthropicDelta {
    #[serde(rename = "type")]
    delta_type: Option<String>,
    text: Option<String>,
}

impl AnthropicProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    fn build_messages(
        messages: Vec<ChatMessage>,
        attachments: Vec<AttachmentData>,
    ) -> Vec<AnthropicMessage> {
        messages
            .into_iter()
            .enumerate()
            .map(|(i, m)| {
                if m.role == "user" && i == 0 && !attachments.is_empty() {
                    let mut content_parts: Vec<serde_json::Value> = Vec::new();
                    for att in &attachments {
                        if att.mime_type.starts_with("image/") {
                            content_parts.push(serde_json::json!({
                                "type": "image",
                                "source": {
                                    "type": "base64",
                                    "media_type": att.mime_type,
                                    "data": att.data
                                }
                            }));
                        }
                    }
                    content_parts.push(serde_json::json!({
                        "type": "text",
                        "text": m.content
                    }));
                    AnthropicMessage {
                        role: m.role,
                        content: serde_json::Value::Array(content_parts),
                    }
                } else {
                    AnthropicMessage {
                        role: m.role,
                        content: serde_json::Value::String(m.content),
                    }
                }
            })
            .collect()
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    async fn list_models(&self) -> ProviderResult<Vec<Model>> {
        Ok(vec![
            Model {
                id: "anthropic:claude-sonnet-4-20250514".to_string(),
                name: "Claude Sonnet 4".to_string(),
                provider: "anthropic".to_string(),
                supports_vision: true,
                supports_streaming: true,
            },
            Model {
                id: "anthropic:claude-opus-4-20250514".to_string(),
                name: "Claude Opus 4".to_string(),
                provider: "anthropic".to_string(),
                supports_vision: true,
                supports_streaming: true,
            },
            Model {
                id: "anthropic:claude-haiku-4-20250414".to_string(),
                name: "Claude Haiku 4".to_string(),
                provider: "anthropic".to_string(),
                supports_vision: true,
                supports_streaming: true,
            },
        ])
    }

    async fn stream_completion(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
        attachments: Vec<AttachmentData>,
    ) -> ProviderResult<BoxStream<'static, ProviderResult<String>>> {
        let request = AnthropicRequest {
            model: model.to_string(),
            messages: Self::build_messages(messages, attachments),
            max_tokens: 4096,
            stream: true,
        };

        let resp = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(ProviderError::Api(format!("Anthropic error: {}", text)));
        }

        let byte_stream = resp.bytes_stream();
        let text_stream = byte_stream.filter_map(|chunk| async move {
            match chunk {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    let mut tokens = String::new();
                    for line in text.lines() {
                        let line = line.trim();
                        if let Some(data) = line.strip_prefix("data: ") {
                            if let Ok(event) =
                                serde_json::from_str::<AnthropicStreamEvent>(data)
                            {
                                if event.event_type == "content_block_delta" {
                                    if let Some(delta) = event.delta {
                                        if let Some(text) = delta.text {
                                            tokens.push_str(&text);
                                        }
                                    }
                                }
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
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&serde_json::json!({
                "model": "claude-haiku-4-20250414",
                "max_tokens": 1,
                "messages": [{"role": "user", "content": "hi"}]
            }))
            .send()
            .await;
        match resp {
            Ok(r) => Ok(r.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    fn provider_name(&self) -> &str {
        "anthropic"
    }
}
