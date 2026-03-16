pub mod docker;
pub mod manager;
pub mod trigger;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub project_dir: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_limit: Option<f64>,
    pub network_enabled: Option<bool>,
    pub environment: Option<std::collections::HashMap<String, String>>,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            project_dir: None,
            memory_limit: Some("2g".to_string()),
            cpu_limit: Some(1.0),
            network_enabled: Some(false),
            environment: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SandboxStatus {
    Creating,
    Running,
    Stopped,
    Failed,
    Destroyed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxInfo {
    pub sandbox_id: String,
    pub conversation_id: String,
    pub status: SandboxStatus,
    pub container_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeProposal {
    pub proposal_id: String,
    pub file_path: String,
    pub description: String,
    pub diff: String,
    pub original_content: Option<String>,
    pub proposed_content: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum SandboxEvent {
    StatusChanged(SandboxStatus),
    Output { stream: String, text: String },
    ProposalReady(ChangeProposal),
    ProposalResult { proposal_id: String, approved: bool },
    Error(String),
}

#[derive(Debug, thiserror::Error)]
pub enum SandboxError {
    #[error("Docker error: {0}")]
    Docker(String),
    #[error("Container not found: {0}")]
    NotFound(String),
    #[error("Sandbox already exists for conversation: {0}")]
    AlreadyExists(String),
    #[error("No pending proposal: {0}")]
    NoPendingProposal(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<SandboxError> for String {
    fn from(e: SandboxError) -> Self {
        e.to_string()
    }
}

#[async_trait]
pub trait SandboxRuntime: Send + Sync {
    async fn create(
        &self,
        conversation_id: &str,
        config: SandboxConfig,
    ) -> Result<SandboxInfo, SandboxError>;

    async fn start(&self, sandbox_id: &str) -> Result<(), SandboxError>;

    async fn exec(
        &self,
        sandbox_id: &str,
        command: Vec<String>,
        event_tx: mpsc::Sender<SandboxEvent>,
    ) -> Result<i64, SandboxError>;

    async fn stop(&self, sandbox_id: &str) -> Result<(), SandboxError>;

    async fn destroy(&self, sandbox_id: &str) -> Result<(), SandboxError>;

    async fn info(&self, sandbox_id: &str) -> Result<SandboxInfo, SandboxError>;

    async fn list(&self) -> Result<Vec<SandboxInfo>, SandboxError>;

    async fn write_file(
        &self,
        sandbox_id: &str,
        path: &str,
        content: &[u8],
    ) -> Result<(), SandboxError>;

    async fn read_file(&self, sandbox_id: &str, path: &str) -> Result<Vec<u8>, SandboxError>;
}
