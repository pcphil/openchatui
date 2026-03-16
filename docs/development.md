# Development Guide

## Prerequisites

- **Node.js** 18+ — [nodejs.org](https://nodejs.org/)
- **Rust** stable toolchain — [rustup.rs](https://rustup.rs/)
- **Tauri v2 system dependencies** — [platform-specific guide](https://v2.tauri.app/start/prerequisites/)

On Windows, you need:
- Microsoft Visual Studio C++ Build Tools
- WebView2 (pre-installed on Windows 10 1803+ and Windows 11)

## Getting Started

```bash
# Clone the repo
git clone <repo-url>
cd openchatui

# Install npm dependencies
npm install

# Start development (launches both Vite dev server and Tauri app)
npx tauri dev
```

The app will open with hot reload enabled. Frontend changes reflect instantly; Rust changes trigger a recompile.

## Project Layout

```
openchatui/
├── package.json          # npm scripts and frontend dependencies
├── vite.config.ts        # Vite config (dev server port 1420)
├── tsconfig.json         # TypeScript config (strict mode)
├── index.html            # HTML entry point
├── src/                  # React frontend
│   ├── main.tsx          # React entry point
│   ├── App.tsx           # Root component
│   ├── components/       # UI components
│   ├── stores/           # Zustand state stores
│   ├── services/         # Tauri IPC wrappers
│   ├── hooks/            # Custom React hooks
│   ├── types/            # TypeScript type definitions
│   └── styles/           # CSS (Tailwind + markdown styles)
├── src-tauri/            # Rust backend
│   ├── Cargo.toml        # Rust dependencies
│   ├── tauri.conf.json   # Tauri app config
│   ├── capabilities/     # Tauri v2 permissions
│   ├── icons/            # App icons
│   └── src/
│       ├── main.rs       # Rust entry point
│       ├── lib.rs        # Tauri builder setup, command registration
│       ├── models.rs     # Shared data structures
│       ├── commands/     # Tauri command handlers
│       ├── providers/    # LLM provider implementations
│       └── db/           # Database migrations
└── docs/                 # Documentation
```

## Common Tasks

### Adding a Tauri Command

1. Write the command in `src-tauri/src/commands/`:
   ```rust
   #[tauri::command]
   pub async fn my_command(arg: String) -> Result<String, String> {
       Ok(format!("Hello {}", arg))
   }
   ```

2. Register it in `src-tauri/src/lib.rs`:
   ```rust
   .invoke_handler(tauri::generate_handler![
       // ... existing commands
       commands::mymodule::my_command,
   ])
   ```

3. Add a typed wrapper in `src/services/tauriCommands.ts`:
   ```typescript
   export async function myCommand(arg: string): Promise<string> {
     return invoke("my_command", { arg });
   }
   ```

### Adding a React Component

Components live in `src/components/` organized by feature:
- `chat/` — Chat-related components
- `layout/` — Structural layout (sidebar, panels)
- `settings/` — Settings and configuration
- `common/` — Reusable primitives (Button, Modal)

### Styling

The project uses Tailwind CSS v4 with CSS custom properties for theming. Theme variables are defined in `src/styles/globals.css`:

- `--bg-primary`, `--bg-secondary`, `--bg-tertiary` — Background colors
- `--text-primary`, `--text-secondary` — Text colors
- `--border-color` — Border color
- `--accent`, `--accent-hover` — Accent colors
- `--danger`, `--danger-hover` — Danger/error colors

Use these via `bg-[var(--bg-primary)]`, `text-[var(--text-secondary)]`, etc.

The `.dark` class on `<html>` switches to dark theme values.

## Building

```bash
# TypeScript check
npx tsc --noEmit

# Frontend-only build
npm run build

# Full Tauri build (frontend + Rust + bundling)
npx tauri build
```

Build output:
- Binary: `src-tauri/target/release/openchatui.exe`
- MSI: `src-tauri/target/release/bundle/msi/OpenChatUI_0.1.0_x64_en-US.msi`
- NSIS: `src-tauri/target/release/bundle/nsis/OpenChatUI_0.1.0_x64-setup.exe`

## Capabilities / Permissions

Tauri v2 uses a capability system. Permissions are declared in `src-tauri/capabilities/default.json`. If you use a new Tauri plugin or API, add the required permission there:

```json
{
  "permissions": [
    "core:default",
    "sql:default",
    "sql:allow-execute",
    "sql:allow-select",
    "fs:default",
    "dialog:default",
    "dialog:allow-open"
  ]
}
```

## Troubleshooting

### White screen on launch
- Check that the Vite dev server started (port 1420)
- Open DevTools with Ctrl+Shift+I to check console errors

### Command returns undefined
- Verify the command is registered in `generate_handler![]` in `lib.rs`
- Argument names must match: camelCase in TypeScript, snake_case in Rust

### Rust compilation slow
- First build downloads and compiles all crate dependencies (~580 crates)
- Subsequent builds are incremental and much faster
- Release builds (`tauri build`) take longer due to optimizations
