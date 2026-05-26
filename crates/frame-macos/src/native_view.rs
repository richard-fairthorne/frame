use frame_core::{NativeViewHandle, NativeViewKind, NativeViewRequest, Rect};
use std::collections::HashMap;

#[cfg(target_os = "macos")]
use objc2::runtime::AnyObject;
#[cfg(target_os = "macos")]
use objc2::MainThreadOnly;
#[cfg(target_os = "macos")]
use objc2_app_kit::NSView;
#[cfg(target_os = "macos")]
use objc2_foundation::{NSPoint, NSRect, NSSize};

struct ManagedView {
    #[cfg(target_os = "macos")]
    ns_view: Option<*mut AnyObject>,
    rect: Rect,
    kind: NativeViewKind,
}

#[cfg(target_os = "macos")]
unsafe impl Send for ManagedView {}
#[cfg(target_os = "macos")]
unsafe impl Sync for ManagedView {}

pub struct NativeViewManager {
    next_id: u64,
    views: HashMap<u64, ManagedView>,
}

impl NativeViewManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            views: HashMap::new(),
        }
    }

    pub fn embed(&mut self, request: NativeViewRequest) -> NativeViewHandle {
        let id = self.next_id;
        self.next_id += 1;

        #[cfg(target_os = "macos")]
        {
            let parent = find_key_window_content_view();
            let ns_view = parent.and_then(|content_view| {
                create_and_add_nsview(&request, &content_view)
            });

            self.views.insert(
                id,
                ManagedView {
                    ns_view,
                    rect: request.rect,
                    kind: request.kind,
                },
            );
        }

        #[cfg(not(target_os = "macos"))]
        {
            self.views.insert(
                id,
                ManagedView {
                    rect: request.rect,
                    kind: request.kind,
                },
            );
        }

        NativeViewHandle { id }
    }

    pub fn update_rect(&mut self, handle: &NativeViewHandle, rect: Rect) {
        if let Some(view) = self.views.get_mut(&handle.id) {
            view.rect = rect;

            #[cfg(target_os = "macos")]
            if let Some(ns_view_ptr) = view.ns_view {
                unsafe {
                    let ns_view: &NSView = &*ns_view_ptr.cast();
                    let frame = NSRect::new(
                        NSPoint::new(rect.origin.x as f64, rect.origin.y as f64),
                        NSSize::new(rect.size.width as f64, rect.size.height as f64),
                    );
                    ns_view.setFrame(frame);
                }
            }
        }
    }

    pub fn remove(&mut self, handle: &NativeViewHandle) {
        if let Some(view) = self.views.remove(&handle.id) {
            #[cfg(target_os = "macos")]
            if let Some(ns_view_ptr) = view.ns_view {
                unsafe {
                    let ns_view: &NSView = &*ns_view_ptr.cast();
                    ns_view.removeFromSuperview();
                    let _: () = objc2::msg_send![ns_view, release];
                }
            }
        }
    }
}

impl Default for NativeViewManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "macos")]
fn find_key_window_content_view() -> Option<objc2::rc::Retained<NSView>> {
    use objc2_app_kit::NSApplication;

    let mtm = objc2::MainThreadMarker::new()?;
    let app = NSApplication::sharedApplication(mtm);
    let window = app.keyWindow()?;
    Some(window.contentView()?)
}

#[cfg(target_os = "macos")]
fn create_and_add_nsview(
    request: &NativeViewRequest,
    parent: &NSView,
) -> Option<*mut AnyObject> {
    let mtm = objc2::MainThreadMarker::new()?;

    let frame = NSRect::new(
        NSPoint::new(request.rect.origin.x as f64, request.rect.origin.y as f64),
        NSSize::new(
            request.rect.size.width as f64,
            request.rect.size.height as f64,
        ),
    );

    unsafe {
        let allocated = NSView::alloc(mtm);
        let view = NSView::initWithFrame(allocated, frame);
        view.setAutoresizingMask(objc2_app_kit::NSAutoresizingMaskOptions::empty());

        match &request.kind {
            NativeViewKind::Web => {
                let bg_color: *mut AnyObject =
                    objc2::msg_send![objc2::runtime::AnyClass::get(c"NSColor")?, clearColor];
                let _: () = objc2::msg_send![&view, setWantsLayer: true];
                let layer: *mut AnyObject =
                    objc2::msg_send![objc2::runtime::AnyClass::get(c"CALayer")?, new];
                let _: () = objc2::msg_send![layer, setBackgroundColor: bg_color];
                let _: () = objc2::msg_send![&view, setLayer: layer];
            }
            _ => {}
        }

        parent.addSubview(&view);

        Some(objc2::rc::Retained::into_raw(view) as *mut AnyObject)
    }
}
