use frame_core::plugin::Plugin;
use frame_core::traits::renderer::WindowHandle;
use frame_core::traits::window::{WindowConfig, WindowHost};

#[test]
fn macos_platform_init() {
    let mut platform = frame_macos::MacosPlatform::new();
    let emitter = frame_core::plugin::EventEmitter::new();
    let mut ctx = frame_core::plugin::PluginContext::new(&emitter);
    platform.init(&mut ctx);
}

#[test]
fn macos_window_create_and_configure() {
    let mut wm = frame_macos::MacosWindowManager::new();
    let emitter = frame_core::plugin::EventEmitter::new();
    let mut ctx = frame_core::plugin::PluginContext::new(&emitter);
    wm.init(&mut ctx);

    let handle = wm.create_window(WindowConfig {
        title: "Test Window".into(),
        size: frame_core::Size::new(640.0, 480.0),
        resizable: true,
    });

    wm.set_title(handle, "Updated Title");
    wm.resize(handle, frame_core::Size::new(1024.0, 768.0));

    wm.on_destroy();
}

#[test]
fn macos_multiple_windows() {
    let mut wm = frame_macos::MacosWindowManager::new();
    let emitter = frame_core::plugin::EventEmitter::new();
    let mut ctx = frame_core::plugin::PluginContext::new(&emitter);
    wm.init(&mut ctx);

    let h1 = wm.create_window(WindowConfig::default());
    let h2 = wm.create_window(WindowConfig::default());

    wm.set_title(h1, "Window 1");
    wm.set_title(h2, "Window 2");

    wm.on_destroy();
}

#[test]
fn macos_raw_window_handle() {
    let mut wm = frame_macos::MacosWindowManager::new();
    let emitter = frame_core::plugin::EventEmitter::new();
    let mut ctx = frame_core::plugin::PluginContext::new(&emitter);
    wm.init(&mut ctx);

    let handle = wm.create_window(WindowConfig {
        title: "Handle Test".into(),
        size: frame_core::Size::new(800.0, 600.0),
        resizable: true,
    });

    let raw = wm.raw_window_handle(&handle);
    #[cfg(target_os = "macos")]
    {
        use objc2::MainThreadMarker;
        if MainThreadMarker::new().is_some() {
            assert!(raw.is_some());
        }
    }

    wm.on_destroy();
}
