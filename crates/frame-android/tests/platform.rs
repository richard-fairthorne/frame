#![cfg(target_os = "android")]

use frame_core::plugin::Plugin;
use frame_core::traits::window::{WindowConfig, WindowHost};
use frame_android::{AndroidPlatform, AndroidWindowManager, AndroidLifecycleState, JniBridge};

#[test]
fn android_platform_init() {
    let mut platform = AndroidPlatform::new();
    let emitter = frame_core::plugin::EventEmitter::new();
    let mut ctx = frame_core::plugin::PluginContext::new(&emitter);
    platform.init(&mut ctx);
    assert_eq!(platform.lifecycle_state(), AndroidLifecycleState::Created);
}

#[test]
fn android_lifecycle() {
    let mut platform = AndroidPlatform::new();
    platform.set_lifecycle(AndroidLifecycleState::Resumed);
    assert_eq!(platform.lifecycle_state(), AndroidLifecycleState::Resumed);
    platform.on_pause();
    assert_eq!(platform.lifecycle_state(), AndroidLifecycleState::Paused);
}

#[test]
fn android_density() {
    let mut platform = AndroidPlatform::new();
    platform.set_density_dpi(320);
    assert_eq!(platform.density(), 2.0);
}

#[test]
fn android_window_create() {
    let mut wm = AndroidWindowManager::new();
    let handle = wm.create_window(WindowConfig {
        title: "Test".into(),
        size: frame_core::Size::new(360.0, 640.0),
        resizable: false,
    });
    wm.resize(handle, frame_core::Size::new(412.0, 915.0));
}

#[test]
fn jni_bridge() {
    let mut bridge = JniBridge::new();
    assert!(!bridge.is_attached());
    bridge.attach_to_vm();
    assert!(bridge.is_attached());
    bridge.detach();
    assert!(!bridge.is_attached());
}

#[test]
fn android_status_and_nav_bar() {
    let platform = AndroidPlatform::new();
    assert!(platform.status_bar_height() > 0.0);
    assert!(platform.navigation_bar_height() > 0.0);
}
