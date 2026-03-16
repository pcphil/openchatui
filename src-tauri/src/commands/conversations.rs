use crate::models::{Conversation, Message};
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn create_conversation(
    db: State<'_, Mutex<Vec<Conversation>>>,
    title: Option<String>,
    model_id: Option<String>,
) -> Result<Conversation, String> {
    let conversation = Conversation {
        id: uuid::Uuid::new_v4().to_string(),
        title: title.unwrap_or_else(|| "New Chat".to_string()),
        model_id: model_id.unwrap_or_else(|| "ollama:llama3".to_string()),
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
        archived: false,
    };
    let mut convos = db.lock().await;
    convos.push(conversation.clone());
    Ok(conversation)
}

#[tauri::command]
pub async fn list_conversations(
    db: State<'_, Mutex<Vec<Conversation>>>,
) -> Result<Vec<Conversation>, String> {
    let convos = db.lock().await;
    let mut result = convos.clone();
    result.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(result)
}

#[tauri::command]
pub async fn get_conversation(
    db: State<'_, Mutex<Vec<Conversation>>>,
    id: String,
) -> Result<Conversation, String> {
    let convos = db.lock().await;
    convos
        .iter()
        .find(|c| c.id == id)
        .cloned()
        .ok_or_else(|| "Conversation not found".to_string())
}

#[tauri::command]
pub async fn update_conversation(
    db: State<'_, Mutex<Vec<Conversation>>>,
    id: String,
    title: Option<String>,
    model_id: Option<String>,
    archived: Option<bool>,
) -> Result<Conversation, String> {
    let mut convos = db.lock().await;
    let convo = convos
        .iter_mut()
        .find(|c| c.id == id)
        .ok_or_else(|| "Conversation not found".to_string())?;

    if let Some(t) = title {
        convo.title = t;
    }
    if let Some(m) = model_id {
        convo.model_id = m;
    }
    if let Some(a) = archived {
        convo.archived = a;
    }
    convo.updated_at = chrono::Utc::now().to_rfc3339();
    Ok(convo.clone())
}

#[tauri::command]
pub async fn delete_conversation(
    db: State<'_, Mutex<Vec<Conversation>>>,
    messages_db: State<'_, Mutex<Vec<Message>>>,
    id: String,
) -> Result<(), String> {
    let mut convos = db.lock().await;
    let mut msgs = messages_db.lock().await;
    convos.retain(|c| c.id != id);
    msgs.retain(|m| m.conversation_id != id);
    Ok(())
}

#[tauri::command]
pub async fn get_messages(
    db: State<'_, Mutex<Vec<Message>>>,
    conversation_id: String,
) -> Result<Vec<Message>, String> {
    let msgs = db.lock().await;
    let mut result: Vec<Message> = msgs
        .iter()
        .filter(|m| m.conversation_id == conversation_id)
        .cloned()
        .collect();
    result.sort_by_key(|m| m.sort_order);
    Ok(result)
}

#[tauri::command]
pub async fn add_message(
    db: State<'_, Mutex<Vec<Message>>>,
    conversation_id: String,
    role: String,
    content: String,
) -> Result<Message, String> {
    let mut msgs = db.lock().await;
    let sort_order = msgs
        .iter()
        .filter(|m| m.conversation_id == conversation_id)
        .count() as i64;
    let message = Message {
        id: uuid::Uuid::new_v4().to_string(),
        conversation_id,
        role,
        content,
        token_count: None,
        created_at: chrono::Utc::now().to_rfc3339(),
        sort_order,
    };
    msgs.push(message.clone());
    Ok(message)
}
