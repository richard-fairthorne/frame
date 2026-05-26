use frame_core::plugin::Plugin;
use frame_core::traits::window::{WindowHost, WindowConfig};
use frame_window::WindowManager;

#[test]
fn create_window() {
    let mut wm = WindowManager::new();
    let emitter = frame_core::plugin::EventEmitter::new();
    let mut ctx = frame_core::plugin::PluginContext::new(&emitter);
    wm.init(&mut ctx);
    let handle = wm.create_window(WindowConfig::default());
    wm.set_title(handle, "Test");
}
