use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub model_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub archived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub content: String,
    pub token_count: Option<i64>,
    pub created_at: String,
    pub sort_order: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: String,
    pub message_id: String,
    pub file_name: String,
    pub file_path: String,
    pub mime_type: String,
    pub file_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub supports_vision: bool,
    pub supports_streaming: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentData {
    pub file_name: String,
    pub mime_type: String,
    pub data: String, // base64 encoded
}

#[derive(Clone, Serialize)]
#[serde(tag = "event", content = "data")]
pub enum StreamEvent {
    Token(String),
    Done(String),
    Error(String),
}
