// Split tree implementation — recursive split/pane operations.

use super::{Split, SplitDirection, SplitNode};
use crate::workspace::pane::Pane;
use crate::workspace::PaneId;

/// Operations on the split tree for creating splits and moving panes.
pub struct SplitTree;

impl SplitTree {
    /// Split an existing pane: replace the pane node with a new Split
    /// containing the original pane and a new pane.
    pub fn split_pane(
        root: &mut SplitNode,
        pane_id: PaneId,
        direction: SplitDirection,
        new_pane: Pane,
    ) -> Result<(), SplitError> {
        match root {
            SplitNode::Pane(pane) if pane.id == pane_id => {
                let existing = std::mem::replace(pane, Pane::new(PaneId::nil()));
                let existing_box = Box::new(SplitNode::Pane(existing));
                let new_box = Box::new(SplitNode::Pane(new_pane));

                let (first, second) = match direction {
                    SplitDirection::Horizontal => (existing_box, new_box),
                    SplitDirection::Vertical => (existing_box, new_box),
                };

                *root = SplitNode::Split(Split {
                    direction,
                    divider_position: 0.5,
                    first,
                    second,
                });
                Ok(())
            }
            SplitNode::Pane(_) => Err(SplitError::PaneNotFound),
            SplitNode::Split(split) => {
                SplitTree::split_pane(&mut split.first, pane_id, direction, new_pane.clone())
                    .or_else(|_| {
                        SplitTree::split_pane(&mut split.second, pane_id, direction, new_pane)
                    })
            }
        }
    }

    /// Close a pane and collapse the tree: remove the pane, and if it was
    /// half of a split, promote the sibling to replace the split.
    pub fn close_pane(
        _root: &mut SplitNode,
        _pane_id: PaneId,
    ) -> Result<SplitNode, SplitError> {
        // TODO: implement tree collapse on pane close
        Err(SplitError::PaneNotFound)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SplitError {
    #[error("pane not found")]
    PaneNotFound,
    #[error("cannot split further")]
    CannotSplit,
}
