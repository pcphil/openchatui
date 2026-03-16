pub mod migrations;

use tauri_plugin_sql::{Migration, MigrationKind};

pub fn get_migrations() -> Vec<Migration> {
    static DESCRIPTIONS: &[&str] = &[
        "create conversations table",
        "create messages table",
        "create attachments table",
        "create settings table",
        "create messages index",
    ];

    migrations::MIGRATIONS
        .iter()
        .zip(DESCRIPTIONS.iter())
        .enumerate()
        .map(|(i, (sql, desc))| Migration {
            version: (i + 1) as i64,
            description: desc,
            sql,
            kind: MigrationKind::Up,
        })
        .collect()
}
