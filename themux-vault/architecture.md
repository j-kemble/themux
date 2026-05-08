# Architecture

> **Status:** Living document | **Last updated:** 2026-05-08

## System Overview

```
┌──────────────────────────────────────────────────────┐
|                   GTK4 UI Layer                       │
│  Sidebar │ Split Panes │ Tab Bar │ Browser Panels     │
│  libghostty-vt Terminal (VT emulation engine)         │
├──────────────────────────────────────────────────────┤
│               Rust Application Core                   │
│  TerminalWidget │ Workspace Manager │ Socket Server   │
│  Config Manager │ Session Persist │ Notifications     │
│  Agent Hooks                                          │
├──────────────────────────────────────────────────────┤
│                   IPC Layer                           │
│  Unix Domain Socket (JSON-RPC V2) — cmux-compatible   │
├──────────────────────────────────────────────────────┤
│           Cross-Platform Components                   │
│  cmuxd-remote (Go) │ CLI (Rust) │ Tests (Python)      │
└──────────────────────────────────────────────────────┘
```

## Crate Dependency Graph

```
themux-app (GTK4 binary)
  ├── themux-core (workspace model, config, session, layout)
  ├── themux-socket (Unix socket server, V2 protocol)
  │   └── themux-core
  ├── ghostty-sys (libghostty-vt FFI bindings)
  └── themux-notify (desktop notifications)
      └── themux-core

themux-cli (CLI binary)
  ├── themux-core
  ├── themux-socket
  └── themux-agent (agent hooks, tmux compat)
      └── themux-core
```

## Domain Model

```
Window
 └── WorkspaceManager
      └── Workspace[] (ordered, one selected)
           ├── id, title, description, color, pinned
           ├── working_directory, git_branch, listening_ports
           ├── remote: RemoteConfig (SSH or WebSocket transport)
           ├── metadata: status, progress, log_entries
           └── layout: SplitNode (recursive split tree)
                ├── Split (direction, divider_position, first, second)
                └── Pane (leaf)
                     └── Panel[] (focused_panel_index)
                          ├── TerminalPanel
                          ├── BrowserPanel
                          ├── MarkdownPanel
                          └── FilePreviewPanel
```

## Data Flow

1. **User → GTK4 UI** → Updates WorkspaceManager state → UI re-renders
2. **User → Keyboard** → KeyboardShortcutSettings lookup → Action dispatch → WorkspaceManager
3. **Agent → CLI** → Socket (V2 JSON-RPC) → themux-socket dispatch → WorkspaceManager
4. **Agent → Tmux shim** → CLI (`themux __tmux-compat`) → Socket → themux-socket
5. **Terminal → OSC seq** → themux-notify OSC parser → NotificationStore → Desktop notification
6. **Remote:** SSH → cmuxd-remote (Go) → WebSocket PTY → Socket relay
7. **Session:** Auto-save timer (8s) → AppSessionSnapshot JSON → `~/.local/share/themux/session.json`
8. **Config:** `~/.config/themux/config.json` → Config loader → merge with `.themux.json` (project-level)

## Key Design Patterns

### Protocol-First Architecture

The V2 JSON-RPC protocol is the **stable contract**. The GTK4 UI, CLI, agent hooks, and Python tests all speak it. This means:
- The CLI can be tested without the GUI
- The Python test suite from cmux validates themux
- Agent hooks work the same on macOS cmux and Linux themux
- The UI can be rebuilt without breaking integrations

### Pure Core Crate

`themux-core` has **zero GTK dependencies**. It contains only data models, business logic, and persistence. This makes it:
- Testable with `cargo test` (no display needed)
- Reusable by both the GUI and CLI
- Easy to reason about

### Submodule Separation

cmuxd-remote will remain a git submodule when added:
- cmuxd-remote: remote daemon (Go, from cmux source)

Ghostty was originally planned as a submodule but is now **vendored source** trimmed to only the libghostty-vt library (terminal emulation core). This avoids the complex Zig build dependency chain and lets us build exactly what we need with `zig build -Demit-lib-vt`.

## Related

- [[project-structure]] — File-by-file map
- [[design-decisions]] — Why these choices
- [[protocol-v2]] — Wire protocol details
- [[development-roadmap]] — Build order
