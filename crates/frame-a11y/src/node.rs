use crate::role::AccessibilityRole;
use frame_core::Rect;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AccessibilityProperties {
    pub role: AccessibilityRole,
    pub label: Option<String>,
    pub value: Option<String>,
    pub hint: Option<String>,
    pub enabled: bool,
    pub focused: bool,
    pub checked: Option<bool>,
    pub selected: Option<bool>,
    pub expanded: Option<bool>,
    pub level: Option<u32>,
    pub position: Option<u32>,
    pub size: Option<u32>,
    pub custom: HashMap<String, String>,
}

impl AccessibilityProperties {
    pub fn new(role: AccessibilityRole) -> Self {
        Self {
            role,
            label: None,
            value: None,
            hint: None,
            enabled: true,
            focused: false,
            checked: None,
            selected: None,
            expanded: None,
            level: None,
            position: None,
            size: None,
            custom: HashMap::new(),
        }
    }

    pub fn label(mut self, label: &str) -> Self { self.label = Some(label.to_string()); self }
    pub fn value(mut self, value: &str) -> Self { self.value = Some(value.to_string()); self }
    pub fn hint(mut self, hint: &str) -> Self { self.hint = Some(hint.to_string()); self }
    pub fn enabled(mut self, enabled: bool) -> Self { self.enabled = enabled; self }
    pub fn focused(mut self, focused: bool) -> Self { self.focused = focused; self }
    pub fn checked(mut self, checked: bool) -> Self { self.checked = Some(checked); self }
    pub fn selected(mut self, selected: bool) -> Self { self.selected = Some(selected); self }
    pub fn expanded(mut self, expanded: bool) -> Self { self.expanded = Some(expanded); self }
    pub fn level(mut self, level: u32) -> Self { self.level = Some(level); self }
    pub fn position(mut self, pos: u32) -> Self { self.position = Some(pos); self }
    pub fn size(mut self, size: u32) -> Self { self.size = Some(size); self }

    pub fn custom(mut self, key: &str, value: &str) -> Self {
        self.custom.insert(key.to_string(), value.to_string());
        self
    }

    pub fn description(&self) -> String {
        let mut parts = Vec::new();
        if let Some(ref label) = self.label { parts.push(label.clone()); }
        if let Some(ref value) = self.value { parts.push(value.clone()); }
        if let Some(ref hint) = self.hint { parts.push(hint.clone()); }
        parts.join(", ")
    }

    pub fn state_description(&self) -> String {
        let mut states = Vec::new();
        if !self.enabled { states.push("disabled"); }
        if self.focused { states.push("focused"); }
        if let Some(true) = self.checked { states.push("checked"); }
        if let Some(false) = self.checked { states.push("not checked"); }
        if let Some(true) = self.selected { states.push("selected"); }
        if let Some(true) = self.expanded { states.push("expanded"); }
        if let Some(false) = self.expanded { states.push("collapsed"); }
        states.join(", ")
    }
}

#[derive(Debug, Clone)]
pub struct AccessibilityNode {
    pub id: u64,
    pub properties: AccessibilityProperties,
    pub bounds: Rect,
    pub children: Vec<u64>,
    pub parent: Option<u64>,
}

impl AccessibilityNode {
    pub fn new(id: u64, properties: AccessibilityProperties, bounds: Rect) -> Self {
        Self { id, properties, bounds, children: Vec::new(), parent: None }
    }

    pub fn is_interactive(&self) -> bool {
        self.properties.role.is_interactive()
    }

    pub fn is_visible(&self) -> bool {
        self.bounds.size.width > 0.0 && self.bounds.size.height > 0.0
    }
}
