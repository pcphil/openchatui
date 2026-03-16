pub const MIGRATIONS: &[&str] = &[
    // Migration 1: Initial schema
    "CREATE TABLE IF NOT EXISTS conversations (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL DEFAULT 'New Chat',
        model_id TEXT NOT NULL DEFAULT 'ollama:llama3',
        created_at TEXT NOT NULL DEFAULT (datetime('now')),
        updated_at TEXT NOT NULL DEFAULT (datetime('now')),
        archived INTEGER NOT NULL DEFAULT 0
    );",
    "CREATE TABLE IF NOT EXISTS messages (
        id TEXT PRIMARY KEY,
        conversation_id TEXT NOT NULL,
        role TEXT NOT NULL CHECK(role IN ('user', 'assistant', 'system')),
        content TEXT NOT NULL,
        token_count INTEGER,
        created_at TEXT NOT NULL DEFAULT (datetime('now')),
        sort_order INTEGER NOT NULL DEFAULT 0,
        FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
    );",
    "CREATE TABLE IF NOT EXISTS attachments (
        id TEXT PRIMARY KEY,
        message_id TEXT NOT NULL,
        file_name TEXT NOT NULL,
        file_path TEXT NOT NULL,
        mime_type TEXT NOT NULL,
        file_size INTEGER NOT NULL DEFAULT 0,
        FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE
    );",
    "CREATE TABLE IF NOT EXISTS settings (
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL
    );",
    "CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages(conversation_id, sort_order);",
];
