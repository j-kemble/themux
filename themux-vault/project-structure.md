# Project Structure

> **Status:** Reference | **Last updated:** 2026-05-08

## Top-Level Layout

```
themux/
├── cmuxaltprd.md              # Original Linux port PRD
├── Cargo.toml                 # Workspace root (6 crates)
├── Makefile                   # Build orchestration
├── README.md                  # Project overview
├── LICENSE                    # GPL-3.0-or-later
├── .gitignore
├── .gitmodules                # Ghostty submodule
│
├── crates/
│   ├── themux-core/           # Pure data models + business logic
│   │   ├── src/
│   │   │   ├── lib.rs         # Crate root, module declarations
│   │   │   ├── workspace/     # Workspace, Pane, Panel types + Manager
│   │   │   ├── config/        # cmux.json loader, schema, shortcuts
│   │   │   ├── layout/        # Split tree (Bonsplit replacement)
│   │   │   ├── session/       # Snapshot + persistence
│   │   │   ├── notification/  # Notification store
│   │   │   └── remote/        # SSH remote workspace config
│   │   └── Cargo.toml
│   │
│   ├── themux-socket/         # Unix socket server + V2 protocol
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── server.rs      # Unix listener, auth, connection handling
│   │   │   ├── protocol/      # V2 dispatch, auth, streaming
│   │   │   └── event_bus.rs   # Ring buffer event system
│   │   └── Cargo.toml
│   │
│   ├── themux-cli/            # CLI binary (the `themux` command)
│   │   ├── src/
│   │   │   ├── main.rs        # Clap CLI definition (~80 commands)
│   │   │   ├── commands/      # Per-domain command stubs
│   │   │   ├── shell.rs       # POSIX shell quoting
│   │   │   └── tmux_compat.rs # Tmux command translation layer
│   │   └── Cargo.toml
│   │
│   ├── themux-agent/          # AI agent hook system
│   │   ├── src/
│   │   │   ├── lib.rs         # AgentKind enum
│   │   │   ├── hooks.rs       # Tmux shim installer
│   │   │   ├── agents/        # Per-agent env vars (Claude, Codex, etc.)
│   │   │   └── vault.rs       # Custom agent registration
│   │   └── Cargo.toml
│   │
│   ├── themux-notify/         # Desktop notification + OSC parser
│   │   ├── src/
│   │   │   ├── lib.rs         # Desktop notification sender
│   │   │   ├── osc.rs         # OSC 9/99/777 sequence parser
│   │   │   └── notification.rs
│   │   └── Cargo.toml
│   │
│   └── themux-app/            # GTK4 application binary
│       ├── src/
│       │   ├── main.rs        # Entry point, GTK app init
│       │   ├── app.rs         # Window builder, sidebar, content
│       │   └── ui/            # Sidebar, content, terminal, browser, panels
│       ├── build.rs
│       └── Cargo.toml
│
├── ui/                        # GTK4 resource files
│   ├── themux.css             # Application stylesheet
│   └── themux.gresource.xml   # GResource manifest
│
├── ghostty/                   # Ghostty submodule directory (currently empty until initialized)
├── daemon/                    # cmuxd-remote Go daemon slot (README present; daemon source not copied yet)
│
├── data/
│   └── default-config.json    # Bundled default config
│
├── docs/
│   ├── architecture.md        # System architecture document
│   ├── protocol-v2.md         # V2 JSON-RPC spec
│   └── contributing.md        # Contributor guide
│
├── scripts/
│   ├── setup.sh               # Full project setup
│   ├── build.sh               # Build with mode selection
│   ├── run.sh                 # Build + launch
│   └── run-tests.sh           # Rust + Python tests
│
├── tests/                     # Python integration tests
│   ├── conftest.py            # Socket connection fixture
│   ├── requirements.txt       # pytest, websockets
│   ├── test_v2_protocol.py    # System ping, identify, capabilities
│   ├── test_workspace_crud.py # Workspace create/list
│   ├── test_browser_api.py    # Browser automation protocol
│   └── test_tmux_compat.py    # Tmux compat protocol tests
│
├── themux-vault/              # Obsidian knowledge base (this vault)
└── .github/workflows/ci.yml   # CI pipeline
```

## Crate Descriptions

| Crate | Responsibility | Deps |
|-------|---------------|------|
| `themux-core` | Domain models, config, session, layout | serde, uuid, chrono, rusqlite |
| `themux-socket` | Socket server, V2 dispatch, event bus | core, tokio, hmac, sha2 |
| `themux-cli` | CLI binary, ~80 commands, tmux compat | core, socket, agent, clap |
| `themux-agent` | Agent hooks, shim installer, vault | core |
| `themux-notify` | Desktop notifications, OSC parser | core, notify-rust |
| `themux-app` | GTK4 GUI binary | core, socket, notify, gtk4, webkit2gtk |

## Current Setup Status

As of 2026-05-08, the repository scaffold exists but is not yet fully initialized:

- No `.git/` directory is present in `/home/josh/Projects/themux`, so `git status` and `git submodule status` do not work yet.
- `.gitmodules` declares the `ghostty` submodule, but `ghostty/` is currently empty and has no `build.zig`.
- `daemon/cmuxd-remote/` is not present yet; only `daemon/README.md` documents how to copy it from cmux.
- The CLI parses commands but `crates/themux-cli/src/main.rs` still has a TODO where socket dispatch should be wired.
- `themux-socket` has partial/stub V2 dispatch only; see [[protocol-v2#Current Implementation Status]].
- Several GTK/WebKit/Ghostty UI modules are placeholders until Phase 0 terminal integration starts.

## Dependency Rules

1. **`themux-core` MUST NOT import any GTK, tokio, or platform crate.** Pure data only.
2. **Crates depend downward only**: `app` → `socket` → `core`, never `core` → `socket`.
3. **`themux-socket`** can use tokio (async socket I/O).
4. **`themux-app`** can use GTK4, WebKitGTK, tokio.
5. **`themux-notify`** can use notify-rust (platform notification bridge).

## Related

- [[architecture]] — How these pieces connect at runtime
- [[design-decisions]] — Why this structure
- [[build-principles]] — Rules for working within this structure
- [[building-checklist]] — Step-by-step build guide
