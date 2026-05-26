use frame_core::plugin::{Plugin, PluginContext};
use frame_core::traits::renderer::WindowHandle;
use frame_core::traits::window::{WindowConfig, WindowHost};
use frame_core::{Size, WindowId};
use slotmap::SlotMap;

#[cfg(target_os = "ios")]
use objc2::runtime::AnyObject;

#[cfg(target_os = "ios")]
use objc2_foundation::CGPoint;

struct IosWindow {
    title: String,
    size: Size,
    #[cfg(target_os = "ios")]
    ui_window: Option<*mut AnyObject>,
    #[cfg(target_os = "ios")]
    metal_layer: Option<*mut std::ffi::c_void>,
}

unsafe impl Send for IosWindow {}
unsafe impl Sync for IosWindow {}

pub struct IosWindowManager {
    windows: SlotMap<WindowId, IosWindow>,
}

impl IosWindowManager {
    pub fn new() -> Self {
        Self {
            windows: SlotMap::with_key(),
        }
    }

    pub fn metal_layer(&self, handle: &WindowHandle) -> Option<*mut std::ffi::c_void> {
        self.windows.get(handle.id).and_then(|w| w.metal_layer)
    }

    #[cfg(target_os = "ios")]
    pub fn scale_factor(&self) -> f32 {
        use objc2::MainThreadMarker;
        let mtm = MainThreadMarker::new();
        match mtm {
            Some(m) => {
                let screen = objc2_ui_kit::UIScreen::mainScreen(m);
                let scale: f64 = unsafe { objc2::msg_send![screen, nativeScale] };
                scale as f32
            }
            None => 1.0,
        }
    }

    #[cfg(not(target_os = "ios"))]
    pub fn scale_factor(&self) -> f32 {
        1.0
    }

    #[cfg(target_os = "ios")]
    pub fn root_view_bounds(&self, handle: &WindowHandle) -> Option<frame_core::Size> {
        let window = self.windows.get(handle.id)?;
        let ui_window = window.ui_window?;
        unsafe {
            let subviews: *mut objc2::runtime::AnyObject = objc2::msg_send![ui_window, subviews];
            let count: usize = objc2::msg_send![subviews, count];
            if count == 0 {
                return None;
            }
            let view: *mut objc2::runtime::AnyObject = objc2::msg_send![subviews, objectAtIndex: 0usize];
            let bounds: objc2_foundation::NSRect = objc2::msg_send![view, bounds];
            Some(frame_core::Size::new(bounds.size.width as f32, bounds.size.height as f32))
        }
    }

    #[cfg(not(target_os = "ios"))]
    pub fn root_view_bounds(&self, _handle: &WindowHandle) -> Option<frame_core::Size> {
        None
    }

    #[cfg(target_os = "ios")]
    pub fn install_tap_gesture(&self, handle: &WindowHandle) {
        use objc2::runtime::{AnyClass, AnyObject, Sel};

        let window = match self.windows.get(handle.id) {
            Some(w) => w,
            None => return,
        };
        let ui_window = match window.ui_window {
            Some(ptr) => ptr,
            None => return,
        };

        unsafe {
            let subviews: *mut AnyObject = objc2::msg_send![ui_window, subviews];
            let count: usize = objc2::msg_send![subviews, count];
            if count == 0 {
                return;
            }
            let root_view: *mut AnyObject = objc2::msg_send![subviews, objectAtIndex: 0usize];

            let handler_class = match AnyClass::get(c"FrameTapHandler") {
                Some(cls) => cls,
                None => {
                    let superclass = AnyClass::get(c"NSObject").unwrap();
                    AnyClass::builder(c"FrameTapHandler", superclass)
                        .add_method(
                            Sel::register("handleTap:"),
                            frame_tap_handler as unsafe extern "C" fn(_, _, _),
                        )
                        .build()
                }
            };

            let handler: *mut AnyObject = objc2::msg_send![handler_class, new];

            let tap_class = AnyClass::get(c"UITapGestureRecognizer").unwrap();
            let tap: *mut AnyObject = objc2::msg_send![
                tap_class,
                initWithTarget: handler,
                action: Sel::register("handleTap:")
            ];

            let _: () = objc2::msg_send![root_view, addGestureRecognizer: tap];
            let _: () = objc2::msg_send![handler, release];
        }
    }

    #[cfg(not(target_os = "ios"))]
    pub fn install_tap_gesture(&self, _handle: &WindowHandle) {}
}

