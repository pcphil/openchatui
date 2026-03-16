# OpenChatUI

A local-first desktop AI chat application built with Tauri v2, React, and TypeScript. Connect to local models via Ollama or cloud APIs from OpenAI, Anthropic, and Google — all from a single interface.

## Features

- **Multi-provider support** — Ollama (local), OpenAI, Anthropic, Google AI
- **Streaming responses** — Real-time token rendering via Tauri Channels
- **Markdown & code highlighting** — GFM support with syntax-highlighted code blocks and copy-to-clipboard
- **File attachments** — Attach images and documents; vision-capable models process images directly
- **Conversation management** — Create, rename, delete, and switch between conversations
- **Model switching** — Change models mid-session, grouped by provider
- **Dark/light theme** — Toggle between themes from settings
- **Local-first & private** — API keys stay on your device, sent directly to provider APIs
- **Keyboard shortcuts** — Ctrl+N (new chat), Ctrl+B (toggle sidebar), Ctrl+, (settings)
- **Lightweight** — ~3 MB installer, ~14 MB binary

## Quick Start

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) (stable toolchain)
- [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/) for your platform

### Development

```bash
# Install dependencies
npm install

# Run in development mode (launches app with hot reload)
npx tauri dev
```

### Build

```bash
# Build for production
npx tauri build
```

Outputs are in `src-tauri/target/release/bundle/`:
- **Windows**: `.msi` and `.exe` (NSIS) installers
- **macOS**: `.dmg` and `.app` bundle
- **Linux**: `.deb` and `.AppImage`

## Connecting to Providers

### Ollama (local, no API key needed)

1. Install [Ollama](https://ollama.com/)
2. Pull a model: `ollama pull llama3`
3. Ollama models appear automatically in the model selector

### Cloud Providers

1. Open **Settings** (gear icon or Ctrl+,)
2. Enter your API key for OpenAI, Anthropic, or Google AI
3. Click **Save**, then **Test** to verify the connection
4. New models appear in the model selector

## Project Structure

```
openchatui/
├── src/                          # React frontend
│   ├── components/
│   │   ├── chat/                 # ChatView, MessageBubble, InputBar, etc.
│   │   ├── layout/               # Sidebar, MainPanel
│   │   ├── settings/             # SettingsModal, ModelSelector
│   │   └── common/               # Button, Modal
│   ├── stores/                   # Zustand state (conversation, settings, UI)
│   ├── services/                 # Typed Tauri IPC wrappers
│   ├── hooks/                    # useKeyboardShortcuts
│   ├── types/                    # TypeScript interfaces
│   └── styles/                   # Tailwind + markdown styles
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── commands/             # Tauri commands (chat, conversations, models, settings)
│   │   ├── providers/            # LlmProvider trait + Ollama, OpenAI, Anthropic, Google
│   │   ├── db/                   # SQLite migrations
│   │   └── models.rs             # Shared data structures
│   ├── capabilities/             # Tauri v2 permission config
│   └── Cargo.toml
└── docs/                         # Documentation
```

See [docs/](docs/) for detailed documentation.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop framework | Tauri v2 |
| Frontend | React 19, TypeScript 5 |
| Bundler | Vite 6 |
| State management | Zustand 5 |
| Styling | Tailwind CSS 4 |
| Markdown | react-markdown, remark-gfm, rehype-highlight |
| Icons | lucide-react |
| HTTP (Rust) | reqwest (async streaming) |
| Database | SQLite via tauri-plugin-sql |

## License

See [LICENSE](LICENSE) for details.
