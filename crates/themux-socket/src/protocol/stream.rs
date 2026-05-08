// Event stream mode: server pushes JSON events to connected clients.

use serde::Serialize;

/// An event in the cmux event bus.
#[derive(Debug, Serialize)]
pub struct Event {
    pub r#type: String,       // event | ack | heartbeat
    pub seq: u64,
    pub id: String,
    pub name: String,
    pub category: Option<String>,
    pub source: Option<String>,
    pub occurred_at: String,
    pub workspace_id: Option<String>,
    pub surface_id: Option<String>,
    pub pane_id: Option<String>,
    pub window_id: Option<String>,
    pub payload: Option<serde_json::Value>,
}
