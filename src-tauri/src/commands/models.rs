use crate::models::Model;
use crate::providers::ProviderRegistry;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn list_models(
    registry: State<'_, Mutex<ProviderRegistry>>,
) -> Result<Vec<Model>, String> {
    let provider_names: Vec<String>;
    {
        let reg = registry.lock().await;
        provider_names = reg.providers().iter().map(|s| s.to_string()).collect();
    }

    let mut all_models = Vec::new();
    for provider_name in &provider_names {
        let reg = registry.lock().await;
        if let Some(provider) = reg.get(provider_name) {
            match provider.list_models().await {
                Ok(models) => all_models.extend(models),
                Err(e) => {
                    log::warn!("Failed to list models for {}: {}", provider_name, e);
                }
            }
        }
    }

    Ok(all_models)
}

#[tauri::command]
pub async fn test_provider_connection(
    registry: State<'_, Mutex<ProviderRegistry>>,
    provider: String,
) -> Result<bool, String> {
    let reg = registry.lock().await;
    let prov = reg
        .get(&provider)
        .ok_or_else(|| format!("Provider '{}' not found", provider))?;
    prov.test_connection().await.map_err(|e| e.to_string())
}
