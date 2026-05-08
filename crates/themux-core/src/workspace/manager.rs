// Workspace manager: ordered list of workspaces per window.

use crate::workspace::{Workspace, WorkspaceId};
use serde::{Deserialize, Serialize};

/// Manages workspaces within a single window.
/// Maintains ordered list, selected index, and CRUD operations.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WorkspaceManager {
    pub workspaces: Vec<Workspace>,
    pub selected_index: usize,
}

impl WorkspaceManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn selected(&self) -> Option<&Workspace> {
        self.workspaces.get(self.selected_index)
    }

    pub fn selected_mut(&mut self) -> Option<&mut Workspace> {
        self.workspaces.get_mut(self.selected_index)
    }

    pub fn add(&mut self, workspace: Workspace) {
        self.workspaces.push(workspace);
        self.selected_index = self.workspaces.len().saturating_sub(1);
    }

    pub fn remove(&mut self, id: WorkspaceId) -> Option<Workspace> {
        let idx = self.workspaces.iter().position(|w| w.id == id)?;
        let removed = self.workspaces.remove(idx);
        if self.selected_index >= self.workspaces.len() {
            self.selected_index = self.workspaces.len().saturating_sub(1);
        }
        Some(removed)
    }

    pub fn select(&mut self, id: WorkspaceId) -> bool {
        if let Some(idx) = self.workspaces.iter().position(|w| w.id == id) {
            self.selected_index = idx;
            true
        } else {
            false
        }
    }

    pub fn select_index(&mut self, index: usize) -> bool {
        if index < self.workspaces.len() {
            self.selected_index = index;
            true
        } else {
            false
        }
    }

    pub fn reorder(&mut self, from_idx: usize, to_idx: usize) {
        if from_idx < self.workspaces.len() && to_idx < self.workspaces.len() {
            let ws = self.workspaces.remove(from_idx);
            self.workspaces.insert(to_idx, ws);
            // Adjust selected index
            if self.selected_index == from_idx {
                self.selected_index = to_idx;
            } else if from_idx < self.selected_index && to_idx >= self.selected_index {
                self.selected_index = self.selected_index.saturating_sub(1);
            } else if from_idx > self.selected_index && to_idx <= self.selected_index {
                self.selected_index += 1;
            }
        }
    }

    pub fn find_by_id(&self, id: WorkspaceId) -> Option<&Workspace> {
        self.workspaces.iter().find(|w| w.id == id)
    }

    pub fn find_by_id_mut(&mut self, id: WorkspaceId) -> Option<&mut Workspace> {
        self.workspaces.iter_mut().find(|w| w.id == id)
    }
}
