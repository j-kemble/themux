// Notification store: OSC sequence parsing, read/unread tracking.

pub mod store;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A notification received from a terminal surface via OSC sequences
/// or from an agent via the socket API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub workspace_id: crate::workspace::WorkspaceId,
    pub panel_id: Option<crate::workspace::PanelId>,
    pub title: String,
    pub subtitle: Option<String>,
    pub body: Option<String>,
    pub category: Option<String>,
    pub read: bool,
    pub created_at: DateTime<Utc>,
}

/// Notification store: per-workspace read/unread tracking with ordering.
#[derive(Debug, Default)]
pub struct NotificationStore {
    pub notifications: Vec<Notification>,
}

impl NotificationStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }

    pub fn unread_count(&self) -> usize {
        self.notifications.iter().filter(|n| !n.read).count()
    }

    pub fn unread_for_workspace(&self, workspace_id: crate::workspace::WorkspaceId) -> usize {
        self.notifications
            .iter()
            .filter(|n| n.workspace_id == workspace_id && !n.read)
            .count()
    }

    pub fn mark_read(&mut self, id: Uuid) -> bool {
        if let Some(n) = self.notifications.iter_mut().find(|n| n.id == id) {
            n.read = true;
            true
        } else {
            false
        }
    }

    pub fn mark_all_read(&mut self) {
        for n in &mut self.notifications {
            n.read = true;
        }
    }

    pub fn clear_all(&mut self) {
        self.notifications.clear();
    }

    pub fn latest_unread(&self) -> Option<&Notification> {
        self.notifications
            .iter()
            .rev()
            .find(|n| !n.read)
    }

    pub fn for_workspace(
        &self,
        workspace_id: crate::workspace::WorkspaceId,
    ) -> Vec<&Notification> {
        self.notifications
            .iter()
            .filter(|n| n.workspace_id == workspace_id)
            .collect()
    }
}
