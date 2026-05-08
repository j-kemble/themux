# Development Roadmap

> **Status:** Phase 0 complete | **Last updated:** 2026-05-08

## Overview

4-phase plan from shell app to ecosystem-ready terminal multiplexer.

```
Phase 0 [Weeks  1-2]  ██░░░░░░░░░░░░  Foundation
Phase 1 [Weeks  3-6]  ██████░░░░░░░░  Core Multiplexer
Phase 2 [Weeks  7-10] ██████████░░░░  Browser + Agents
Phase 3 [Weeks 11-13] █████████████░  Session + Polish
Phase 4 [Weeks   14+] ██████████████  Ecosystem
```

## Phase Summary

| Phase | Goal | Weeks | Key Deliverable |
|-------|------|-------|-----------------|
| [[phase-0-foundation]] | Shell app + terminal + basic socket | 1-2 | Terminal renders, `system.ping` responds |
| [[phase-1-core-multiplexer]] | Workspaces, splits, sidebar, notifications | 3-6 | Full terminal multiplexer, CLI controllable |
| [[phase-2-browser-agents]] | Browser, agent hooks, SSH workspaces | 7-10 | Agent launch in splits, remote SSH |
| [[phase-3-session-polish]] | Session persistence, settings, polish | 11-13 | Survives quit/relaunch |
| [[phase-4-ecosystem]] | Cloud VMs, Flatpak, community | 14+ | Ship-ready |

## Milestones

- **M0** (End of Phase 0): Terminal renders in GTK window, `system.ping` works via socket
- **M1** (End of Phase 1): Multiple workspaces with splits, notifications firing, CLI working
- **M2** (End of Phase 2): Claude Code runs in a split pane via tmux shim, SSH works
- **M3** (End of Phase 3): Session survives restart, settings UI functional
- **M4** (End of Phase 4): Flatpak on Flathub, community contributing

## Dependencies Between Phases

```
Phase 0 ──► Phase 1 ──► Phase 2 ──► Phase 3 ──► Phase 4
                              │
                              └── cmuxd-remote (Go, cross-platform)
```

Phase 2 depends on libghostty-vt being built (Phase 0) — provides terminal emulation engine via `crates/ghostty-sys/`.
Phase 3 depends on workspace model from Phase 1.
Phase 4 depends on session persistence from Phase 3.

## Related

- [[project-goals]] — What we're building toward
- [[phase-0-foundation]] — Start here
- [[architecture]] — System overview
- [[building-checklist]] — Build verification per phase
