use std::sync::RwLock;
use std::sync::LazyLock;

type DeepLinkHandler = Box<dyn Fn(String) + Send + Sync>;

static DEEP_LINK_HANDLER: LazyLock<RwLock<Option<DeepLinkHandler>>> =
    LazyLock::new(|| RwLock::new(None));

pub fn set_deep_link_handler(handler: DeepLinkHandler) {
    *DEEP_LINK_HANDLER.write().unwrap() = Some(handler);
}

pub fn handle_deep_link(url: String) {
    if let Some(ref handler) = *DEEP_LINK_HANDLER.read().unwrap() {
        handler(url);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn setup_history_routing() {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    let handler = Closure::wrap(Box::new(|| {
        if let Some(window) = web_sys::window() {
            if let Ok(href) = window.location().href() {
                handle_deep_link(href);
            }
        }
    }) as Box<dyn Fn()>);

    if let Some(window) = web_sys::window() {
        let _ = window.add_event_listener_with_callback("popstate", handler.as_ref().unchecked_ref());
    }

    handler.forget();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn setup_history_routing() {}
