use crate::models::{AttachmentData, ChatMessage, Message, StreamEvent};
use crate::providers::ProviderRegistry;
use crate::sandbox::manager::SandboxManager;
use crate::sandbox::trigger::{parse_sandbox_trigger, strip_sandbox_block, SANDBOX_SYSTEM_PROMPT};
use crate::sandbox::SandboxConfig;
use futures::StreamExt;
use tauri::ipc::Channel;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn send_message(
    registry: State<'_, Mutex<ProviderRegistry>>,
    messages_db: State<'_, Mutex<Vec<Message>>>,
    sandbox_manager: State<'_, Mutex<SandboxManager>>,
    conversation_id: String,
    content: String,
    model_id: String,
    attachments: Vec<AttachmentData>,
    on_event: Channel<StreamEvent>,
) -> Result<String, String> {
    // Parse provider:model format
    let (provider_name, model_name) = model_id
        .split_once(':')
        .ok_or_else(|| "Invalid model ID format. Expected 'provider:model'".to_string())?;

    // Save user message
    {
        let mut msgs = messages_db.lock().await;
        let sort_order = msgs
            .iter()
            .filter(|m| m.conversation_id == conversation_id)
            .count() as i64;
        let msg = Message {
            id: uuid::Uuid::new_v4().to_string(),
            conversation_id: conversation_id.clone(),
            role: "user".to_string(),
            content: content.clone(),
            token_count: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            sort_order,
        };
        msgs.push(msg);
    }

    // Build message history for the provider, with sandbox system prompt
    let chat_messages: Vec<ChatMessage> = {
        let msgs = messages_db.lock().await;
        let mut messages: Vec<ChatMessage> = vec![ChatMessage {
            role: "system".to_string(),
            content: SANDBOX_SYSTEM_PROMPT.to_string(),
        }];
        messages.extend(
            msgs.iter()
                .filter(|m| m.conversation_id == conversation_id)
                .map(|m| ChatMessage {
                    role: m.role.clone(),
                    content: m.content.clone(),
                }),
        );
        messages
    };

    // Get provider and stream completion
    // Lock registry, get the stream, then drop the lock before awaiting
    let stream_result = {
        let reg = registry.lock().await;
        let provider = reg
            .get(provider_name)
            .ok_or_else(|| format!("Provider '{}' not configured", provider_name))?;
        provider
            .stream_completion(model_name, chat_messages, attachments)
            .await
            .map_err(|e| e.to_string())
    };

    match stream_result {
        Ok(mut stream) => {
            let mut full_response = String::new();

            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(token) => {
                        full_response.push_str(&token);
                        let _ = on_event.send(StreamEvent::Token(token));
                    }
                    Err(e) => {
                        let err_msg = e.to_string();
                        let _ = on_event.send(StreamEvent::Error(err_msg.clone()));
                        return Err(err_msg);
                    }
                }
            }

            // Check for sandbox trigger before saving the response
            let visible_response = if let Some(trigger) = parse_sandbox_trigger(&full_response) {
                let stripped = strip_sandbox_block(&full_response);

                // Spawn sandbox creation in the background
                if trigger.action == "launch" {
                    let mgr = sandbox_manager.lock().await;
                    let conv_id = conversation_id.clone();
                    let config = SandboxConfig {
                        project_dir: trigger.project_dir,
                        ..Default::default()
                    };

                    match mgr.create_sandbox(&conv_id, config).await {
                        Ok(info) => {
                            // Write conversation context to the sandbox
                            let context = format!(
                                "# Task\n{}\n\n# Description\n{}",
                                content,
                                trigger.description.unwrap_or_default()
                            );
                            let _ = mgr
                                .write_file(
                                    &info.sandbox_id,
                                    "/workspace/.context.md",
                                    context.as_bytes(),
                                )
                                .await;

                            // Note: OpenCode execution is triggered by the frontend
                            // via exec_in_sandbox after it receives the sandbox info
                            log::info!("Sandbox {} created for conversation {}", info.sandbox_id, conv_id);
                        }
                        Err(e) => {
                            log::warn!("Failed to create sandbox: {}", e);
                        }
                    }
                }

                stripped
            } else {
                full_response.clone()
            };

            // Save assistant message (with sandbox blocks stripped)
            {
                let mut msgs = messages_db.lock().await;
                let sort_order = msgs
                    .iter()
                    .filter(|m| m.conversation_id == conversation_id)
                    .count() as i64;
                let assistant_msg = Message {
                    id: uuid::Uuid::new_v4().to_string(),
                    conversation_id,
                    role: "assistant".to_string(),
                    content: visible_response.clone(),
                    token_count: None,
                    created_at: chrono::Utc::now().to_rfc3339(),
                    sort_order,
                };
                msgs.push(assistant_msg);
            }

            let _ = on_event.send(StreamEvent::Done(visible_response.clone()));
            Ok(visible_response)
        }
        Err(e) => {
            let _ = on_event.send(StreamEvent::Error(e.clone()));
            Err(e)
        }
    }
}
