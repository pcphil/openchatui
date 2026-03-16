# Provider Guide

OpenChatUI supports four LLM providers. This document covers setup, supported models, and implementation details for each.

## Ollama (Local)

**No API key required.** Ollama runs models locally on your machine.

### Setup

1. Install Ollama from [ollama.com](https://ollama.com/)
2. Pull a model:
   ```bash
   ollama pull llama3
   ollama pull mistral
   ollama pull llava    # Vision-capable
   ```
3. Ollama runs at `http://localhost:11434` by default
4. Models appear automatically in the OpenChatUI model selector

### Supported Features

- Streaming (NDJSON format)
- Vision (models with `llava` or `vision` in the name)
- Dynamic model list (fetched from `/api/tags`)

### API Endpoints Used

| Endpoint | Purpose |
|----------|---------|
| `GET /api/tags` | List installed models |
| `POST /api/chat` | Chat completion (streaming) |

---

## OpenAI

### Setup

1. Get an API key from [platform.openai.com/api-keys](https://platform.openai.com/api-keys)
2. Open Settings in OpenChatUI (Ctrl+,)
3. Paste your key and click Save
4. Click Test to verify

### Supported Models

Models are fetched dynamically from the API. Filtered to models starting with `gpt-` or `o`.

Common models:
- `gpt-4o` — Multimodal, fast
- `gpt-4o-mini` — Cheaper, still capable
- `o1`, `o3`, `o4-mini` — Reasoning models

### Supported Features

- Streaming (SSE format)
- Vision (GPT-4 and o-series models)
- Image attachments sent as base64 data URLs

### API Endpoints Used

| Endpoint | Purpose |
|----------|---------|
| `GET /v1/models` | List available models |
| `POST /v1/chat/completions` | Chat completion (streaming) |

---

## Anthropic

### Setup

1. Get an API key from [console.anthropic.com](https://console.anthropic.com/)
2. Open Settings in OpenChatUI (Ctrl+,)
3. Paste your key and click Save
4. Click Test to verify

### Supported Models

Hardcoded model list (Anthropic's API doesn't have a public model list endpoint):

- `claude-opus-4-20250514` — Most capable
- `claude-sonnet-4-20250514` — Balanced
- `claude-haiku-4-20250414` — Fast and affordable

### Supported Features

- Streaming (SSE format)
- Vision (all Claude models)
- Image attachments sent as base64 with media type
- Default max_tokens: 4096

### API Endpoints Used

| Endpoint | Purpose |
|----------|---------|
| `POST /v1/messages` | Chat completion (streaming) |

### Headers

- `x-api-key` — API key
- `anthropic-version` — `2023-06-01`

---

## Google AI (Gemini)

### Setup

1. Get an API key from [aistudio.google.com/apikey](https://aistudio.google.com/apikey)
2. Open Settings in OpenChatUI (Ctrl+,)
3. Paste your key and click Save
4. Click Test to verify

### Supported Models

Hardcoded model list:

- `gemini-2.5-pro` — Most capable
- `gemini-2.5-flash` — Fast, good quality
- `gemini-2.0-flash` — Fastest

### Supported Features

- Streaming (SSE format)
- Vision (all Gemini models)
- Image attachments sent as inline_data with base64

### API Endpoints Used

| Endpoint | Purpose |
|----------|---------|
| `GET /v1beta/models` | List models (used for connection test) |
| `POST /v1beta/models/{model}:streamGenerateContent` | Chat completion (streaming) |

### Notes

- Google uses `role: "model"` instead of `role: "assistant"` — the provider handles this mapping automatically.
- The API key is passed as a query parameter (`?key=...`), not as a header.

---

## Adding a New Provider

To add a new LLM provider:

1. Create `src-tauri/src/providers/yourprovider.rs`
2. Implement the `LlmProvider` trait:
   ```rust
   #[async_trait]
   impl LlmProvider for YourProvider {
       async fn list_models(&self) -> ProviderResult<Vec<Model>>;
       async fn stream_completion(&self, model: &str, messages: Vec<ChatMessage>, attachments: Vec<AttachmentData>)
           -> ProviderResult<BoxStream<'static, ProviderResult<String>>>;
       async fn test_connection(&self) -> ProviderResult<bool>;
       fn provider_name(&self) -> &str;
   }
   ```
3. Add `pub mod yourprovider;` to `src-tauri/src/providers/mod.rs`
4. Add a `configure_provider` case in `ProviderRegistry`
5. Add the API key setting handler in `commands/settings.rs`

The model ID format is `"yourprovider:model-name"`. The `send_message` command automatically routes to the correct provider based on the prefix.
