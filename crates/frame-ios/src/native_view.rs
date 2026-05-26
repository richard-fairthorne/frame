use frame_core::{NativeViewHandle, NativeViewKind, NativeViewRequest, Rect};
use std::collections::HashMap;

#[cfg(target_os = "ios")]
use objc2::runtime::AnyObject;
#[cfg(target_os = "ios")]
use objc2::MainThreadMarker;
#[cfg(target_os = "ios")]
use objc2_ui_kit::UIView;

struct ManagedView {
    #[cfg(target_os = "ios")]
    ui_view: Option<*mut AnyObject>,
    rect: Rect,
    kind: NativeViewKind,
}

#[cfg(target_os = "ios")]
unsafe impl Send for ManagedView {}
#[cfg(target_os = "ios")]
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

        #[cfg(target_os = "ios")]
        {
            let parent = find_root_view();
            let ui_view = parent.and_then(|parent_view| {
                create_and_add_uiview(&request, &parent_view)
            });

            self.views.insert(
                id,
                ManagedView {
                    ui_view,
                    rect: request.rect,
                    kind: request.kind,
                },
            );
        }

        #[cfg(not(target_os = "ios"))]
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

            #[cfg(target_os = "ios")]
            if let Some(ui_view_ptr) = view.ui_view {
                unsafe {
                    let ui_view: &UIView = &*ui_view_ptr.cast();
                    let cg_rect = objc2_foundation::CGRect::new(
                        objc2_foundation::CGPoint::new(rect.origin.x as f64, rect.origin.y as f64),
                        objc2_foundation::CGSize::new(rect.size.width as f64, rect.size.height as f64),
                    );
                    let _: () = objc2::msg_send![ui_view, setFrame: cg_rect];
                }
            }
        }
    }

    pub fn remove(&mut self, handle: &NativeViewHandle) {
        if let Some(view) = self.views.remove(&handle.id) {
            #[cfg(target_os = "ios")]
            if let Some(ui_view_ptr) = view.ui_view {
                unsafe {
                    let ui_view: &UIView = &*ui_view_ptr.cast();
                    let _: () = objc2::msg_send![ui_view, removeFromSuperview];
                    let _: () = objc2::msg_send![ui_view, release];
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

#[cfg(target_os = "ios")]
fn find_root_view() -> Option<objc2::rc::Retained<UIView>> {
    let mtm = MainThreadMarker::new()?;

    unsafe {
        let app: *mut AnyObject = objc2::msg_send![
            objc2::runtime::AnyClass::get(c"UIApplication")?,
            sharedApplication
        ];
        if app.is_null() {
            return None;
        }

        let key_window: *mut AnyObject = objc2::msg_send![app, keyWindow];
        if key_window.is_null() {
            let windows: *mut AnyObject = objc2::msg_send![app, windows];
            let count: usize = objc2::msg_send![windows, count];
            if count == 0 {
                return None;
            }
            return None;
        }

        let root_vc: *mut AnyObject = objc2::msg_send![key_window, rootViewController];
        if root_vc.is_null() {
            let subviews: *mut AnyObject = objc2::msg_send![key_window, subviews];
            let count: usize = objc2::msg_send![subviews, count];
            if count == 0 {
                return None;
            }
            let first: *mut AnyObject = objc2::msg_send![subviews, objectAtIndex: 0usize];
            if first.is_null() {
                return None;
            }
            let retained = objc2::rc::Retained::retain(first.cast());
            return Some(retained);
        }

        let view: *mut AnyObject = objc2::msg_send![root_vc, view];
        if view.is_null() {
            return None;
        }

        let retained = objc2::rc::Retained::retain(view.cast());
        Some(retained)
    }
}

#[cfg(target_os = "ios")]
fn create_and_add_uiview(
    request: &NativeViewRequest,
    parent: &UIView,
) -> Option<*mut AnyObject> {
    let mtm = MainThreadMarker::new()?;

    let frame = objc2_foundation::CGRect::new(
        objc2_foundation::CGPoint::new(request.rect.origin.x as f64, request.rect.origin.y as f64),
        objc2_foundation::CGSize::new(
            request.rect.size.width as f64,
            request.rect.size.height as f64,
        ),
    );

    unsafe {
        let allocated = mtm.alloc::<UIView>();
        let view = UIView::initWithFrame(allocated, frame);

        match &request.kind {
            NativeViewKind::Web => {
                let _: () = objc2::msg_send![&view, setOpaque: false];
                let bg_color: *mut AnyObject = objc2::msg_send![
                    objc2::runtime::AnyClass::get(c"UIColor")?,
                    clearColor
                ];
                let _: () = objc2::msg_send![&view, setBackgroundColor: bg_color];
            }
            _ => {}
        }

        let _: () = objc2::msg_send![parent, addSubview: &view];

        Some(objc2::rc::Retained::into_raw(view) as *mut AnyObject)
    }
}
