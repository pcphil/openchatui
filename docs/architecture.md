# Architecture

## Overview

OpenChatUI is a desktop application built with [Tauri v2](https://v2.tauri.app/). The frontend is a React SPA that communicates with a Rust backend via Tauri's IPC system.

```
┌───────────────────────────────────────────────────────────┐
│                     Tauri v2 Shell                        │
│                                                           │
│  ┌─────────────────────┐   ┌───────────────────────────┐  │
│  │  React Frontend     │   │  Rust Backend             │  │
│  │                     │   │                           │  │
│  │  Sidebar            │   │  Tauri Commands           │  │
│  │  ChatView           │   │  ├─ send_message          │  │
│  │  SettingsModal       │   │  ├─ conversation CRUD    │  │
│  │  ModelSelector      │   │  ├─ list_models           │  │
│  │                     │   │  └─ settings get/set      │  │
│  │  Zustand Stores     │   │                           │  │
│  │  ├─ conversation    │   │  Provider Layer           │  │
│  │  ├─ settings        │   │  ├─ Ollama               │  │
│  │  └─ ui              │   │  ├─ OpenAI               │  │
│  │                     │   │  ├─ Anthropic             │  │
│  └─────────┬───────────┘   │  └─ Google               │  │
│            │               │                           │  │
│       invoke() /           │  In-memory storage        │  │
│       Channel  ◄──────────►│  (Vec<Conversation>, etc) │  │
│                            └───────────────────────────┘  │
└───────────────────────────────────────────────────────────┘
```

## IPC

Tauri v2 provides two IPC mechanisms, both used in this app:

- **`invoke()`** — Request/response. Used for conversation CRUD, settings, and model listing.
- **Channels** — Unidirectional streaming from Rust to frontend. Used for streaming LLM tokens during chat.

### Streaming Flow

1. Frontend calls `invoke('send_message', { ..., onEvent: channel })` with a `Channel<StreamEvent>`
2. Rust resolves the provider from the model ID (`"provider:model"` format)
3. Rust calls `provider.stream_completion()` which returns an async stream
4. Each chunk is sent to the frontend via `channel.send(StreamEvent::Token(text))`
5. On completion: `StreamEvent::Done(full_text)`, on error: `StreamEvent::Error(msg)`
6. The Zustand conversation store appends tokens in real-time

### StreamEvent enum

```rust
#[serde(tag = "event", content = "data")]
enum StreamEvent {
    Token(String),   // Partial token
    Done(String),    // Full completed response
    Error(String),   // Error message
}
```

## Frontend Architecture

### State Management

Three Zustand stores manage all frontend state:

| Store | Purpose | Key State |
|-------|---------|-----------|
| `conversationStore` | Conversations and messages | `conversations`, `messages`, `streamingContent`, `isStreaming` |
| `settingsStore` | Provider config, model selection, theme | `models`, `selectedModelId`, `apiKeys`, `theme` |
| `uiStore` | UI toggles | `sidebarOpen` |

### Component Tree

```
App
├── Sidebar
│   ├── New Chat button
│   ├── Conversation list (with inline rename/delete)
│   └── Settings button
├── MainPanel
│   ├── WelcomeScreen (no active conversation)
│   └── ChatView (active conversation)
│       ├── ModelSelector
│       ├── Message list
│       │   ├── MessageBubble (completed messages)
│       │   └── StreamingBubble (in-progress response)
│       └── InputBar
│           ├── File attach button
│           ├── Textarea
│           └── Send/Stop button
└── SettingsModal
    ├── Theme toggle
    └── API key inputs (with test connection)
```

## Backend Architecture

### Provider Abstraction

All LLM providers implement the `LlmProvider` trait:

```rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn list_models(&self) -> ProviderResult<Vec<Model>>;
    async fn stream_completion(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
        attachments: Vec<AttachmentData>,
    ) -> ProviderResult<BoxStream<'static, ProviderResult<String>>>;
    async fn test_connection(&self) -> ProviderResult<bool>;
    fn provider_name(&self) -> &str;
}
```

The `ProviderRegistry` manages provider instances. Ollama is always registered. Cloud providers are registered when the user saves an API key.

### Managed State

The Tauri app manages four pieces of state via `tokio::sync::Mutex`:

- `Vec<Conversation>` — All conversations
- `Vec<Message>` — All messages
- `ProviderRegistry` — Provider instances
- `HashMap<String, String>` — Settings key-value store

### Commands

| Command | Description |
|---------|-------------|
| `create_conversation` | Create a new conversation |
| `list_conversations` | List all conversations (sorted by updated_at desc) |
| `get_conversation` | Get a single conversation by ID |
| `update_conversation` | Update title, model, or archived status |
| `delete_conversation` | Delete conversation and its messages |
| `get_messages` | Get messages for a conversation |
| `add_message` | Add a message to a conversation |
| `send_message` | Send a message and stream the AI response |
| `list_models` | List available models across all providers |
| `test_provider_connection` | Test if a provider API key is valid |
| `get_setting` | Get a single setting value |
| `set_setting` | Set a setting (auto-configures providers for API keys) |
| `get_all_settings` | Get all settings (API keys are masked) |

## Model ID Format

Models are identified by `"provider:model"` strings:

- `ollama:llama3`
- `openai:gpt-4o`
- `anthropic:claude-sonnet-4-20250514`
- `google:gemini-2.5-pro`

The `send_message` command splits this to resolve the provider and pass the model name.
