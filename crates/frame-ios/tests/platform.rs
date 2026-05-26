#![cfg(target_os = "ios")]

use frame_core::plugin::Plugin;
use frame_core::traits::window::{WindowConfig, WindowHost};
use frame_ios::{IosPlatform, IosWindowManager, InterfaceOrientation, SafeAreaInsets};

#[test]
fn ios_platform_init() {
    let mut platform = IosPlatform::new();
    let emitter = frame_core::plugin::EventEmitter::new();
    let mut ctx = frame_core::plugin::PluginContext::new(&emitter);
    platform.init(&mut ctx);
}

#[test]
fn ios_interface_orientation() {
    let mut platform = IosPlatform::new();
    assert_eq!(
        platform.interface_orientation(),
        InterfaceOrientation::Portrait
    );
    platform.set_interface_orientation(InterfaceOrientation::LandscapeLeft);
    assert_eq!(
        platform.interface_orientation(),
        InterfaceOrientation::LandscapeLeft
    );
}

#[test]
fn ios_status_bar_height() {
    let platform = IosPlatform::new();
    assert_eq!(platform.status_bar_height(), 44.0);
    let mut landscape = IosPlatform::new();
    landscape.set_interface_orientation(InterfaceOrientation::LandscapeLeft);
    assert_eq!(landscape.status_bar_height(), 0.0);
}

#[test]
fn ios_safe_area_insets() {
    let insets = SafeAreaInsets::with_notch(47.0);
    assert_eq!(insets.top, 47.0);
    assert_eq!(insets.bottom, 34.0);
    assert_eq!(insets.vertical_total(), 81.0);
}

#[test]
fn ios_window_create() {
    let mut wm = IosWindowManager::new();
    let handle = wm.create_window(WindowConfig {
        title: "Test".into(),
        size: frame_core::Size::new(375.0, 812.0),
        resizable: false,
    });
    wm.resize(handle, frame_core::Size::new(414.0, 896.0));
}
