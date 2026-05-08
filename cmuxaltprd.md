# cmux-alt: Linux Port PRD

## Product Requirements Document

**Author:** Josh (with Hermes Agent "Pip")
**Date:** 2026-05-08
**Status:** Draft v1.0
**Source:** Deep analysis of [github.com/manaflow-ai/cmux](https://github.com/manaflow-ai/cmux)

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [What is cmux?](#2-what-is-cmux)
3. [Current Architecture (macOS)](#3-current-architecture-macos)
4. [Complete Feature Catalog](#4-complete-feature-catalog)
5. [Linux Port Strategy](#5-linux-port-strategy)
6. [Recommended Technical Stack](#6-recommended-technical-stack)
7. [Phased Implementation Plan](#7-phased-implementation-plan)
8. [Risk Assessment](#8-risk-assessment)
9. [Appendix: Source Code Map](#9-appendix-source-code-map)

---

## 1. Executive Summary

**cmux** is a native macOS terminal multiplexer built on Ghostty's GPU-accelerated rendering engine, designed specifically for developers running multiple AI coding agents in parallel. It combines a terminal emulator, in-app browser, notification system, workspace management, and extensive automation APIs into a single AppKit/SwiftUI application.

**cmux-alt** is the proposed Linux port of cmux's core functionality. The macOS app is deeply coupled to Apple platform APIs (AppKit, WKWebView, Metal, CoreText, Carbon, IOSurface) across ~13,000+ lines of Swift. A Linux port requires a new UI layer while preserving the architectural patterns, CLI/socket API, WebSocket relay protocol, and cross-platform components.

**Key insight:** The Ghostty rendering engine itself supports Linux (via GTK and OpenGL/Vulkan backends). The cmuxd-remote Go daemon, the WebSocket lease protocol, the JSON-RPC socket API (V2), and the web platform (Next.js on Vercel) are all cross-platform. The port effort is primarily about replacing the AppKit/SwiftUI UI layer.

---

## 2. What is cmux?

### 2.1 Core Value Proposition

cmux is a "terminal multiplexer plus" -- it combines what you'd traditionally get from tmux + a terminal emulator + notification manager + browser + scripting API into one cohesive application. Its killer feature is **AI agent orchestration**: it detects agent waiting states (Claude Code, Codex, OpenCode, Hermes, etc.), surfaces notifications with context, and lets you jump directly to the relevant pane.

### 2.2 Design Philosophy

As stated in the project's "Zen of cmux":

- cmux is a **primitive, not a solution** -- it gives you terminals, browser, notifications, workspaces, splits, tabs, and a CLI to control them
- It is **not prescriptive** about how developers work
- Composability over walled-garden workflows
- Native performance over Electron

### 2.3 Target Users

- Developers running multiple AI coding agents (Claude Code, Codex, OpenCode) in parallel
- Power terminal users who need split panes, workspaces, and session persistence
- Developers who want a scriptable terminal environment with browser integration
- Remote developers using SSH-heavy workflows

---

## 3. Current Architecture (macOS)

### 3.1 Codebase Overview

| Component | Language | Lines (approx) | Role |
|-----------|----------|----------------|------|
| `Sources/` | Swift | ~200 files, ~80K+ lines | Main macOS app: UI, terminal, browser, workspaces, settings |
| `CLI/` | Swift | ~16 files, ~8K lines | CLI executable for socket-based control |
| `Packages/` | Swift (SPM) | ~30 files | Modular packages: AgentVault, AgentLaunch, AuthCore, Workstream, DebugLog, PasteboardFidelity |
| `web/` | TypeScript | ~150 files | Next.js 16 platform: marketing, docs, Cloud VM API, settings schema |
| `daemon/remote/` | Go | ~12 files | cmuxd-remote: WebSocket RPC + PTY daemon for SSH/cloud workspaces |
| `ghostty/` | Zig/C (submodule) | vendored | Ghostty terminal engine (forked, built as xcframework) |
| `scripts/` | Bash | ~20 files | Build, reload, code-sign, test automation |
| `cmuxTests/` | Swift | ~100+ files | Unit + integration tests |
| `tests/`, `tests_v2/` | Python | ~40+ files | End-to-end socket API tests |

### 3.2 Core Domain Model

```
Window
 └── TabManager (manages workspaces within a window)
      └── Workspace[] (ordered list, one selected)
           ├── title, description, color, pinned state
           ├── git branch, PR status, listening ports
           ├── log entries, progress state
           ├── remote configuration (SSH transport)
           └── BonsplitController (recursive split tree)
                ├── Split (horizontal/vertical divider)
                │    ├── first: Pane | Split
                │    └── second: Pane | Split
                └── Pane (leaf container)
                     └── Panel[] (surface tabs within a pane)
                          ├── TerminalPanel (libghostty-backed)
                          ├── BrowserPanel (WKWebView-backed)
                          ├── MarkdownPanel
                          └── FilePreviewPanel
```

**Naming convention:** "Tab" = "Workspace" (legacy typealias). "Surface" = "Panel" (legacy naming). The hierarchy is: Window > Workspace > Pane > Panel/Surface.

### 3.3 Technology Stack Layers

```
┌─────────────────────────────────────────────┐
│  SwiftUI (Settings, menus, command palette)  │
├─────────────────────────────────────────────┤
│  AppKit (NSWindow, NSView, NSEvent, NSMenu)  │
├──────────────┬──────────────┬────────────────┤
│  GhosttyKit  │   WKWebView  │  SwiftTerm     │
│  (Metal GPU) │  (in-app     │  (terminal     │
│  terminal    │   browser)   │   emulation)   │
├──────────────┴──────────────┴────────────────┤
│  Unix Domain Socket (JSON-RPC V1 + V2)       │
├──────────────────────────────────────────────┤
│  CLI (cmux swift binary)                     │
├──────────────────────────────────────────────┤
│  cmuxd-remote (Go) | WebSocket RPC + PTY     │
├──────────────────────────────────────────────┤
│  Web Platform (Next.js/Vercel/Postgres)      │
└──────────────────────────────────────────────┘
```

---

## 4. Complete Feature Catalog

### 4.1 Core Terminal Features

| Feature | macOS Implementation | Priority for Linux | Notes |
|---------|---------------------|-------------------|-------|
| GPU-accelerated terminal rendering | libghostty via Metal | **P0** | Ghostty already supports Linux via GL/Vulkan |
| Multiple terminal instances per workspace | GhosttyTerminalView (13K lines) | **P0** | Core function |
| Font rendering (ligatures, CJK, emoji) | CoreText via Ghostty | **P0** | FreeType/Fontconfig on Linux |
| Scrollback (configurable, up to 4000 lines) | ghostty_surface scrollback | **P0** | Built into Ghostty |
| Copy/paste (rich text, shell escaping) | NSPasteboard | **P0** | GTK clipboard or wl-clipboard/xclip |
| Font size zoom (Cmd+/-) | GhosttyConfig.fontSize | **P1** | Keybind mapping |
| Clear scrollback (Cmd+K) | ghostty_surface clear | **P1** | Keybind mapping |
| Ghostty config compatibility | Reads ~/.config/ghostty/config | **P0** | Same path on Linux |
| Theme support (light/dark, catppuccin, etc.) | Ghostty theme system | **P0** | Same on Linux |

### 4.2 Workspace & Tab Management

| Feature | macOS Implementation | Priority for Linux | Notes |
|---------|---------------------|-------------------|-------|
| Vertical sidebar workspace tabs | SidebarState + SidebarBonsplitTab | **P0** | Core differentiator |
| Workspace CRUD (create, rename, close, reorder) | Workspace + TabManager | **P0** | |
| Workspace pinning | SidebarState pin API | **P1** | |
| Workspace color coding (16 colors + custom) | WorkspaceTabColorResolution | **P1** | |
| Workspace descriptions (markdown) | workspace description field | **P2** | |
| Drag-and-drop workspace reorder | SidebarBonsplitTabWorkspaceDropOverlay | **P2** | |
| Sidebar toggle (Cmd+B) | Sidebar visibility state | **P1** | |
| Sidebar metadata display (git, ports, PRs) | SidebarPortDisplayText, etc. | **P1** | |
| Numbered workspace navigation (Cmd+1-9) | keyboard shortcut routing | **P1** | |

### 4.3 Split Panes

| Feature | macOS Implementation | Priority for Linux | Notes |
|---------|---------------------|-------------------|-------|
| Horizontal split (Cmd+D) | Bonsplit package | **P0** | |
| Vertical split (Cmd+Shift+D) | Bonsplit package | **P0** | |
| Directional pane focus (Opt+Cmd+arrows) | Pane focus routing | **P0** | |
| Equalize splits | equalizeSplits support | **P1** | |
| Split resize via mouse drag | Bonsplit divider drag | **P1** | |
| Zoom/focus single pane | toggle split zoom | **P2** | |
| Surface tab bar within panes | Bonsplit tab strip | **P1** | Multiple surfaces per pane |

### 4.4 In-App Browser

| Feature | macOS Implementation | Priority for Linux | Notes |
|---------|---------------------|-------------------|-------|
| Full in-app browser as panel type | WKWebView (CmuxWebView) | **P0** | WebKitGTK on Linux |
| Address bar with search engines | BrowserOmnibar | **P1** | |
| Back/forward/reload navigation | WKNavigationDelegate | **P1** | |
| Browser automation API (agent-browser port) | Socket V2 browser.* methods | **P0** | Scriptable via socket |
| JavaScript evaluation | browser.eval / browser.eval_async | **P0** | |
| Screenshot capture | browser.screenshot | **P1** | |
| DOM element querying + interaction | browser.element, click, type, scroll | **P0** | |
| Multiple browser profiles (isolated storage) | WKWebViewConfiguration per profile | **P2** | |
| Developer tools / JS console | WKWebView inspector | **P2** | |
| Browser history import | Import from Chrome, Firefox, etc. | **P3** | |
| WebAuthn/FIDO2 | Platform authenticator passthrough | **P3** | |

### 4.5 Notification System

| Feature | macOS Implementation | Priority for Linux | Notes |
|---------|---------------------|-------------------|-------|
| OSC 9/99/777 terminal sequence detection | TerminalNotificationCallerResolver | **P0** | Standard terminal sequences |
| Notification store (per-workspace read/unread) | TerminalNotificationStore | **P0** | |
| Blue notification rings on panes/sidebar | Accent color ring overlay | **P1** | |
| Jump to latest unread (Cmd+Shift+U) | jumpToLatestUnread() | **P0** | |
| Desktop notifications | UNUserNotificationCenter | **P0** | libnotify / D-Bus on Linux |
| Notification sounds (system + custom) | NSSound | **P2** | |
| Custom notification commands | Shell command on notification | **P2** | |
| Notification panel/sidebar view | NotificationsPage | **P1** | |
| Menu bar notification count | MenuBarExtraController | **P2** | System tray on Linux |

### 4.6 Agent AI Integration

| Feature | macOS Implementation | Priority for Linux | Notes |
|---------|---------------------|-------------------|-------|
| Agent session resume (Claude Code, Codex, etc.) | RestorableAgentSession | **P0** | |
| Agent hooks (tmux shim injection) | CLI + tmux compat layer | **P0** | |
| Claude Code Teams (teammate mode) | CMUXCLI+TmuxCompatSupport | **P0** | |
| OpenCode integration (omo) | oh-my-opencode plugin | **P1** | |
| Codex integration (omx) | oh-my-codex | **P1** | |
| Hermes Agent integration | CMUXCLI+HermesAgentHooks | **P1** | |
| Feed/workstream (blocking hooks) | CMUXWorkstream + FeedCoordinator | **P2** | Agent permission requests |
| Agent vault (custom agent registration) | CMUXAgentVault config | **P2** | |
| Agent process tracking (kqueue) | VaultAgentProcessScanner | **P2** | /proc scanning on Linux |
| Shell activity state | idle/active/working tracking | **P3** | |

### 4.7 CLI & Socket API

| Feature | macOS Implementation | Priority for Linux | Notes |
|---------|---------------------|-------------------|-------|
| Unix domain socket (JSON-RPC V2) | TerminalController socket | **P0** | Already cross-platform |
| All workspace/surface/pane CRUD commands | CLI/*.swift full command set | **P0** | ~80+ commands |
| Browser automation via socket | V2 browser.* methods | **P0** | |
| tmux compatibility layer | __tmux-compat shim | **P0** | |
| Text/keyboard injection (send, send-key) | V2 surface.send_text/send_key | **P0** | |
| Screen reading (read-screen) | V2 surface.read_text | **P1** | |
| File/directory open via socket | cmux_open command | **P1** | |
| Event bus (streaming) | CmuxEventBus + events.stream | **P2** | |
| TCP relay for remote workspaces | Socket relay with HMAC auth | **P0** | Essential for remote |

### 4.8 Remote & SSH Workspaces

| Feature | macOS Implementation | Priority for Linux | Notes |
|---------|---------------------|-------------------|-------|
| SSH workspace creation | cmux ssh user@host | **P0** | |
| Remote daemon bootstrap | cmuxd-remote binary upload | **P0** | Go binary, already cross-platform |
| Remote PTY via WebSocket | cmuxd-remote ws_pty.go | **P0** | |
| Browser proxy through remote | WorkspaceRemoteProxyBroker | **P1** | |
| Remote port forwarding | ssh -L for detected ports | **P1** | |
| Cloud VM integration (E2B, Freestyle) | CloudVMActionLauncher | **P2** | Requires web platform |
| Remote file drop/upload | Image/file transfer via SCP | **P2** | |
| Remote workspace reconnect | Exponential backoff reconnection | **P1** | |

### 4.9 Session Persistence

| Feature | macOS Implementation | Priority for Linux | Notes |
|---------|---------------------|-------------------|-------|
| Auto-save every 8 seconds | SessionPersistence autosave timer | **P0** | |
| Restore window/workspace/pane layout | AppSessionSnapshot JSON | **P0** | |
| Restore working directories | Per-panel cwd | **P0** | |
| Terminal scrollback restore | CMUX_RESTORE_SCROLLBACK_FILE env | **P1** | |
| Browser URL + history restore | Browser snapshot | **P1** | |
| Agent session resume on relaunch | RestorableAgentSession commands | **P0** | |
| Manual "Reopen Previous Session" | Cmd+Shift+O | **P1** | |

### 4.10 Settings & Configuration

| Feature | macOS Implementation | Priority for Linux | Notes |
|---------|---------------------|-------------------|-------|
| cmux.json config file | ~/.config/cmux/cmux.json | **P0** | |
| Keyboard shortcut customization | KeyboardShortcutSettings | **P0** | |
| 80+ configurable shortcut actions | KeyboardShortcutSettings.Action enum | **P0** | |
| Appearance (light/dark/system) | AppearanceSettings | **P1** | |
| Custom commands & actions | CmuxConfig actions/commands | **P1** | |
| Project-level .cmux.json | Action trust/authorization system | **P2** | |
| Settings UI (searchable) | Settings window with navigation | **P2** | |

### 4.11 Additional Features

| Feature | Priority | Notes |
|---------|----------|-------|
| File explorer in sidebar | **P2** | Directory tree with git status |
| Task/process manager | **P2** | Per-workspace CPU/memory monitoring |
| Markdown panel | **P2** | Render .md files as panels |
| File preview panel | **P3** | Quick look for various file types |
| Global hotkey (show/hide) | **P1** | System-wide keyboard shortcut |
| Command palette | **P1** | Fuzzy search across all commands |
| Welcome/onboarding screen | **P3** | |
| PostHog analytics | **P3** | Cross-platform SDK |
| Sentry crash reporting | **P1** | Cross-platform SDK |
| Auto-updater | **P2** | Flatpak/AppImage or custom |

---

## 5. Linux Port Strategy

### 5.1 What Survives (Cross-Platform)

| Component | Portability | Effort |
|-----------|------------|--------|
| Ghostty terminal engine | Ghostty already runs on Linux (GTK + GL/Vulkan) | Low -- reuse as-is |
| cmuxd-remote Go daemon | Standard Go, uses Unix sockets + WebSockets + PTY | None -- compiles on Linux |
| Web platform (Next.js) | Deployed on Vercel, platform-agnostic | None |
| CLI protocol (V1/V2 JSON-RPC) | Protocol design is platform-agnostic | None |
| Unix domain socket transport | Standard POSIX | None |
| tmux compatibility layer | Protocol translation, no platform deps | Low |
| Agent hook system | Shell script injection, env vars | Low |
| Session snapshot JSON format | Pure data model | Low |
| cmux.json config schema | JSON schema | None |
| Notification OSC sequences | Terminal standard | None |

### 5.2 What Must Be Replaced (macOS-Specific)

| Layer | macOS | Linux Replacement | Effort |
|-------|-------|-------------------|--------|
| **Window system** | AppKit (NSWindow, NSApplication) | GTK4 or Qt6 | **High** |
| **View/widget tree** | NSView hierarchy, NSHostingView | GTK widget tree | **High** |
| **Terminal rendering** | Ghostty NSView (Metal) via xcframework | Ghostty GTK widget (GL/Vulkan) | **Medium** |
| **In-app browser** | WKWebView (WebKit) | WebKitGTK (gtk-webkit2) | **Medium** |
| **Keyboard events** | NSEvent, Carbon HIToolbox | XKB + xdg-keys or wl_keyboard | **Medium** |
| **Pasteboard** | NSPasteboard | GTK Clipboard / wl_data_device | **Low** |
| **Desktop notifications** | UNUserNotificationCenter | libnotify / D-Bus | **Low** |
| **Settings storage** | NSUserDefaults | GSettings / JSON files | **Low** |
| **Menu bar** | NSMenu, SwiftUI Commands | GTK MenuBar / PopoverMenu | **Medium** |
| **Global hotkey** | Carbon RegisterEventHotKey | X11 grab / wlr-foreign-toplevel | **Medium** |
| **Drag-and-drop** | NSDraggingDestination | GTK DragSource/DropTarget | **Medium** |
| **Font rendering** | CoreText | FreeType/Fontconfig (via Ghostty) | **None** (Ghostty handles) |
| **App lifecycle** | NSApplicationDelegate | GTK Application | **Medium** |
| **Process monitoring** | proc_pidinfo / kqueue | /proc filesystem | **Low** |
| **Code signing** | codesign + entitlements | N/A | **None** |
| **Auto-updater** | Sparkle | Flatpak/AppImage + custom | **Medium** |
| **CSS/inspector** | WKWebView Inspector | WebKitGTK Inspector | **Low** |

### 5.3 Architecture Decision: UI Framework

Three viable paths for the Linux UI layer:

| Option | Language | Pros | Cons | Recommendation |
|--------|----------|------|------|----------------|
| **A: GTK4 + C/Vala** | C/Vala | Ghostty already uses GTK; native look; battle-tested; use Ghostty's existing GTK terminal widget | Language mismatch with Swift core; requires C interop | **RECOMMENDED** for MVP |
| **B: Swift + Swift-GTK bindings** | Swift | Code reuse from macOS; same language | Swift-GTK is immature; small ecosystem; high bindings maintenance | Experimental option |
| **C: Qt/QML** | C++/QML | Rich widget set; good Wayland support; QtWebEngine for browser | Heavier; C++ complexity; QtWebEngine is huge | Viable alternative |

**Recommendation: Option A (GTK4 + C) with Rust or C++ for business logic.**

The strongest argument: Ghostty's Linux build already provides a working GTK terminal widget with GPU acceleration. cmux-alt can build on that foundation rather than reinventing terminal rendering.

### 5.4 Architecture Decision: Language for New Code

Given the macOS Swift codebase (~80K lines) is deeply coupled to AppKit/Metal, and the cross-platform components (cmuxd-remote, socket protocol, web platform) are in Go and TypeScript respectively:

| Component | Language | Rationale |
|-----------|----------|-----------|
| **UI layer** | C + GTK4 | Ghostty's existing GTK code; mature ecosystem |
| **Application logic** | Rust or C++ | Memory safety (Rust); good GTK bindings (gtk-rs); strong Linux ecosystem |
| **CLI** | Rust, Go, or keep Swift via swift-argument-parser | Go if kept minimal; Rust for richer CLI; Swift possible but adds LLVM dependency |
| **Socket server** | Rust or Go | Go is already proven in cmuxd-remote; Rust for more control |
| **cmuxd-remote** | Go (unchanged) | Already cross-platform |
| **Web platform** | TypeScript (unchanged) | Already cross-platform |
| **Ghostty fork** | Zig (unchanged) | Maintain as submodule |

**Recommendation: Rust** for the application core (socket server, business logic, workspace model) + **C/GTK4** for the UI layer (building on Ghostty's GTK terminal widget). Rust via gtk-rs provides safe bindings; C is used where direct Ghostty API interop is needed.

---

## 6. Recommended Technical Stack

### 6.1 Component Map

```
┌──────────────────────────────────────────────────────┐
│                   GTK4 UI Layer                       │
│  ┌───────────┐  ┌──────────────┐  ┌───────────────┐  │
│  │ Sidebar   │  │ Split Panes  │  │ Tab Bar       │  │
│  │ (GtkList) │  │ (GtkPaned)   │  │ (GtkNotebook) │  │
│  └───────────┘  └──────────────┘  └───────────────┘  │
│  ┌──────────────────────────────────────────────────┐ │
│  │ Ghostty GTK Terminal Widget (GPU-accelerated)    │ │
│  └──────────────────────────────────────────────────┘ │
│  ┌──────────────────────────────────────────────────┐ │
│  │ WebKitGTK Browser Widget                         │ │
│  └──────────────────────────────────────────────────┘ │
├──────────────────────────────────────────────────────┤
│              Rust Application Core                    │
│  ┌────────────┐ ┌───────────┐ ┌───────────────────┐  │
│  │ Workspace  │ │ Socket    │ │ Notification      │  │
│  │ Manager    │ │ Server    │ │ Manager           │  │
│  └────────────┘ └───────────┘ └───────────────────┘  │
│  ┌────────────┐ ┌───────────┐ ┌───────────────────┐  │
│  │ Session    │ │ Config    │ │ Agent Hook        │  │
│  │ Persistence│ │ Manager   │ │ Manager           │  │
│  └────────────┘ └───────────┘ └───────────────────┘  │
├──────────────────────────────────────────────────────┤
│                  IPC / Protocols                      │
│  ┌──────────────────────────────────────────────────┐ │
│  │ Unix Socket (JSON-RPC V2) — same protocol as     │ │
│  │ macOS cmux, compatible with existing CLI + tests │ │
│  └──────────────────────────────────────────────────┘ │
├──────────────────────────────────────────────────────┤
│            Cross-Platform Components                  │
│  ┌─────────────┐ ┌──────────────┐ ┌───────────────┐  │
│  │ cmuxd-remote│ │ CLI binary   │ │ Web Platform  │  │
│  │ (Go, reused)│ │ (Rust/Go)    │ │ (Next.js)     │  │
│  └─────────────┘ └──────────────┘ └───────────────┘  │
└──────────────────────────────────────────────────────┘
```

### 6.2 Key Dependencies

| Dependency | Purpose | License |
|-----------|---------|---------|
| Ghostty (forked submodule) | Terminal rendering engine | MIT |
| GTK 4.x | UI toolkit | LGPL 2.1+ |
| gtk-rs (gtk4 crate) | Rust GTK4 bindings | MIT |
| WebKitGTK 6.0 | In-app browser | LGPL 2+ |
| libnotify | Desktop notifications | LGPL 2.1 |
| libxkbcommon | Keyboard handling | MIT |
| serde / serde_json | JSON serialization (config, session, protocol) | MIT/Apache 2.0 |
| tokio | Async runtime for socket server and I/O | MIT |
| sqlite (via rusqlite) | Browser history, session store | MIT |
| Sentry Rust SDK | Crash reporting | MIT |
| libadwaita (optional) | Modern GNOME styling | LGPL 2.1+ |

### 6.3 Build System

| Component | Build System |
|-----------|-------------|
| Ghostty (terminal engine) | Zig build system (unchanged from cmux fork) |
| Rust application core + CLI | Cargo |
| C/GTK4 glue layer | Meson + Ninja |
| cmuxd-remote Go daemon | Go modules (unchanged) |
| Web platform | Bun/Next.js (unchanged) |

**Overall orchestration:** Top-level Makefile or `justfile` that sequences: clone Ghostty submodule -> zig build Ghostty -> cargo build Rust core -> meson build GTK UI -> go build cmuxd-remote.

---

## 7. Phased Implementation Plan

### Phase 0: Foundation (Weeks 1-2)

**Goal:** Shell app with terminal rendering, basic socket server, CLI compatibility.

**Deliverables:**
- [ ] Set up build system (Cargo workspace, Meson, submodule integration)
- [ ] Integrate Ghostty GTK terminal widget into a GTK4 window
- [ ] Implement basic Unix socket server (JSON-RPC V2 `system.identify`, `system.ping`)
- [ ] Implement `workspace.create` and `surface.create` (terminal) via socket
- [ ] Port CLI to Rust or Go (minimal: `version`, `ping`, `new-workspace`, `new-surface`, `send`)
- [ ] Run existing Python test suite (`tests_v2/`) against cmux-alt socket

**Exit criteria:** Can create a workspace with a terminal, send text, and pass the basic V2 protocol tests.

### Phase 1: Core Multiplexer (Weeks 3-6)

**Goal:** Full workspace/split/pane hierarchy, sidebar, notifications.

**Deliverables:**
- [ ] Implement full Workspace model in Rust (parity with macOS Workspace.swift)
- [ ] Implement Bonsplit replacement (recursive split tree) in Rust
- [ ] Build sidebar UI (vertical workspace tabs, GTK ListBox)
- [ ] Window > Workspace > Pane > Panel hierarchy in GTK
- [ ] Horizontal/vertical split panes via GtkPaned
- [ ] Pane focus navigation (keyboard + mouse)
- [ ] Surface tab bar within panes (GtkNotebook)
- [ ] Terminal notification system (OSC 9/99/777 detection)
- [ ] Notification store with read/unread tracking
- [ ] Desktop notifications via libnotify
- [ ] All workspace/surface/pane CRUD socket commands
- [ ] Keyboard shortcut infrastructure (fully customizable)
- [ ] Ghostty config loading (~/.config/ghostty/config)

**Exit criteria:** Can replicate the core macOS cmux workflow: multiple workspaces, splits, terminals, notifications, all controllable via CLI.

### Phase 2: Browser & Agent Integration (Weeks 7-10)

**Goal:** In-app browser, agent hooks, tmux compat, SSH workspaces.

**Deliverables:**
- [ ] WebKitGTK browser panel as first-class panel type
- [ ] Browser automation API via socket (browser.navigate, .eval, .screenshot, .element, .click, .type, .scroll)
- [ ] Browser profiles (isolated storage per profile)
- [ ] Address bar with search engine support
- [ ] Agent hook system (Claude Code, OpenCode, Codex, Hermes)
- [ ] Tmux compatibility layer (__tmux-compat shim)
- [ ] Claude Teams / OpenCode integration (proxy tmux to cmux socket)
- [ ] cmuxd-remote daemon integration (already cross-platform)
- [ ] SSH workspace creation (cmux ssh user@host)
- [ ] Remote browser proxy (browser traffic through SSH tunnel)
- [ ] Remote workspace lifecycle (bootstrap, connect, reconnect, disconnect)
- [ ] Agent session resume (Claude Code --resume, etc.)
- [ ] Agent process tracking via /proc

**Exit criteria:** Can launch Claude Code in a split pane with teammate mode, open browser panels, SSH into remote machines, and have agents trigger notifications.

### Phase 3: Session & Polish (Weeks 11-13)

**Goal:** Session persistence, settings UI, quality-of-life features.

**Deliverables:**
- [ ] Session auto-save (every 8 seconds)
- [ ] Session restore on launch (window layout, workspaces, terminals, browser URLs)
- [ ] Terminal scrollback capture and replay
- [ ] Agent session save/restore (persist session IDs)
- [ ] cmux.json config support (full schema compatibility)
- [ ] Settings window (searchable, GTK-based)
- [ ] Appearance: light/dark/system mode
- [ ] Command palette (Ctrl+Shift+P, fuzzy search)
- [ ] Global hotkey (show/hide all windows)
- [ ] Workspace metadata display (git branch, ports, PR status)
- [ ] File explorer in sidebar (directory tree with git status)
- [ ] Task manager (per-workspace CPU/memory)
- [ ] Sentry crash reporting
- [ ] Flatpak packaging

**Exit criteria:** Feature-complete enough to be a daily driver for the primary use case (AI agent orchestration).

### Phase 4: Ecosystem & Advanced (Weeks 14+)

**Goal:** Cloud VMs, advanced features, community readiness.

**Deliverables:**
- [ ] Cloud VM integration (cmux vm new, list, shell, destroy)
- [ ] Workspace feed/workstream system (agent permission requests)
- [ ] Markdown panel
- [ ] File preview panel
- [ ] Browser history import from other browsers
- [ ] Drag-and-drop files into terminals/browser
- [ ] Auto-updater (Flatpak/AppImage)
- [ ] PostHog analytics
- [ ] Comprehensive documentation
- [ ] CI/CD pipeline (GitHub Actions, Flatpak builds)
- [ ] Distribution: Flatpak on Flathub, AUR package, .deb/.rpm

---

## 8. Risk Assessment

### 8.1 Technical Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|-----------|------------|
| **Ghostty GTK API differences** from macOS API | High | Medium | Ghostty's GTK backend is production-tested; differences are in the embedding API, not rendering |
| **WebKitGTK limitations** vs WKWebView for automation | Medium | Medium | WebKitGTK supports JS evaluation, screenshot, DOM access; some developer tool features may differ |
| **Split pane GTK performance** at scale (many panes) | Medium | Low | GtkPaned is lightweight; Ghostty surfaces are the bottleneck, not pane containers |
| **Keyboard shortcut conflicts** with Linux DE/WM | Medium | High | Make all shortcuts configurable; provide sensible Linux-native defaults |
| **Clipboard interop** between X11 and Wayland | Low | Medium | Use GTK clipboard abstraction; handle both protocols transparently |
| **Rust async runtime** (tokio) integration with GTK main loop | Medium | Medium | Use glib main loop integration or spawn GTK on main thread, tokio on separate threads with channel communication |
| **Wayland vs X11** fragmentation | Medium | Medium | GTK4 abstracts both; Ghostty supports both; test on both |

### 8.2 Project Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|-----------|------------|
| **Scope creep** -- trying to match macOS feature-for-feature | High | High | Strict P0/P1/P2/P3 prioritization; P0+P1 is viable MVP |
| **Upstream cmux divergence** -- macOS cmux continues rapid development | Medium | Medium | Focus on protocol compatibility (V2 socket API); UI differences are acceptable |
| **Solo developer burnout** | High | Medium | Ship P0+P1, then build community; open source encourages contributors |
| **Ghostty API instability** -- upstream changes break integration | Medium | Low | Pin Ghostty submodule; cmux already maintains a fork |

### 8.3 Mitigation Strategies

1. **Protocol-first development:** The V2 JSON-RPC socket protocol is the stable contract. Build and test against it from day one. This ensures CLI compatibility and enables the Python test suite as an acceptance test harness.

2. **Incremental delivery:** Each phase produces a working, testable artifact. Phase 0 alone (terminal + basic socket) is already useful as a tmux alternative.

3. **Reuse aggressively:** cmuxd-remote (Go), the Python test suite, the web platform, the config schema, and Ghostty's GTK terminal widget are all ready to use. Don't rebuild what works.

4. **Community alignment:** Publish early, document the architecture, and make it easy for contributors to add features within the established patterns.

---

## 9. Appendix: Source Code Map

### 9.1 Key macOS Source Files and Their Linux Equivalents

| macOS File | Lines | Purpose | Linux Equivalent |
|-----------|-------|---------|-----------------|
| `Sources/cmuxApp.swift` | 8,246 | App entry, SwiftUI scene, menus, commands | `src/main.rs` + GTK application setup |
| `Sources/AppDelegate.swift` | ~2,000 | Central controller, window management, keyboard routing | `src/app.rs` Application struct |
| `Sources/TabManager.swift` | ~800 | Workspace list management per window | `src/workspace/manager.rs` |
| `Sources/Workspace.swift` | ~14,000 | Heavy model: panels, bonsplit, remote, snapshots | `src/workspace/mod.rs` (split into modules) |
| `Sources/ContentView.swift` | ~3,000 | Main window layout, sidebar + content area | `src/ui/content_view.rs` + GTK builder XML |
| `Sources/Sidebar/SidebarState.swift` | ~500 | Sidebar visibility, selection, appearance | `src/ui/sidebar.rs` |
| `Sources/GhosttyTerminalView.swift` | 13,458 | Terminal rendering: Ghostty surface, Metal, keyboard, pasteboard | Ghostty GTK widget (from Ghostty upstream) |
| `Sources/TerminalController.swift` | ~3,000 | Unix socket server (V1 + V2 protocol) | `src/socket/server.rs` |
| `Sources/Panels/BrowserPanel.swift` | ~2,000 | WKWebView browser panel | `src/ui/browser_panel.rs` (WebKitGTK) |
| `Sources/TerminalNotificationStore.swift` | ~600 | Notification data model, read/unread | `src/notification/store.rs` |
| `Sources/KeyboardShortcutSettings.swift` | ~1,500 | 80+ configurable shortcuts | `src/settings/shortcuts.rs` |
| `Sources/SessionPersistence.swift` | ~1,500 | Auto-save, restore, scrollback capture | `src/session/persistence.rs` |
| `Sources/CmuxConfig.swift` | ~500 | cmux.json parsing and model | `src/config/mod.rs` |
| `CLI/cmux.swift` | ~4,000 | CLI command parsing and dispatch | `src/cli/main.rs` |
| `CLI/CMUXCLI+TmuxCompatSupport.swift` | ~2,500 | Tmux compatibility layer | `src/cli/tmux_compat.rs` |

### 9.2 Cross-Platform Components (Reused As-Is)

| Path | Lines | Language | Purpose |
|------|-------|----------|---------|
| `daemon/remote/cmd/cmuxd-remote/` | ~3,000 | Go | Remote daemon: WebSocket RPC + PTY |
| `web/` | ~20,000 | TypeScript | Next.js platform |
| `ghostty/` | vendor | Zig/C | Terminal engine (forked submodule) |
| `web/data/cmux.schema.json` | 872 | JSON | Config schema |
| `web/data/cmux-settings.schema.json` | 5 | JSON | Settings schema alias |
| `web/data/cmux-shortcuts.ts` | ~200 | TypeScript | Shortcut reference data |
| `tests/` + `tests_v2/` | ~5,000 | Python | End-to-end API tests |

### 9.3 Swift Packages (Reimplement in Rust)

| Package | Purpose | Rust Equivalent |
|---------|---------|----------------|
| `CMUXWorkstream` | Agent feed/workstream events | `src/workstream/` module |
| `CMUXAgentVault` | Agent binary registration/indexing | `src/agents/vault.rs` |
| `CMUXAgentLaunch` | Agent launch sanitization, session resolution | `src/agents/launch.rs` |
| `CMUXAuthCore` | Auth state, keychain identity store | `src/auth/` + Secret Service API |
| `CMUXDebugLog` | Debug event log with ring buffer | `tracing` crate |
| `CMUXPasteboardFidelity` | Clipboard text comparison | `src/clipboard.rs` |
| `Bonsplit` (external dep) | Split pane tree layout engine | `src/layout/split_tree.rs` |

### 9.4 Protocol Documentation

**V2 JSON-RPC Protocol** (implemented in `TerminalController.swift`):

- Transport: Unix domain socket, newline-delimited JSON frames
- Auth: Password-based challenge (password from flag > env > file > keychain)
- Request format: `{"id":"<uuid>","method":"<name>","params":{...}}`
- Response format: `{"id":"<uuid>","ok":true,"result":{...}}`
- Error format: `{"id":"<uuid>","ok":false,"error":{"code":"...","message":"..."}}`
- Stream mode: Server sends multiple JSON lines after initial request
- Timeout: Default 15 seconds, configurable

**Method namespaces:**
- `system.*` -- ping, identify, capabilities
- `workspace.*` -- list, current, create, select, close, rename, reorder, action, move_to_window
- `pane.*` -- list, surfaces, create, focus
- `surface.*` -- list, focus, close, create, split, split_off, move, reorder, rename, send_text, send_key, read_text, action, health, trigger_flash
- `browser.*` -- navigate, eval, eval_async, back, forward, reload, url.get, screenshot, element, click, type, scroll, dialog.*, frame.*, cookies.*, storage.*, tab.*, state.*, console.*, errors.*, highlight, addinitscript, addscript, addstyle, focus_webview, open_split
- `notification.*` -- create, create_for_caller
- `auth.*` -- status, begin_sign_in, sign_out
- `settings.*` -- open
- `events.*` -- stream
- `vm.*` -- list, create, destroy, ssh_info, exec, ssh_attach, pty_attach
- `markdown.*` -- open
- `debug.*` -- terminals

---

## Document Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-05-08 | Initial comprehensive PRD based on deep codebase analysis |

---

*This PRD was generated by Hermes Agent (Pip) through deep analysis of the cmux codebase using parallel sub-agents examining the Swift app architecture, CLI/socket API, web platform, Ghostty integration, build system, and remote daemon.*
