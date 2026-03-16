use super::{LlmProvider, ProviderError, ProviderResult};
use crate::models::{AttachmentData, ChatMessage, Model};
use async_trait::async_trait;
use futures::stream::{BoxStream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OpenAiProvider {
    client: Client,
    api_key: String,
}

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    stream: bool,
}

#[derive(Serialize)]
struct OpenAiMessage {
    role: String,
    content: serde_json::Value,
}

#[derive(Deserialize)]
struct OpenAiStreamChunk {
    choices: Vec<OpenAiChoice>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    delta: Option<OpenAiDelta>,
}

#[derive(Deserialize)]
struct OpenAiDelta {
    content: Option<String>,
}

#[derive(Deserialize)]
struct OpenAiModelList {
    data: Vec<OpenAiModelInfo>,
}

#[derive(Deserialize)]
struct OpenAiModelInfo {
    id: String,
}

impl OpenAiProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    fn build_messages(
        messages: Vec<ChatMessage>,
        attachments: Vec<AttachmentData>,
    ) -> Vec<OpenAiMessage> {
        messages
            .into_iter()
            .enumerate()
            .map(|(i, m)| {
                // For the last user message, include image attachments
                if m.role == "user" && i == 0 && !attachments.is_empty() {
                    let mut content_parts: Vec<serde_json::Value> =
                        vec![serde_json::json!({"type": "text", "text": m.content})];
                    for att in &attachments {
                        if att.mime_type.starts_with("image/") {
                            content_parts.push(serde_json::json!({
                                "type": "image_url",
                                "image_url": {
                                    "url": format!("data:{};base64,{}", att.mime_type, att.data)
                                }
                            }));
                        }
                    }
                    OpenAiMessage {
                        role: m.role,
                        content: serde_json::Value::Array(content_parts),
                    }
                } else {
                    OpenAiMessage {
                        role: m.role,
                        content: serde_json::Value::String(m.content),
                    }
                }
            })
            .collect()
    }
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    async fn list_models(&self) -> ProviderResult<Vec<Model>> {
        let resp = self
            .client
            .get("https://api.openai.com/v1/models")
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(ProviderError::Api(format!("OpenAI error: {}", text)));
        }

        let list: OpenAiModelList = resp.json().await?;
        let models = list
            .data
            .into_iter()
            .filter(|m| m.id.starts_with("gpt-") || m.id.starts_with("o"))
            .map(|m| {
                let supports_vision = m.id.contains("gpt-4") || m.id.contains("o1") || m.id.contains("o3") || m.id.contains("o4");
                Model {
                    id: format!("openai:{}", m.id),
                    name: m.id.clone(),
                    provider: "openai".to_string(),
                    supports_vision,
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
        let request = OpenAiRequest {
            model: model.to_string(),
            messages: Self::build_messages(messages, attachments),
            stream: true,
        };

        let resp = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(ProviderError::Api(format!("OpenAI error: {}", text)));
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
                            if data == "[DONE]" {
                                break;
                            }
                            if let Ok(chunk) = serde_json::from_str::<OpenAiStreamChunk>(data) {
                                for choice in chunk.choices {
                                    if let Some(delta) = choice.delta {
                                        if let Some(content) = delta.content {
                                            tokens.push_str(&content);
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
            .get("https://api.openai.com/v1/models")
            .bearer_auth(&self.api_key)
            .send()
            .await;
        match resp {
            Ok(r) => Ok(r.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    fn provider_name(&self) -> &str {
        "openai"
    }
}
