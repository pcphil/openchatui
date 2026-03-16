mod commands;
mod db;
mod models;
mod providers;
pub mod sandbox;

use models::{Conversation, Message};
use providers::ProviderRegistry;
use sandbox::manager::SandboxManager;
use std::collections::HashMap;
use tokio::sync::Mutex;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(Vec::<Conversation>::new()))
        .manage(Mutex::new(Vec::<Message>::new()))
        .manage(Mutex::new(ProviderRegistry::new()))
        .manage(Mutex::new(HashMap::<String, String>::new()))
        .manage(Mutex::new(SandboxManager::new().expect("Failed to initialize SandboxManager")))
        .invoke_handler(tauri::generate_handler![
            commands::conversations::create_conversation,
            commands::conversations::list_conversations,
            commands::conversations::get_conversation,
            commands::conversations::update_conversation,
            commands::conversations::delete_conversation,
            commands::conversations::get_messages,
            commands::conversations::add_message,
            commands::chat::send_message,
            commands::models::list_models,
            commands::models::test_provider_connection,
            commands::settings::get_setting,
            commands::settings::set_setting,
            commands::settings::get_all_settings,
            commands::sandbox::create_sandbox,
            commands::sandbox::exec_in_sandbox,
            commands::sandbox::approve_proposal,
            commands::sandbox::reject_proposal,
            commands::sandbox::stop_sandbox,
            commands::sandbox::destroy_sandbox,
            commands::sandbox::get_sandbox_for_conversation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