#[cfg(target_os = "ios")]
unsafe extern "C" fn frame_tap_handler(
    _this: &objc2::runtime::AnyObject,
    _cmd: objc2::runtime::Sel,
    gesture: &objc2::runtime::AnyObject,
) {
    let location: objc2_foundation::CGPoint = objc2::msg_send![
        gesture,
        locationInView: std::ptr::null::<objc2::runtime::AnyObject>()
    ];
    let point = frame_core::Point::new(location.x as f32, location.y as f32);
    frame_rendering::click::dispatch_click(point);
}

impl Default for IosWindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for IosWindowManager {
    fn init(&mut self, _ctx: &mut PluginContext<'_>) {}

    fn on_destroy(&mut self) {
        #[cfg(target_os = "ios")]
        {
            for (_, window) in self.windows.iter_mut() {
                if let Some(ptr) = window.ui_window.take() {
                    unsafe {
                        let _: () = objc2::msg_send![ptr, release];
                    }
                }
            }
        }
        self.windows.clear();
    }
}

impl WindowHost for IosWindowManager {
    fn create_window(&mut self, config: WindowConfig) -> WindowHandle {
        let (ui_window, metal_layer) = create_ui_window(config.size.width, config.size.height);

        let id = self.windows.insert(IosWindow {
            title: config.title,
            size: config.size,
            #[cfg(target_os = "ios")]
            ui_window,
            #[cfg(target_os = "ios")]
            metal_layer,
        });
        WindowHandle::new(id)
    }

    fn set_title(&mut self, _handle: WindowHandle, _title: &str) {}

    fn resize(&mut self, handle: WindowHandle, size: Size) {
        if let Some(window) = self.windows.get_mut(handle.id) {
            window.size = size;
        }
    }
}

#[cfg(target_os = "ios")]
fn create_ui_window(
    width: f32,
    height: f32,
) -> (Option<*mut AnyObject>, Option<*mut std::ffi::c_void>) {
    use objc2::MainThreadMarker;
    use objc2_foundation::{NSPoint, NSRect, NSSize};
    use objc2_ui_kit::UIWindow;

    let mtm = MainThreadMarker::new();
    let mtm = match mtm {
        Some(m) => m,
        None => return (None, None),
    };

    let screen = objc2_ui_kit::UIScreen::mainScreen(mtm);
    let screen_bounds = screen.bounds();

    unsafe {
        let allocated = mtm.alloc::<UIWindow>();
        let window = UIWindow::initWithFrame(allocated, screen_bounds);

        let metal_layer = setup_metal_layer(&window, width, height);

        window.makeKeyAndVisible();

        let ptr = objc2::rc::Retained::into_raw(window) as *mut AnyObject;
        (Some(ptr), metal_layer)
    }
}

#[cfg(target_os = "ios")]
unsafe fn setup_metal_layer(
    window: &UIWindow,
    _width: f32,
    _height: f32,
) -> Option<*mut std::ffi::c_void> {
    use objc2::runtime::AnyClass;
    use objc2_quartz_core::CAMetalLayer;
    use objc2_ui_kit::UIView;

    let root_view = objc2_ui_kit::UIView::new();
    window.addSubview(&root_view);

    let layer_class = AnyClass::get(c"CAMetalLayer")?;
    let layer: *mut objc2::runtime::AnyObject = objc2::msg_send![layer_class, new];
    if layer.is_null() {
        return None;
    }

    root_view.setAutoresizingMask(
        objc2_ui_kit::UIViewAutoresizing::FlexibleWidth
            | objc2_ui_kit::UIViewAutoresizing::FlexibleHeight,
    );

    let root_layer = root_view.layer();
    root_layer.addSublayer(&*(layer as *const CAMetalLayer));

    let bounds = root_view.bounds();
    let _: () = objc2::msg_send![layer, setBounds: bounds];

    let screen: &objc2_ui_kit::UIScreen = &*{
        let mtm = objc2::MainThreadMarker::new().unwrap();
        objc2_ui_kit::UIScreen::mainScreen(mtm)
    };
    let scale: f64 = objc2::msg_send![screen, nativeScale];
    let _: () = objc2::msg_send![layer, setContentsScale: scale];

    Some(layer as *mut std::ffi::c_void)
}

#[cfg(not(target_os = "ios"))]
fn create_ui_window(
    _width: f32,
    _height: f32,
) -> (Option<*mut std::ffi::c_void>, Option<*mut std::ffi::c_void>) {
    (None, None)
}
