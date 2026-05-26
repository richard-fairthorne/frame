#[cfg(target_arch = "wasm32")]
use frame_core::plugin::Plugin;
#[cfg(target_arch = "wasm32")]
use frame_core::traits::window::{WindowConfig, WindowHost};

#[cfg(target_arch = "wasm32")]
#[test]
fn web_platform_init() {
    let mut platform = frame_web::WebPlatform::new();
    let emitter = frame_core::plugin::EventEmitter::new();
    let mut ctx = frame_core::plugin::PluginContext::new(&emitter);
    platform.init(&mut ctx);
}

#[cfg(target_arch = "wasm32")]
#[test]
fn web_window_manager_create_window() {
    let mut wm = frame_web::WebWindowManager::new();
    let emitter = frame_core::plugin::EventEmitter::new();
    let mut ctx = frame_core::plugin::PluginContext::new(&emitter);
    wm.init(&mut ctx);

    let handle = wm.create_window(WindowConfig::default());
    wm.set_title(handle, "Test Window");
}
