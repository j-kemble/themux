# Project Goals

> **Status:** Active | **Last updated:** 2026-05-08

## Vision

themux is a **native Linux terminal multiplexer for AI coding agents** — porting the core functionality of macOS [cmux](https://github.com/manaflow-ai/cmux) to Linux. It combines GPU-accelerated terminal rendering (Ghostty), an in-app browser (WebKitGTK), vertical workspace tabs, split panes, a notification system, and an extensive scripting API into one native GTK4 application built with Rust.

## Primary Goal

Give Linux developers the same "primitive, not a solution" terminal workspace that cmux provides on macOS: composable terminals, browser, notifications, workspaces, splits, and a CLI to control it all — purpose-built for running multiple AI coding agents in parallel.

## Scope

### In Scope (P0-P1)

- GPU-accelerated terminal via Ghostty GTK widget
- Vertical sidebar workspace tabs with metadata
- Split pane layout (horizontal/vertical)
- In-app browser via WebKitGTK with automation API
- Notification system (OSC sequences + desktop notifications)
- Full CLI with 80+ commands (cmux-compatible)
- Unix socket server with V2 JSON-RPC protocol
- Agent hooks: Claude Code, Codex, OpenCode, Hermes Agent
- Tmux compatibility layer
- SSH remote workspaces via cmuxd-remote
- Session persistence (auto-save, restore)
- Config system (cmux.json compatible)
- Keyboard shortcut customization
- cmuxd-remote Go daemon (reused as-is)
- Python test suite compatibility

### Out of Scope (P3+ / Not Planned)

- macOS-style glass/blur compositing effects
- Menu bar extra (system tray: P2)
- File explorer sidebar (P2)
- Task/process manager (P2)
- Cloud VM integration (P3, requires web platform)
- Markdown panel (P3)
- File preview panel (P3)
- Browser history import (P3)
- PostHog analytics (P3)
- Sparkle-style auto-updater (replaced by Flatpak)

## Non-Goals

- Feature-for-feature parity with macOS cmux — protocol compatibility matters, UI doesn't
- Electron or web-based UI — must be native GTK
- Proprietary licensing — GPL-3.0-or-later
- iOS/Android support

## Success Criteria

1. An AI agent (Claude Code, Codex) can be launched in a split pane with tmux shims
2. The Python test suite from cmux (`tests_v2/`) passes against themux socket
3. A developer can SSH into a remote machine and get a workspace with browser proxy
4. Session restore survives a quit and relaunch

## Related

- [[architecture]] — How it all fits together
- [[development-roadmap]] — When each goal ships
- [[design-decisions]] — Why Rust, why GTK4, why the protocol-first approach
- [[build-principles]] — How we build
