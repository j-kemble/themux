// Session snapshot data structures.
//
// Mirrors cmux's AppSessionSnapshot JSON structure for compatibility.

use crate::workspace::Workspace;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Top-level session snapshot saved on quit and restored on launch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSessionSnapshot {
    pub version: u32,
    pub timestamp: DateTime<Utc>,
    pub windows: Vec<WindowSnapshot>,
}

/// Per-window snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSnapshot {
    pub frame: WindowFrame,
    pub workspaces: Vec<WorkspaceSnapshot>,
    pub selected_workspace_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowFrame {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Per-workspace snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSnapshot {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub pinned: bool,
    pub working_directory: Option<String>,
    pub panel_snapshots: Vec<PanelSnapshot>,
    pub layout: serde_json::Value, // Arbitrary layout tree
}

/// Per-panel (surface) snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelSnapshot {
    pub id: String,
    pub panel_type: String,
    pub title: String,
    pub custom_title: Option<String>,
    pub working_directory: Option<String>,
    pub pinned: bool,
    /// Terminal-specific: scrollback text.
    pub scrollback: Option<String>,
    /// Agent session ID for resume.
    pub agent_session_id: Option<String>,
    pub agent_kind: Option<String>,
    /// Browser-specific: URL and history.
    pub url: Option<String>,
    pub browser_history: Option<Vec<String>>,
    /// Git branch at snapshot time.
    pub git_branch: Option<String>,
}
