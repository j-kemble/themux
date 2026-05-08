// Pane: the leaf container in the split tree that holds panels (surfaces).

use crate::workspace::{PaneId, PanelId};
use serde::{Deserialize, Serialize};

/// A pane is a leaf node in the split tree. It holds one or more panels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pane {
    pub id: PaneId,
    pub panels: Vec<Panel>,
    pub focused_panel_index: usize,
}

impl Pane {
    pub fn new(id: PaneId) -> Self {
        Self {
            id,
            panels: Vec::new(),
            focused_panel_index: 0,
        }
    }

    pub fn focused_panel(&self) -> Option<&Panel> {
        self.panels.get(self.focused_panel_index)
    }

    pub fn focused_panel_mut(&mut self) -> Option<&mut Panel> {
        self.panels.get_mut(self.focused_panel_index)
    }

    pub fn add_panel(&mut self, panel: Panel) {
        self.panels.push(panel);
        self.focused_panel_index = self.panels.len().saturating_sub(1);
    }

    pub fn remove_panel(&mut self, id: PanelId) -> Option<Panel> {
        let idx = self.panels.iter().position(|p| p.id == id)?;
        let removed = self.panels.remove(idx);
        if self.focused_panel_index >= self.panels.len() {
            self.focused_panel_index = self.panels.len().saturating_sub(1);
        }
        Some(removed)
    }
}

/// A panel (surface) within a pane. Can be terminal, browser, markdown, or file preview.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    pub id: PanelId,
    pub panel_type: PanelType,
    pub title: String,
    pub custom_title: Option<String>,
    pub working_directory: Option<String>,
    pub pinned: bool,
    pub notification_count: u32,
    pub has_unread: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelType {
    Terminal {
        command: Option<String>,
        scrollback_lines: usize,
    },
    Browser {
        url: Option<String>,
    },
    Markdown {
        file_path: Option<String>,
    },
    FilePreview {
        file_path: Option<String>,
    },
}

impl PanelType {
    pub fn name(&self) -> &'static str {
        match self {
            PanelType::Terminal { .. } => "terminal",
            PanelType::Browser { .. } => "browser",
            PanelType::Markdown { .. } => "markdown",
            PanelType::FilePreview { .. } => "file_preview",
        }
    }
}
