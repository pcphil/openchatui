use super::{
    ChangeProposal, SandboxConfig, SandboxError, SandboxEvent, SandboxInfo, SandboxRuntime,
    SandboxStatus,
};
use super::docker::DockerRuntime;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

pub struct SandboxManager {
    runtime: Arc<DockerRuntime>,
    /// conversation_id → sandbox_id
    sessions: RwLock<HashMap<String, String>>,
    /// sandbox_id → SandboxInfo
    sandboxes: RwLock<HashMap<String, SandboxInfo>>,
    /// proposal_id → (sandbox_id, ChangeProposal)
    pending_proposals: Arc<RwLock<HashMap<String, (String, ChangeProposal)>>>,
}

impl SandboxManager {
    pub fn new() -> Result<Self, SandboxError> {
        let runtime = DockerRuntime::new()?;
        Ok(Self {
            runtime: Arc::new(runtime),
            sessions: RwLock::new(HashMap::new()),
            sandboxes: RwLock::new(HashMap::new()),
            pending_proposals: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub async fn create_sandbox(
        &self,
        conversation_id: &str,
        config: SandboxConfig,
    ) -> Result<SandboxInfo, SandboxError> {
        // Check if conversation already has a sandbox
        {
            let sessions = self.sessions.read().await;
            if sessions.contains_key(conversation_id) {
                return Err(SandboxError::AlreadyExists(conversation_id.to_string()));
            }
        }

        let info = self.runtime.create(conversation_id, config).await?;
        self.runtime.start(&info.sandbox_id).await?;

        let mut running_info = info.clone();
        running_info.status = SandboxStatus::Running;

        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(conversation_id.to_string(), running_info.sandbox_id.clone());
        }
        {
            let mut sandboxes = self.sandboxes.write().await;
            sandboxes.insert(running_info.sandbox_id.clone(), running_info.clone());
        }

        Ok(running_info)
    }

    pub async fn exec_in_sandbox(
        &self,
        sandbox_id: &str,
        command: Vec<String>,
        event_tx: mpsc::Sender<SandboxEvent>,
    ) -> Result<i64, SandboxError> {
        // Wrap the event_tx to intercept proposals
        let (proxy_tx, mut proxy_rx) = mpsc::channel::<SandboxEvent>(256);

        let sandbox_id_owned = sandbox_id.to_string();
        let pending = self.pending_proposals.clone();
        let user_tx = event_tx.clone();

        // Spawn a task to intercept proposals and store them
        tokio::spawn(async move {
            while let Some(event) = proxy_rx.recv().await {
                match &event {
                    SandboxEvent::ProposalReady(proposal) => {
                        let mut pending = pending.write().await;
                        pending.insert(
                            proposal.proposal_id.clone(),
                            (sandbox_id_owned.clone(), proposal.clone()),
                        );
                    }
                    _ => {}
                }
                let _ = user_tx.send(event).await;
            }
        });

        self.runtime.exec(sandbox_id, command, proxy_tx).await
    }

    pub async fn approve_proposal(
        &self,
        proposal_id: &str,
    ) -> Result<(String, ChangeProposal), SandboxError> {
        let mut pending = self.pending_proposals.write().await;
        let (sandbox_id, proposal) = pending
            .remove(proposal_id)
            .ok_or_else(|| SandboxError::NoPendingProposal(proposal_id.to_string()))?;
        Ok((sandbox_id, proposal))
    }

    pub async fn reject_proposal(
        &self,
        proposal_id: &str,
    ) -> Result<(String, ChangeProposal), SandboxError> {
        let mut pending = self.pending_proposals.write().await;
        let (sandbox_id, proposal) = pending
            .remove(proposal_id)
            .ok_or_else(|| SandboxError::NoPendingProposal(proposal_id.to_string()))?;
        Ok((sandbox_id, proposal))
    }

    pub async fn stop_sandbox(&self, sandbox_id: &str) -> Result<(), SandboxError> {
        self.runtime.stop(sandbox_id).await?;
        let mut sandboxes = self.sandboxes.write().await;
        if let Some(info) = sandboxes.get_mut(sandbox_id) {
            info.status = SandboxStatus::Stopped;
        }
        Ok(())
    }

    pub async fn destroy_sandbox(&self, sandbox_id: &str) -> Result<(), SandboxError> {
        self.runtime.destroy(sandbox_id).await?;

        // Clean up session mapping
        let mut sessions = self.sessions.write().await;
        sessions.retain(|_, v| v != sandbox_id);

        let mut sandboxes = self.sandboxes.write().await;
        sandboxes.remove(sandbox_id);

        // Clean up pending proposals for this sandbox
        let mut pending = self.pending_proposals.write().await;
        pending.retain(|_, (sid, _)| sid != sandbox_id);

        Ok(())
    }

    pub async fn get_sandbox_for_conversation(
        &self,
        conversation_id: &str,
    ) -> Option<SandboxInfo> {
        let sessions = self.sessions.read().await;
        let sandbox_id = sessions.get(conversation_id)?;
        let sandboxes = self.sandboxes.read().await;
        sandboxes.get(sandbox_id).cloned()
    }

    pub async fn get_sandbox_info(&self, sandbox_id: &str) -> Option<SandboxInfo> {
        let sandboxes = self.sandboxes.read().await;
        sandboxes.get(sandbox_id).cloned()
    }

    pub async fn write_file(
        &self,
        sandbox_id: &str,
        path: &str,
        content: &[u8],
    ) -> Result<(), SandboxError> {
        self.runtime.write_file(sandbox_id, path, content).await
    }
}
