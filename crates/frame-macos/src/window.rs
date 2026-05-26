use frame_core::plugin::{Plugin, PluginContext};
use frame_core::traits::renderer::WindowHandle;
use frame_core::traits::window::{WindowConfig, WindowHost};
use frame_core::{Size, WindowId};
use slotmap::SlotMap;

#[cfg(target_os = "macos")]
use objc2::MainThreadMarker;
#[cfg(target_os = "macos")]
use objc2_app_kit::{NSBackingStoreType, NSWindow, NSWindowStyleMask};
#[cfg(target_os = "macos")]
use objc2_foundation::{NSPoint, NSRect, NSSize, NSString};

struct MacosWindow {
    ns_window: Option<*mut objc2::runtime::AnyObject>,
    metal_layer: Option<*mut std::ffi::c_void>,
    title: String,
    size: Size,
}

unsafe impl Send for MacosWindow {}
unsafe impl Sync for MacosWindow {}

pub struct MacosWindowManager {
    windows: SlotMap<WindowId, MacosWindow>,
}

impl MacosWindowManager {
    pub fn new() -> Self {
        Self {
            windows: SlotMap::with_key(),
        }
    }

    pub fn raw_window_handle(&self, handle: &WindowHandle) -> Option<*mut std::ffi::c_void> {
        self.windows
            .get(handle.id)
            .and_then(|w| w.ns_window.map(|p| p as *mut std::ffi::c_void))
    }

    pub fn metal_layer(&self, handle: &WindowHandle) -> Option<*mut std::ffi::c_void> {
        self.windows
            .get(handle.id)
            .and_then(|w| w.metal_layer)
    }

    pub fn backing_scale_factor(&self, handle: &WindowHandle) -> f32 {
        #[cfg(target_os = "macos")]
        if let Some(window) = self.windows.get(handle.id) {
            if let Some(ptr) = window.ns_window {
                unsafe {
                    let ns_window: &NSWindow = &*ptr.cast();
                    let scale: f64 = objc2::msg_send![ns_window, backingScaleFactor];
                    return scale as f32;
                }
            }
        }
        1.0
    }

    #[cfg(target_os = "macos")]
    pub fn content_size(&self, handle: &WindowHandle) -> Size {
        if let Some(window) = self.windows.get(handle.id) {
            if let Some(ptr) = window.ns_window {
                unsafe {
                    let ns_window: &NSWindow = &*ptr.cast();
                    if let Some(content_view) = ns_window.contentView() {
                        let bounds = content_view.bounds();
                        return Size::new(bounds.size.width as f32, bounds.size.height as f32);
                    }
                }
            }
        }
        self.windows
            .get(handle.id)
            .map(|w| w.size)
            .unwrap_or(Size::new(800.0, 600.0))
    }

    #[cfg(not(target_os = "macos"))]
    pub fn content_size(&self, handle: &WindowHandle) -> Size {
        self.windows
            .get(handle.id)
            .map(|w| w.size)
            .unwrap_or(Size::new(800.0, 600.0))
    }
}

impl Default for MacosWindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for MacosWindowManager {
    fn init(&mut self, _ctx: &mut PluginContext<'_>) {}

    fn on_destroy(&mut self) {
        #[cfg(target_os = "macos")]
        {
            for (_, window) in self.windows.iter_mut() {
                if let Some(ptr) = window.ns_window.take() {
                    unsafe {
                        let window: &NSWindow = &*ptr.cast();
                        window.close();
                    }
                }
            }
        }
        self.windows.clear();
    }
}

impl WindowHost for MacosWindowManager {
    fn create_window(&mut self, config: WindowConfig) -> WindowHandle {
        let (ns_window, metal_layer) = create_ns_window(config.size.width, config.size.height);

        #[cfg(target_os = "macos")]
        if let Some(ptr) = ns_window {
            unsafe {
                let window: &NSWindow = &*ptr.cast();
                let title = NSString::from_str(&config.title);
                window.setTitle(&title);
                window.center();
            }
        }

        let id = self.windows.insert(MacosWindow {
            ns_window,
            metal_layer,
            title: config.title,
            size: config.size,
        });
        WindowHandle::new(id)
    }

    fn set_title(&mut self, handle: WindowHandle, title: &str) {
        if let Some(window) = self.windows.get_mut(handle.id) {
            window.title = title.to_string();
            #[cfg(target_os = "macos")]
            if let Some(ptr) = window.ns_window {
                unsafe {
                    let ns_window: &NSWindow = &*ptr.cast();
                    let ns_title = NSString::from_str(title);
                    ns_window.setTitle(&ns_title);
                }
            }
        }
    }

    fn resize(&mut self, handle: WindowHandle, size: Size) {
        if let Some(window) = self.windows.get_mut(handle.id) {
            window.size = size;
            #[cfg(target_os = "macos")]
            if let Some(ptr) = window.ns_window {
                unsafe {
                    let ns_window: &NSWindow = &*ptr.cast();
                    let frame_size = NSSize::new(size.width as f64, size.height as f64);
                    ns_window.setContentSize(frame_size);
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn create_ns_window(width: f32, height: f32) -> (Option<*mut objc2::runtime::AnyObject>, Option<*mut std::ffi::c_void>) {
    let mtm = MainThreadMarker::new();
    let mtm = match mtm {
        Some(m) => m,
        None => return (None, None),
    };
    let rect = NSRect::new(
        NSPoint::new(0.0, 0.0),
        NSSize::new(width as f64, height as f64),
    );

    let style_mask = NSWindowStyleMask::Titled
        | NSWindowStyleMask::Closable
        | NSWindowStyleMask::Miniaturizable
        | NSWindowStyleMask::Resizable;

    unsafe {
        let allocated = mtm.alloc::<NSWindow>();
        let window = NSWindow::initWithContentRect_styleMask_backing_defer(
            allocated,
            rect,
            style_mask,
            NSBackingStoreType::Buffered,
            false,
        );

        let metal_layer = setup_metal_layer(&window);

        window.setReleasedWhenClosed(true);
        window.makeKeyAndOrderFront(None);
        let ptr = objc2::rc::Retained::into_raw(window) as *mut objc2::runtime::AnyObject;
        (Some(ptr), metal_layer)
    }
}

#[cfg(target_os = "macos")]
unsafe fn setup_metal_layer(window: &NSWindow) -> Option<*mut std::ffi::c_void> {
    use objc2::runtime::AnyClass;
    use objc2_app_kit::NSView;

    let view = window.contentView();
    let view: &NSView = view.as_ref()?;

    let layer_class = AnyClass::get(c"CAMetalLayer")?;
    let layer: *mut objc2::runtime::AnyObject = objc2::msg_send![layer_class, new];
    if layer.is_null() {
        return None;
    }

    view.setWantsLayer(true);
    let layer_ptr: *mut objc2::runtime::AnyObject = layer;
    let _: () = objc2::msg_send![view, setLayer: layer_ptr];

    let bounds = view.bounds();
    let _: () = objc2::msg_send![layer, setBounds: bounds];

    let window_scale: f64 = objc2::msg_send![window, backingScaleFactor];
    if window_scale > 1.0 {
        let _: () = objc2::msg_send![layer, setContentsScale: window_scale];
    }

    Some(layer as *mut std::ffi::c_void)
}

#[cfg(not(target_os = "macos"))]
fn create_ns_window(_width: f32, _height: f32) -> (Option<*mut std::ffi::c_void>, Option<*mut std::ffi::c_void>) {
    (None, None)
}
