// Workspace model: the core domain types.
//
// Hierarchy: Window > TabManager > Workspace > Pane > Panel (Surface)
//
// A Workspace owns a split tree (Bonsplit replacement) that divides the
// available space into recursive Split nodes and leaf Pane nodes.
// Each Pane holds one or more Panels (terminals, browsers, etc.).

pub mod manager;
pub mod pane;
pub mod surface;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a workspace.
pub type WorkspaceId = Uuid;

/// Unique identifier for a pane within a workspace.
pub type PaneId = Uuid;

/// Unique identifier for a panel (surface) within a pane.
pub type PanelId = Uuid;

/// Unique identifier for a window.
pub type WindowId = Uuid;

/// The top-level workspace model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub title: String,
    pub description: Option<String>,
    pub color: Option<WorkspaceColor>,
    pub pinned: bool,
    pub working_directory: Option<String>,
    pub git_branch: Option<String>,
    pub listening_ports: Vec<u16>,
    pub created_at: DateTime<Utc>,
    pub layout: crate::layout::SplitNode,
    pub remote: Option<RemoteConfig>,
    pub metadata: WorkspaceMetadata,
}

/// Configurable color for workspace identification in the sidebar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceColor {
    pub name: String,
    pub hex: String,
}

/// Remote workspace transport configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
    pub transport: RemoteTransport,
    pub state: RemoteConnectionState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteTransport {
    Ssh {
        destination: String,
        port: u16,
        identity_file: Option<String>,
    },
    WebSocket {
        url: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteConnectionState {
    Disconnected,
    Connecting,
    Connected { heartbeat_count: u64 },
    Error { message: String },
}

/// Additional metadata displayed in the sidebar.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkspaceMetadata {
    pub status: Option<String>,
    pub progress: Option<ProgressInfo>,
    pub log_entries: Vec<LogEntry>,
    pub pr_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressInfo {
    pub percent: u8,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub message: String,
}
