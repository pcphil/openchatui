use crate::sandbox::manager::SandboxManager;
use crate::sandbox::{SandboxConfig, SandboxEvent, SandboxInfo};
use tauri::ipc::Channel;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn create_sandbox(
    manager: State<'_, Mutex<SandboxManager>>,
    conversation_id: String,
    config: Option<SandboxConfig>,
    on_event: Channel<SandboxEvent>,
) -> Result<SandboxInfo, String> {
    let mgr = manager.lock().await;
    let config = config.unwrap_or_default();

    let _ = on_event.send(SandboxEvent::StatusChanged(
        crate::sandbox::SandboxStatus::Creating,
    ));

    let info = mgr
        .create_sandbox(&conversation_id, config)
        .await
        .map_err(|e| e.to_string())?;

    let _ = on_event.send(SandboxEvent::StatusChanged(
        crate::sandbox::SandboxStatus::Running,
    ));

    Ok(info)
}

#[tauri::command]
pub async fn exec_in_sandbox(
    manager: State<'_, Mutex<SandboxManager>>,
    sandbox_id: String,
    command: Vec<String>,
    on_event: Channel<SandboxEvent>,
) -> Result<i64, String> {
    let mgr = manager.lock().await;
    let (tx, mut rx) = tokio::sync::mpsc::channel::<SandboxEvent>(256);

    // Forward events from the channel to the Tauri Channel
    let event_channel = on_event.clone();
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            let _ = event_channel.send(event);
        }
    });

    let exit_code = mgr
        .exec_in_sandbox(&sandbox_id, command, tx)
        .await
        .map_err(|e| e.to_string())?;

    Ok(exit_code)
}

#[tauri::command]
pub async fn approve_proposal(
    manager: State<'_, Mutex<SandboxManager>>,
    proposal_id: String,
    on_event: Channel<SandboxEvent>,
) -> Result<(), String> {
    let mgr = manager.lock().await;
    let (sandbox_id, proposal) = mgr
        .approve_proposal(&proposal_id)
        .await
        .map_err(|e| e.to_string())?;

    let _ = on_event.send(SandboxEvent::ProposalResult {
        proposal_id: proposal.proposal_id.clone(),
        approved: true,
    });

    // Write approval to the sandbox's stdin (OpenCode reads this)
    // We use exec to echo the approval signal
    let (tx, _rx) = tokio::sync::mpsc::channel::<SandboxEvent>(1);
    let _ = mgr
        .exec_in_sandbox(
            &sandbox_id,
            vec![
                "sh".to_string(),
                "-c".to_string(),
                "echo APPROVED > /tmp/sandbox_approval".to_string(),
            ],
            tx,
        )
        .await;

    Ok(())
}

#[tauri::command]
pub async fn reject_proposal(
    manager: State<'_, Mutex<SandboxManager>>,
    proposal_id: String,
    on_event: Channel<SandboxEvent>,
) -> Result<(), String> {
    let mgr = manager.lock().await;
    let (_sandbox_id, proposal) = mgr
        .reject_proposal(&proposal_id)
        .await
        .map_err(|e| e.to_string())?;

    let _ = on_event.send(SandboxEvent::ProposalResult {
        proposal_id: proposal.proposal_id.clone(),
        approved: false,
    });

    let (tx, _rx) = tokio::sync::mpsc::channel::<SandboxEvent>(1);
    let _ = mgr
        .exec_in_sandbox(
            &_sandbox_id,
            vec![
                "sh".to_string(),
                "-c".to_string(),
                "echo REJECTED > /tmp/sandbox_approval".to_string(),
            ],
            tx,
        )
        .await;

    Ok(())
}

#[tauri::command]
pub async fn stop_sandbox(
    manager: State<'_, Mutex<SandboxManager>>,
    sandbox_id: String,
) -> Result<(), String> {
    let mgr = manager.lock().await;
    mgr.stop_sandbox(&sandbox_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn destroy_sandbox(
    manager: State<'_, Mutex<SandboxManager>>,
    sandbox_id: String,
) -> Result<(), String> {
    let mgr = manager.lock().await;
    mgr.destroy_sandbox(&sandbox_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_sandbox_for_conversation(
    manager: State<'_, Mutex<SandboxManager>>,
    conversation_id: String,
) -> Result<Option<SandboxInfo>, String> {
    let mgr = manager.lock().await;
    Ok(mgr.get_sandbox_for_conversation(&conversation_id).await)
}
