//! Calendar tree structure for representing calendar hierarchies.

use serde::{Deserialize, Serialize};

/// A node in the calendar tree representing a calendar and its children.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CalendarTreeNode {
    /// The calendar ID.
    pub id: String,
    /// Child calendar nodes.
    pub children: Vec<CalendarTreeNode>,
}

impl CalendarTreeNode {
    /// Create a new calendar tree node.
    pub fn new(id: String) -> Self {
        Self {
            id,
            children: Vec::new(),
        }
    }

    /// Add a child node to this node.
    pub fn add_child(&mut self, child: CalendarTreeNode) {
        self.children.push(child);
    }

    /// Find a node by its ID in the tree.
    pub fn find_by_id(&self, id: &str) -> Option<&CalendarTreeNode> {
        if self.id == id {
            return Some(self);
        }

        for child in &self.children {
            if let Some(found) = child.find_by_id(id) {
                return Some(found);
            }
        }

        None
    }

    /// Get all calendar IDs in the tree as a flat list.
    pub fn get_all_ids(&self) -> Vec<String> {
        let mut ids = vec![self.id.clone()];
        for child in &self.children {
            ids.extend(child.get_all_ids());
        }
        ids
    }
}
