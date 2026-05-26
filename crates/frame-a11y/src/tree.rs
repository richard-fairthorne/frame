use crate::node::{AccessibilityNode, AccessibilityProperties};
use crate::role::AccessibilityRole;
use frame_core::Rect;
use std::collections::HashMap;

pub struct AccessibilityTree {
    nodes: HashMap<u64, AccessibilityNode>,
    next_id: u64,
    root_id: Option<u64>,
    focus_order: Vec<u64>,
}

impl AccessibilityTree {
    pub fn new() -> Self {
        Self { nodes: HashMap::new(), next_id: 1, root_id: None, focus_order: Vec::new() }
    }

    pub fn add_node(&mut self, properties: AccessibilityProperties, bounds: Rect, parent: Option<u64>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let mut node = AccessibilityNode::new(id, properties, bounds);
        node.parent = parent;

        if parent.is_none() && self.root_id.is_none() {
            self.root_id = Some(id);
        }

        if let Some(parent_id) = parent {
            if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
                parent_node.children.push(id);
            }
        }

        if node.is_interactive() {
            self.focus_order.push(id);
        }

        self.nodes.insert(id, node);
        id
    }

    pub fn remove_node(&mut self, id: u64) {
        if let Some(node) = self.nodes.remove(&id) {
            if let Some(parent_id) = node.parent {
                if let Some(parent) = self.nodes.get_mut(&parent_id) {
                    parent.children.retain(|&c| c != id);
                }
            }
            for child_id in node.children {
                self.remove_node(child_id);
            }
        }
        self.focus_order.retain(|&x| x != id);
    }

    pub fn get(&self, id: u64) -> Option<&AccessibilityNode> { self.nodes.get(&id) }
    pub fn get_mut(&mut self, id: u64) -> Option<&mut AccessibilityNode> { self.nodes.get_mut(&id) }

    pub fn root(&self) -> Option<&AccessibilityNode> {
        self.root_id.and_then(|id| self.nodes.get(&id))
    }

    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn interactive_count(&self) -> usize { self.focus_order.len() }

    pub fn focus_order(&self) -> &[u64] { &self.focus_order }

    pub fn find_by_label(&self, label: &str) -> Vec<&AccessibilityNode> {
        self.nodes.values()
            .filter(|n| n.properties.label.as_ref().map(|l| l.contains(label)).unwrap_or(false))
            .collect()
    }

    pub fn find_by_role(&self, role: AccessibilityRole) -> Vec<&AccessibilityNode> {
        self.nodes.values().filter(|n| n.properties.role == role).collect()
    }

    pub fn children_of(&self, id: u64) -> Vec<&AccessibilityNode> {
        self.nodes.get(&id)
            .map(|n| n.children.iter().filter_map(|&c| self.nodes.get(&c)).collect())
            .unwrap_or_default()
    }

    pub fn update_bounds(&mut self, id: u64, bounds: Rect) {
        if let Some(node) = self.nodes.get_mut(&id) { node.bounds = bounds; }
    }

    pub fn update_property<F>(&mut self, id: u64, f: F)
    where F: FnOnce(&mut AccessibilityProperties) {
        if let Some(node) = self.nodes.get_mut(&id) { f(&mut node.properties); }
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.focus_order.clear();
        self.root_id = None;
    }
}

impl Default for AccessibilityTree {
    fn default() -> Self { Self::new() }
}
