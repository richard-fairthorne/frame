use crate::router::Router;
use std::sync::RwLock;

pub struct Navigator {
    history: RwLock<Vec<String>>,
    router: Router,
}

impl Navigator {
    pub fn new(router: Router) -> Self {
        Self {
            history: RwLock::new(vec!["/".to_string()]),
            router,
        }
    }

    pub fn push(&self, path: &str) {
        self.history.write().unwrap().push(path.to_string());
    }

    pub fn replace(&self, path: &str) {
        let mut history = self.history.write().unwrap();
        if !history.is_empty() {
            let last = history.len() - 1;
            history[last] = path.to_string();
        } else {
            history.push(path.to_string());
        }
    }

    pub fn back(&self) {
        let mut history = self.history.write().unwrap();
        if history.len() > 1 {
            history.pop();
        }
    }

    pub fn current(&self) -> String {
        let history = self.history.read().unwrap();
        history.last().cloned().unwrap_or_default()
    }

    pub fn resolve_current(&self) -> Option<crate::router::RouteMatch> {
        self.router.resolve(&self.current())
    }
}
