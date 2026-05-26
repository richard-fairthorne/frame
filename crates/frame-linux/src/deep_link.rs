use std::sync::RwLock;
use std::sync::LazyLock;

static DEEP_LINK_HANDLER: LazyLock<RwLock<Option<Box<dyn Fn(String) + Send + Sync>>>> =
    LazyLock::new(|| RwLock::new(None));

pub fn set_deep_link_handler(handler: Box<dyn Fn(String) + Send + Sync>) {
    *DEEP_LINK_HANDLER.write().unwrap() = Some(handler);
}

pub fn handle_deep_link(url: String) {
    if let Some(ref handler) = *DEEP_LINK_HANDLER.read().unwrap() {
        handler(url);
    }
}

#[cfg(target_os = "linux")]
pub fn register_uri_scheme(_scheme: &str) {
    let args: Vec<String> = std::env::args().collect();
    for arg in &args[1..] {
        if arg.contains("://") {
            handle_deep_link(arg.clone());
            return;
        }
    }
}

#[cfg(not(target_os = "linux"))]
pub fn register_uri_scheme(_scheme: &str) {}
