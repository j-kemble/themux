# Design Decisions

> **Status:** Living document | **Last updated:** 2026-05-08

## 1. Rust over Swift

**Decision:** Use Rust for the application core, not Swift.

**Rationale:**
- Rust has first-class GTK4 bindings (gtk-rs) — Swift does not
- Rust's cargo is simpler than Swift Package Manager + Xcode on Linux
- No LLVM/Swift runtime dependency on Linux
- Rich async ecosystem (tokio) for the socket server
- Memory safety without garbage collection
- Same language can be used for core, CLI, and UI (unified toolchain)

**Trade-off:** Can't reuse any of the ~80K lines of cmux Swift code. But that code is so deeply coupled to AppKit/Metal that it wouldn't port anyway.

## 2. GTK4 over Qt

**Decision:** Use GTK4 for the UI, not Qt.

**Rationale:**
- Ghostty already uses GTK4 on Linux — we can reuse its terminal widget directly
- WebKitGTK integrates naturally (no QtWebEngine bloat)
- libadwaita provides modern GNOME styling out of the box
- Smaller dependency footprint than Qt
- Strong Rust bindings (gtk-rs)

**Trade-off:** Qt has better cross-platform story, but we're Linux-only.

## 3. Protocol-First Development

**Decision:** Build the V2 JSON-RPC socket server before the GUI.

**Rationale:**
- The protocol is the stable contract between all components
- Python tests can validate protocol compliance immediately
- CLI can be developed and tested headlessly
- Agent hooks work from day one (they speak V2)
- GUI becomes "just another client" of the protocol

**Implementation:** `themux-socket` is a standalone crate. It starts before the GTK app and the GUI connects to it as a privileged internal client.

## 4. Pure Core Crate (No UI Dependencies)

**Decision:** `themux-core` must not depend on GTK, tokio, or any platform-specific crate.

**Rationale:**
- Testable with plain `cargo test`
- Reusable by CLI without pulling in GTK
- Forces clean separation of concerns
- Makes it easy to build a headless mode later

## 5. Cargo Workspace over Single Crate

**Decision:** 6 crates instead of one monolith.

**Rationale:**
- Compile times: only rebuild what changed
- Dependency isolation: `themux-core` doesn't pull GTK
- Clear API boundaries between components
- Each crate has a single responsibility

## 6. Ghostty as Submodule (Not Vendored)

**Decision:** Git submodule for Ghostty, not vendored source.

**Rationale:**
- Easier to track upstream changes
- Clear attribution and licensing
- Can switch to upstream ghostty-org/ghostty instead of cmux's fork
- Avoids bloating the repo with Zig/C source

## 7. cmuxd-remote Reused As-Is

**Decision:** Reuse cmuxd-remote Go daemon without rewrite.

**Rationale:**
- Already cross-platform (standard Go, Unix sockets, WebSockets)
- Proven in production on macOS cmux
- No benefit to rewriting 3K lines of Go in Rust
- Go toolchain is already a dependency (Zig builds Ghostty)

## 8. GPL-3.0-or-later License

**Decision:** Same license as cmux (GPL-3.0-or-later).

**Rationale:**
- Legal compatibility with cmux (GPL-3.0-or-later)
- Ensures derivatives stay open source
- Commercial license option for enterprises

## Rejected Alternatives

| Idea | Why Rejected |
|------|-------------|
| Electron app | Performance, memory, native feel |
| Rewrite cmuxd-remote in Rust | No benefit, adds risk |
| Single monolithic crate | Slow compile, poor separation |
| Vendored Ghostty source | Maintenance burden, attribution |
| Qt instead of GTK4 | Ghostty is GTK, WebKitGTK > QtWebEngine |
| Swift on Linux | Immature GTK bindings, LLVM dependency |

## Related

- [[architecture]] — How these decisions manifest in the system
- [[build-principles]] — Rules that flow from these decisions
- [[project-goals]] — What we're building
