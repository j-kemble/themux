// Split tree layout engine — Bonsplit replacement.
//
// Recursive binary tree where interior nodes are Splits (horizontal/vertical
// dividers) and leaves are Panes. This is the core layout primitive used
// by Workspace to manage spatial arrangement.

pub mod split_tree;

use crate::workspace::pane::Pane;
use serde::{Deserialize, Serialize};

/// A node in the recursive split tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SplitNode {
    Pane(Pane),
    Split(Split),
}

/// An interior node that divides space between two children.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Split {
    pub direction: SplitDirection,
    /// Normalized divider position (0.0 = all first, 1.0 = all second).
    pub divider_position: f64,
    pub first: Box<SplitNode>,
    pub second: Box<SplitNode>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

impl SplitNode {
    /// Collect all pane IDs in this subtree.
    pub fn pane_ids(&self) -> Vec<crate::workspace::PaneId> {
        let mut ids = Vec::new();
        self.collect_pane_ids(&mut ids);
        ids
    }

    fn collect_pane_ids(&self, ids: &mut Vec<crate::workspace::PaneId>) {
        match self {
            SplitNode::Pane(pane) => ids.push(pane.id),
            SplitNode::Split(split) => {
                split.first.collect_pane_ids(ids);
                split.second.collect_pane_ids(ids);
            }
        }
    }

    /// Find a pane by ID in this subtree.
    pub fn find_pane(&self, id: crate::workspace::PaneId) -> Option<&Pane> {
        match self {
            SplitNode::Pane(pane) if pane.id == id => Some(pane),
            SplitNode::Pane(_) => None,
            SplitNode::Split(split) => split
                .first
                .find_pane(id)
                .or_else(|| split.second.find_pane(id)),
        }
    }

    /// Find a mutable pane by ID in this subtree.
    pub fn find_pane_mut(&mut self, id: crate::workspace::PaneId) -> Option<&mut Pane> {
        match self {
            SplitNode::Pane(pane) if pane.id == id => Some(pane),
            SplitNode::Pane(_) => None,
            SplitNode::Split(split) => split
                .first
                .find_pane_mut(id)
                .or_else(|| split.second.find_pane_mut(id)),
        }
    }
}
