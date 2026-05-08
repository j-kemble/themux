# Dos and Don'ts

> **Status:** Active | **Last updated:** 2026-05-08
> Practical patterns and anti-patterns for themux development.

## Do

### Architecture

- **DO** keep `themux-core` pure — no tokio, no GTK, no platform deps
- **DO** add new V2 methods to `themux-socket/src/protocol/v2.rs` first
- **DO** use `thiserror` for library error types, `anyhow` for binary `main()`
- **DO** wrap all GTK code behind `themux-app/src/ui/` module boundaries
- **DO** keep Ghostty changes in `ghostty/src/` to only what libghostty-vt needs
- **DO** run `./scripts/build-libghostty.sh` before `cargo build` when ghostty source changes
- **DO** use `#![allow(non_camel_case_types)]` in ghostty-sys for C API compatibility
- **DO** keep Ghostty changes focused on libghostty-vt — avoid pulling in full upstream features
- **DO** write protocol tests in Python before implementing new socket methods
- **DO** serialize workspace state with serde — it's the session persistence format
- **DO** use UUIDs for all domain object IDs (workspace, pane, panel)
- **DO** log with `tracing::info!` / `tracing::debug!` — configure via `RUST_LOG`

### Process

- **DO** run `cargo fmt` before every commit
- **DO** run `cargo clippy --workspace -- -D warnings` before pushing
- **DO** use conventional commit messages
- **DO** update [[architecture]] when architecture changes
- **DO** update [[protocol-v2]] when adding methods
- **DO** check [[development-roadmap]] for phase alignment
- **DO** document design rationale in [[design-decisions]]

### Code

- **DO** derive `Serialize + Deserialize` for any type that goes over the socket
- **DO** use `#[serde(rename_all = "snake_case")]` for JSON compatibility
- **DO** make OSC parsing whitespace-tolerant (terminals are messy)
- **DO** use `tokio::spawn` for socket connection handlers
- **DO** use atomic file writes (write to `.tmp`, then rename) for session persistence
- **DO** use `glib::MainContext::default().spawn_local()` for GTK UI callbacks
- **DO** use `#[cfg(test)]` for unit tests in the same file

## Don't

### Architecture

- **DON'T** add GTK imports to `themux-core` — violates purity rule
- **DON'T** create circular crate dependencies — always `app` → `socket` → `core`
- **DON'T** bypass the V2 protocol for internal communication — use the socket
- **DON'T** modify `ghostty/` source except for libghostty-vt build fixes
- **DON'T** use platform-specific APIs outside `themux-app` and `themux-notify`

### Process

- **DON'T** commit without running `cargo fmt` and `cargo clippy`
- **DON'T** push to main without tests passing
- **DON'T** leave `TODO` comments without a tracking issue reference
- **DON'T** skip updating docs when changing behavior
- **DON'T** build before reading [[building-checklist]]

### Code

- **DON'T** use `unsafe` without a comment explaining why it's necessary and safe
- **DON'T** use `unwrap()` in library code — use proper error handling
- **DON'T** block the GTK main thread with synchronous I/O
- **DON'T** hardcode paths — use `dirs::data_dir()`, `dirs::config_dir()`
- **DON'T** use `println!` for logging — use `tracing` macros
- **DON'T** allocate large buffers in the socket hot path — 4MB max per frame

### Specific Pitfalls

- **DON'T** assume X11 — use GTK4/GDK abstractions that work on Wayland too
- **DON'T** assume `~/.config` exists — create it if needed
- **DON'T** leak socket files — clean up stale ones on bind
- **DON'T** spawn threads from GTK callbacks — use `glib::spawn_future_local`
- **DON'T** write large scrollback to session JSON — cap at 4000 lines / 400K chars

## When in Doubt

1. Check [[architecture]] for where code belongs
2. Check [[design-decisions]] for established patterns
3. Check [[build-principles]] for rules
4. Check the existing code for similar patterns
5. Ask in the [[themux-dev|MOC]]

## Related

- [[build-principles]] — Formal rules
- [[design-decisions]] — Why we chose these patterns
- [[architecture]] — Where code lives
- [[building-checklist]] — Verify before shipping
