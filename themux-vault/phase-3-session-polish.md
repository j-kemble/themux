# Phase 3: Session + Polish

> **Status:** Not started | **Last updated:** 2026-05-08 | **Weeks:** 11-13 | **Goal:** Session persistence, settings UI, quality-of-life

## Prerequisites

- [ ] [[phase-2-browser-agents]] complete

## Deliverables

### 3.1 Session Persistence

- [ ] `SessionPersistence` auto-save timer (8 seconds)
- [ ] `AppSessionSnapshot` JSON serialization
- [ ] Snapshot captures: window frames, workspace layout, pane splits
- [ ] Snapshot captures: all panel types (terminal, browser, markdown, filepreview)
- [ ] Terminal scrollback capture (up to 4000 lines / 400K chars)
- [ ] Browser URL + navigation history in snapshot
- [ ] Agent session IDs in snapshot (for resume)
- [ ] Auto-save to `~/.local/share/themux/session.json`
- [ ] Atomic writes (temp file → rename)
- [ ] Previous session snapshot (`session-previous.json`) on quit
- [ ] Restore on launch: recreate windows, workspaces, splits, panels
- [ ] Restore: replay terminal scrollback via `CMUX_RESTORE_SCROLLBACK_FILE`
- [ ] Restore: browser navigation to saved URL
- [ ] Restore: agent session resume (send resume command)
- [ ] Manual restore: "Reopen Previous Session" command

### 3.2 Settings & Configuration

- [ ] `cmux.json` compatible config (`~/.config/themux/config.json`)
- [ ] Config loader with merge (global + project-level `.themux.json`)
- [ ] Settings window (GTK4, searchable)
- [ ] Settings categories: Appearance, Terminal, Shortcuts, Notifications, Browser, Advanced
- [ ] Keyboard shortcut recording UI
- [ ] Font family and size picker
- [ ] Theme picker (light/dark/system)
- [ ] Sidebar appearance: material, tint, opacity, corner radius
- [ ] Workspace color customization
- [ ] Custom actions and commands from config
- [ ] Vault agent registration from config
- [ ] `cmux.json` schema validation

### 3.3 Appearance

- [ ] System / Light / Dark mode
- [ ] Ghostty config sync on appearance change
- [ ] Sidebar styling: glass, native, custom presets
- [ ] Terminal background opacity
- [ ] CSS theming via `ui/themux.css`
- [ ] libadwaita integration (optional feature flag)

### 3.4 Command Palette

- [ ] Command palette overlay (Super+Shift+P)
- [ ] Fuzzy search across all registered commands
- [ ] Custom actions from config
- [ ] Quick workspace switching
- [ ] Quick command execution

### 3.5 Quality of Life

- [ ] Welcome/onboarding screen (first launch)
- [ ] Menu bar: File, Edit, View, Workspace, Help
- [ ] Right-click context menus on workspaces and surfaces
- [ ] Surface tab close button always visible
- [ ] Git branch display in sidebar
- [ ] Listening port display in sidebar
- [ ] PR status display in sidebar
- [ ] Loading indicator in terminal while loading
- [ ] Cmd+click opens links in themux browser (not external)

### 3.6 Global Hotkey

- [ ] System-wide hotkey to show/hide all windows
- [ ] Configurable keybinding
- [ ] Works on both X11 and Wayland

### 3.7 Sentry Integration

- [ ] Panic handler sends crash reports
- [ ] Error context: themux version, OS, Rust version
- [ ] Opt-in / opt-out setting

### 3.8 Protocol Tests

- [ ] Session save/restore test
- [ ] Config load/reload test
- [ ] Settings open test

## Exit Criteria

```
✓ Quit themux, relaunch — all workspaces and terminals restored
✓ Terminal scrollback survives restart
✓ Browser URL restored on relaunch
✓ Settings window opens, changes persist
✓ Command palette opens and searches commands
✓ System hotkey works
```

## Related

- [[phase-2-browser-agents]] — Previous phase
- [[phase-4-ecosystem]] — Next phase
- [[development-roadmap]] — Full overview
- [[architecture]] — Session persistence flow
