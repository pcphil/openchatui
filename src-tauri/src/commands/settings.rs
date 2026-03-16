use crate::providers::ProviderRegistry;
use std::collections::HashMap;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn get_setting(
    settings: State<'_, Mutex<HashMap<String, String>>>,
    key: String,
) -> Result<Option<String>, String> {
    let settings = settings.lock().await;
    Ok(settings.get(&key).cloned())
}

#[tauri::command]
pub async fn set_setting(
    settings: State<'_, Mutex<HashMap<String, String>>>,
    registry: State<'_, Mutex<ProviderRegistry>>,
    key: String,
    value: String,
) -> Result<(), String> {
    {
        let mut settings = settings.lock().await;
        settings.insert(key.clone(), value.clone());
    }

    // If setting an API key, configure the provider
    match key.as_str() {
        "openai_api_key" => {
            let mut reg = registry.lock().await;
            reg.configure_provider("openai", &value);
        }
        "anthropic_api_key" => {
            let mut reg = registry.lock().await;
            reg.configure_provider("anthropic", &value);
        }
        "google_api_key" => {
            let mut reg = registry.lock().await;
            reg.configure_provider("google", &value);
        }
        _ => {}
    }

    Ok(())
}

#[tauri::command]
pub async fn get_all_settings(
    settings: State<'_, Mutex<HashMap<String, String>>>,
) -> Result<HashMap<String, String>, String> {
    let settings = settings.lock().await;
    // Return settings but mask API keys
    let mut result = settings.clone();
    for key in ["openai_api_key", "anthropic_api_key", "google_api_key"] {
        if let Some(val) = result.get(key) {
            if val.len() > 8 {
                let masked = format!("{}...{}", &val[..4], &val[val.len() - 4..]);
                result.insert(key.to_string(), masked);
            }
        }
    }
    Ok(result)
}
