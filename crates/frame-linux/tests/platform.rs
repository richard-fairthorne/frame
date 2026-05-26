#![cfg(target_os = "linux")]

use frame_core::plugin::Plugin;
use frame_core::traits::window::{WindowConfig, WindowHost};

#[test]
fn linux_platform_init() {
    let mut platform = frame_linux::LinuxPlatform::new();
    let emitter = frame_core::plugin::EventEmitter::new();
    let mut ctx = frame_core::plugin::PluginContext::new(&emitter);
    platform.init(&mut ctx);
}

#[test]
fn linux_window_manager_create_window() {
    let mut wm = frame_linux::LinuxWindowManager::new();
    let emitter = frame_core::plugin::EventEmitter::new();
    let mut ctx = frame_core::plugin::PluginContext::new(&emitter);
    wm.init(&mut ctx);

    let handle = wm.create_window(WindowConfig::default());
    wm.set_title(handle, "Test Window");
}
