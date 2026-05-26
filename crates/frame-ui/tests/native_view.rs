use frame_core::traits::widget::{RenderContext, Widget};
use frame_core::{Constraints, Point, Size, WidgetId};
use frame_ui::{NativeOverlay, NativeView, NativeViewContext, NativeViewFactory, NativeViewHandle};

struct TestNativeViewFactory;

impl NativeViewFactory for TestNativeViewFactory {
    fn create_view(&self, _size: Size, _context: &mut NativeViewContext) -> NativeViewHandle {
        NativeViewHandle::new(42 as *mut std::ffi::c_void, "test")
    }
    fn update_view(&self, _handle: &NativeViewHandle, _size: Size) {}
    fn destroy_view(&self, _handle: NativeViewHandle) {}
}

#[test]
fn native_view_measure() {
    let nv = NativeView::new().size(400.0, 300.0);
    let size = nv.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert_eq!(size, Size::new(400.0, 300.0));
}

#[test]
fn native_view_constrained() {
    let nv = NativeView::new().size(1000.0, 1000.0);
    let size = nv.measure(Constraints::tight(Size::new(500.0, 500.0)));
    assert_eq!(size, Size::new(500.0, 500.0));
}

#[test]
fn native_view_factory_creates_handle() {
    let mut nv = NativeView::new()
        .size(200.0, 100.0)
        .factory(Box::new(TestNativeViewFactory));
    let mut ctx = RenderContext {
        id_counter: WidgetId::default(),
    };
    nv.render(&mut ctx);
    let handle = nv.handle().unwrap();
    assert_eq!(handle.platform_tag(), "test");
    assert!(!handle.is_null());
}

#[test]
fn native_overlay_positioning() {
    let handle = NativeViewHandle::new(1 as *mut std::ffi::c_void, "macos");
    let overlay = NativeOverlay::new(handle)
        .position(10.0, 20.0)
        .size(300.0, 200.0);
    assert_eq!(overlay.position_value(), Point::new(10.0, 20.0));
    let size = overlay.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert_eq!(size, Size::new(300.0, 200.0));
}

#[test]
fn native_handle_null_check() {
    let null_handle = NativeViewHandle::new(std::ptr::null_mut(), "none");
    assert!(null_handle.is_null());
    let valid_handle = NativeViewHandle::new(1 as *mut std::ffi::c_void, "macos");
    assert!(!valid_handle.is_null());
}
