# themux Architecture

## Overview

themux is a native Linux terminal multiplexer designed for AI coding agent orchestration, porting the core functionality of macOS cmux to Linux using Rust and GTK4.

## Technology Stack

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| UI toolkit | GTK4 + libadwaita | Native Linux look, Ghostty uses GTK |
| Application core | Rust (tokio async) | Memory safety, strong type system, good GTK bindings |
| Terminal rendering | Ghostty (Zig) via GTK widget | GPU-accelerated, production-tested |
| In-app browser | WebKitGTK 6.0 | Native WebKit integration |
| IPC | Unix domain sockets (JSON-RPC V2) | Compatible with cmux protocol |
| Remote daemon | Go (cmuxd-remote) | Cross-platform, unchanged |
| Web platform | TypeScript / Next.js | Unchanged from cmux |
| Build system | Cargo + Meson + Makefile | Standard Rust + GTK toolchain |

## Crate Dependency Graph

```
themux-app (GTK4 binary)
  ├── themux-core (workspace model, config, session, layout)
  ├── themux-socket (Unix socket server, V2 protocol)
  │   └── themux-core
  └── themux-notify (desktop notifications)
      └── themux-core

themux-cli (CLI binary)
  ├── themux-core
  ├── themux-socket
  └── themux-agent (agent hooks, tmux compat)
      └── themux-core
```

## Data Flow

1. User interacts with GTK4 UI (sidebar clicks, keyboard shortcuts, split drags)
2. UI updates themux-core WorkspaceManager state
3. State changes trigger socket events broadcast to connected CLI clients
4. CLI sends commands via Unix socket → themux-socket dispatches to core
5. Agent hooks (Claude Code, etc.) use tmux shims → CLI → socket → core
6. Remote workspaces: SSH → cmuxd-remote (Go) → WebSocket → socket relay

## Session Lifecycle

1. Launch: `themux-app` starts socket server, loads config, restores session
2. Runtime: WorkspaceManager handles CRUD, split tree operations
3. Auto-save: Every 8 seconds, SessionPersistence snapshots all state
4. Quit: Session saved to `session.json`, previous snapshot saved to `session-previous.json`
5. Restore: `session.json` loaded, workspaces recreated, agent sessions resumed

## Protocol Compatibility

themux implements cmux's V2 JSON-RPC protocol. Method dispatch table:

See `crates/themux-socket/src/protocol/v2.rs` for the full list.

## Key Differences from macOS cmux

| Aspect | macOS cmux | themux (Linux) |
|--------|-----------|----------------|
| Language | Swift | Rust |
| UI layer | AppKit + SwiftUI | GTK4 |
| Terminal render | Ghostty via Metal | Ghostty via GL/Vulkan |
| Browser | WKWebView | WebKitGTK |
| Notifications | UNUserNotificationCenter | libnotify |
| Auto-update | Sparkle | Flatpak |
| Settings store | NSUserDefaults | JSON files |
| Code signing | codesign | N/A |
| Clipboard | NSPasteboard | GTK clipboard |
| Font rendering | CoreText | FreeType (via Ghostty) |
