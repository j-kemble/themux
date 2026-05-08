# Phase 1: Core Multiplexer

> **Status:** Not started | **Last updated:** 2026-05-08 | **Weeks:** 3-6 | **Goal:** Full workspace/split/pane hierarchy, sidebar, notifications

## Prerequisites

- [ ] [[phase-0-foundation]] complete

## Deliverables

### 1.1 Workspace Model

- [ ] `WorkspaceManager` fully implemented in `themux-core`
- [ ] Workspace CRUD: create, rename, close, reorder, select
- [ ] Workspace metadata: title, description, color, pinned state
- [ ] Working directory tracking per workspace
- [ ] Git branch detection
- [ ] Listening port detection
- [ ] `workspace.list` — returns all workspaces
- [ ] `workspace.create` — creates workspace with cwd, name, command
- [ ] `workspace.current` — returns focused workspace
- [ ] `workspace.select` — switches to workspace by ID
- [ ] `workspace.close` — closes workspace
- [ ] `workspace.rename` — renames workspace
- [ ] `workspace.reorder` — moves workspace in sidebar order
- [ ] `workspace.action` — pin/unpin

### 1.2 Split Pane Layout

- [ ] `SplitTree` replaces macOS Bonsplit
- [ ] `SplitNode` enum: Pane (leaf) + Split (divider + two children)
- [ ] `pane.create` — creates new pane in a split
- [ ] `surface.split` — splits a pane horizontally or vertically
- [ ] `pane.focus` — focuses pane by ID or direction
- [ ] `pane.list` — lists panes in workspace
- [ ] Divider position adjustable (default 0.5)
- [ ] Pane resize via mouse drag
- [ ] Surface tab bar within panes (multiple surfaces per pane)

### 1.3 Sidebar UI

- [ ] Vertical workspace list in GTK4 sidebar
- [ ] Workspace tabs show: title, color, notification count, git branch
- [ ] Click workspace to select
- [ ] Right-click context menu (rename, close, pin, color)
- [ ] Drag to reorder workspaces
- [ ] Sidebar toggle (Super+B)
- [ ] Workspace pinning (pinned stays at top)
- [ ] Notification badge on workspace tabs

### 1.4 Terminal Panels

- [ ] Multiple terminal panels per workspace via Ghostty GTK widget
- [ ] `surface.create` with type=terminal
- [ ] `surface.close` closes panel
- [ ] `surface.send_text` sends text to terminal
- [ ] `surface.send_key` sends keystroke
- [ ] `surface.read_text` reads terminal content
- [ ] Terminal title updates from OSC sequences
- [ ] Custom terminal titles

### 1.5 Notification System

- [ ] OSC 9, 99, 777 sequence parser in `themux-notify` (has tests)
- [ ] Terminal output intercepted for OSC sequences
- [ ] `NotificationStore` in `themux-core` tracks read/unread
- [ ] Desktop notifications via libnotify
- [ ] `notification.create` via socket
- [ ] `notification.create_for_caller` auto-associates with surface
- [ ] Notification panel/page in right sidebar
- [ ] Jump to latest unread (Super+Shift+U)
- [ ] Mark all read, clear all
- [ ] Notification sounds (optional)

### 1.6 Keyboard Shortcuts

- [ ] `KeyboardShortcutSettings` in `themux-core`
- [ ] 50+ configurable actions
- [ ] Default shortcuts matching cmux (Super instead of Cmd)
- [ ] Shortcuts stored in `~/.config/themux/config.json`
- [ ] Conflict detection
- [ ] Numbered workspace/surface selection (Super+1-9)

### 1.7 CLI Completion

- [ ] All Phase 1 V2 methods exposed via CLI
- [ ] `themux workspace list|create|select|close|rename`
- [ ] `themux new-pane`
- [ ] `themux new-surface`
- [ ] `themux send|send-key`
- [ ] `themux notify`

### 1.8 Protocol Tests

- [ ] `test_workspace_crud.py` passes all workspace operations
- [ ] `test_v2_protocol.py` updated with new methods
- [ ] Notification tests added

## Exit Criteria

```
✓ Multiple workspaces with custom names and colors
✓ Split panes: horizontal and vertical
✓ Sidebar shows all workspaces with metadata
✓ Notifications fire from terminal OSC sequences
✓ CLI can create workspaces, split panes, send text
✓ All Phase 1 protocol tests pass
```

## Related

- [[phase-0-foundation]] — Previous phase
- [[phase-2-browser-agents]] — Next phase
- [[development-roadmap]] — Full overview
- [[architecture]] — Workspace/Split/Pane hierarchy
- [[protocol-v2]] — All V2 methods implemented
