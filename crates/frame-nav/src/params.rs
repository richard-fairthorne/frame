use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct RouteParams {
    params: HashMap<String, String>,
}

impl RouteParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.params.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }

    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }
}
