# themux V2 JSON-RPC Protocol

## Transport

- **Mode:** Unix domain socket (`SOCK_STREAM`)
- **Default path:** `~/.local/share/themux/themux.sock`
- **Framing:** Newline-delimited JSON frames (one JSON object per line)
- **Encoding:** UTF-8

## Authentication

Password-based challenge. Resolution order:
1. `--password` CLI flag
2. `THEMUX_SOCKET_PASSWORD` environment variable
3. `~/.local/share/themux/socket-password` file
4. Secret Service (freedesktop.org) — TBD

Flow:
```
Client: auth <password>\n
Server: OK\n          (success)
Server: ERROR: ...\n  (failure)
```

Modes: `off`, `automation`, `password`, `allowAll`

## Message Format

### Request
```json
{"id": "<uuid>", "method": "<namespace.action>", "params": {...}}
```

### Success Response
```json
{"id": "<uuid>", "ok": true, "result": {...}}
```

### Error Response
```json
{"id": "<uuid>", "ok": false, "error": {"code": "...", "message": "..."}}
```

## Method Namespaces

### system.*
| Method | Params | Returns | Description |
|--------|--------|---------|-------------|
| `system.ping` | `{}` | `{"pong": true}` | Health check |
| `system.identify` | `{}` | `{"server": "themux", "version": "..."}` | Server identity |
| `system.capabilities` | `{}` | `{"protocols": [...], "features": [...]}` | Feature flags |

### workspace.*
| Method | Params | Returns | Description |
|--------|--------|---------|-------------|
| `workspace.list` | `{}` | `{"workspaces": [...]}` | List all workspaces |
| `workspace.current` | `{}` | `{"workspace_id": "..."}` | Get current workspace |
| `workspace.create` | `{cwd?, name?, description?, command?, layout?}` | `{"workspace_id": "..."}` | Create workspace |
| `workspace.select` | `{workspace_id}` | `{"selected": true}` | Select workspace |
| `workspace.close` | `{workspace_id}` | `{"closed": true}` | Close workspace |
| `workspace.rename` | `{workspace_id, title}` | `{"renamed": true}` | Rename workspace |
| `workspace.reorder` | `{workspace_id, index}` | `{"reordered": true}` | Reorder workspace |
| `workspace.action` | `{workspace_id, action}` | `{}` | Pin/unpin/etc. |
| `workspace.move_to_window` | `{workspace_id, window_id}` | `{}` | Move to window |

### pane.*
| Method | Params | Returns | Description |
|--------|--------|---------|-------------|
| `pane.list` | `{workspace_id?}` | `{"panes": [...]}` | List panes |
| `pane.create` | `{workspace_id?, type?, direction?, url?, focus?}` | `{"pane_id": "..."}` | Create pane |
| `pane.focus` | `{pane_id}` | `{}` | Focus pane |
| `pane.surfaces` | `{pane_id}` | `{"surfaces": [...]}` | List surfaces in pane |

### surface.*
| Method | Params | Returns | Description |
|--------|--------|---------|-------------|
| `surface.list` | `{workspace_id?}` | `{"surfaces": [...]}` | List all surfaces |
| `surface.focus` | `{surface_id}` | `{}` | Focus surface |
| `surface.close` | `{surface_id}` | `{}` | Close surface |
| `surface.create` | `{workspace_id?, type?, pane_id?, url?, focus?}` | `{"surface_id": "..."}` | Create surface |
| `surface.split` | `{workspace_id?, surface_id?, direction?}` | `{"pane_id": "..."}` | Split pane |
| `surface.send_text` | `{surface_id?, text}` | `{"sent": true}` | Send text to terminal |
| `surface.send_key` | `{surface_id?, key}` | `{"sent": true}` | Send key to terminal |
| `surface.read_text` | `{surface_id?, lines?, scrollback?}` | `{"text": "..."}` | Read terminal content |
| `surface.rename` | `{surface_id, title}` | `{}` | Rename surface tab |
| `surface.move` | `{surface_id, workspace_id?, pane_id?, window_id?, before?, after?, index?}` | `{}` | Move surface |
| `surface.health` | `{surface_id?}` | `{"healthy": true}` | Check surface health |

### browser.*
See cmux's agent-browser port spec for full browser automation API.

### notification.*
| Method | Params | Returns | Description |
|--------|--------|---------|-------------|
| `notification.create` | `{title, subtitle?, body?, category?}` | `{"notification_id": "..."}` | Create notification |
| `notification.create_for_caller` | `{title, subtitle?, body?, category?}` | `{"notification_id": "..."}` | Create from calling surface |

### events.*
| Method | Params | Returns | Description |
|--------|--------|---------|-------------|
| `events.stream` | `{after_seq?, name?, category?, reconnect?, limit?}` | Stream | Subscribe to events |

### vm.*
Cloud VM management (requires web platform).

---

## Event Types

Events follow cmux event bus format:
- `type`: `event` | `ack` | `heartbeat`
- `seq`: Monotonic sequence number
- `id`: Unique event ID
- `name`: Event name (e.g., `workspace.created`, `notification.received`)
- `category`: Event category
- `source`: Event source (e.g., `osc`, `cli`, `agent`)
- `workspace_id`, `surface_id`, `pane_id`, `window_id`: Context IDs
- `payload`: Arbitrary JSON payload
