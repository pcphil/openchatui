use super::{LlmProvider, ProviderError, ProviderResult};
use crate::models::{AttachmentData, ChatMessage, Model};
use async_trait::async_trait;
use futures::stream::{BoxStream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct GoogleProvider {
    client: Client,
    api_key: String,
}

#[derive(Serialize)]
struct GoogleRequest {
    contents: Vec<GoogleContent>,
}

#[derive(Serialize)]
struct GoogleContent {
    role: String,
    parts: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
struct GoogleStreamChunk {
    candidates: Option<Vec<GoogleCandidate>>,
}

#[derive(Deserialize)]
struct GoogleCandidate {
    content: Option<GoogleCandidateContent>,
}

#[derive(Deserialize)]
struct GoogleCandidateContent {
    parts: Option<Vec<GooglePart>>,
}

#[derive(Deserialize)]
struct GooglePart {
    text: Option<String>,
}

impl GoogleProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait]
impl LlmProvider for GoogleProvider {
    async fn list_models(&self) -> ProviderResult<Vec<Model>> {
        Ok(vec![
            Model {
                id: "google:gemini-2.5-pro".to_string(),
                name: "Gemini 2.5 Pro".to_string(),
                provider: "google".to_string(),
                supports_vision: true,
                supports_streaming: true,
            },
            Model {
                id: "google:gemini-2.5-flash".to_string(),
                name: "Gemini 2.5 Flash".to_string(),
                provider: "google".to_string(),
                supports_vision: true,
                supports_streaming: true,
            },
            Model {
                id: "google:gemini-2.0-flash".to_string(),
                name: "Gemini 2.0 Flash".to_string(),
                provider: "google".to_string(),
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
        let contents: Vec<GoogleContent> = messages
            .into_iter()
            .enumerate()
            .map(|(i, m)| {
                let role = if m.role == "assistant" {
                    "model".to_string()
                } else {
                    m.role.clone()
                };

                let mut parts: Vec<serde_json::Value> =
                    vec![serde_json::json!({"text": m.content})];

                if m.role == "user" && i == 0 {
                    for att in &attachments {
                        if att.mime_type.starts_with("image/") {
                            parts.push(serde_json::json!({
                                "inline_data": {
                                    "mime_type": att.mime_type,
                                    "data": att.data
                                }
                            }));
                        }
                    }
                }

                GoogleContent { role, parts }
            })
            .collect();

        let request = GoogleRequest { contents };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?alt=sse&key={}",
            model, self.api_key
        );

        let resp = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(ProviderError::Api(format!("Google AI error: {}", text)));
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
                            if let Ok(chunk) =
                                serde_json::from_str::<GoogleStreamChunk>(data)
                            {
                                if let Some(candidates) = chunk.candidates {
                                    for candidate in candidates {
                                        if let Some(content) = candidate.content {
                                            if let Some(parts) = content.parts {
                                                for part in parts {
                                                    if let Some(text) = part.text {
                                                        tokens.push_str(&text);
                                                    }
                                                }
                                            }
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
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models?key={}",
            self.api_key
        );
        let resp = self.client.get(&url).send().await;
        match resp {
            Ok(r) => Ok(r.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    fn provider_name(&self) -> &str {
        "google"
    }
}
