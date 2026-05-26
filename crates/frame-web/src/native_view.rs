use frame_core::{NativeViewHandle, NativeViewKind, NativeViewRequest, Rect};
use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlElement;

struct ManagedView {
    #[cfg(target_arch = "wasm32")]
    element: Option<HtmlElement>,
    #[cfg(not(target_arch = "wasm32"))]
    _element: (),
    rect: Rect,
    kind: NativeViewKind,
}

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

    #[cfg(target_arch = "wasm32")]
    pub fn embed(&mut self, request: NativeViewRequest) -> NativeViewHandle {
        let id = self.next_id;
        self.next_id += 1;

        let element = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|doc| {
                let tag = match &request.kind {
                    NativeViewKind::Web => "iframe",
                    NativeViewKind::Video => "video",
                    NativeViewKind::Map => "div",
                    NativeViewKind::Custom(name) => name,
                };
                doc.create_element(tag).ok()
            })
            .and_then(|el| {
                el.set_attribute(
                    "style",
                    &format!(
                        "position:absolute;left:{}px;top:{}px;width:{}px;height:{}px;pointer-events:auto;border:none;",
                        request.rect.origin.x,
                        request.rect.origin.y,
                        request.rect.size.width,
                        request.rect.size.height
                    ),
                )
                .ok()?;
                el.dyn_into::<HtmlElement>().ok()
            });

        if let Some(ref el) = element {
            let _ = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|doc| doc.body())
                .map(|body| body.append_child(el));
        }

        self.views.insert(
            id,
            ManagedView {
                element,
                rect: request.rect,
                kind: request.kind,
            },
        );

        NativeViewHandle { id }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn embed(&mut self, request: NativeViewRequest) -> NativeViewHandle {
        let id = self.next_id;
        self.next_id += 1;
        self.views.insert(
            id,
            ManagedView {
                _element: (),
                rect: request.rect,
                kind: request.kind,
            },
        );
        NativeViewHandle { id }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn update_rect(&mut self, handle: &NativeViewHandle, rect: Rect) {
        if let Some(view) = self.views.get_mut(&handle.id) {
            view.rect = rect;
            if let Some(ref el) = view.element {
                let _ = el.set_attribute(
                    "style",
                    &format!(
                        "position:absolute;left:{}px;top:{}px;width:{}px;height:{}px;pointer-events:auto;border:none;",
                        rect.origin.x, rect.origin.y, rect.size.width, rect.size.height
                    ),
                );
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn update_rect(&mut self, handle: &NativeViewHandle, rect: Rect) {
        if let Some(view) = self.views.get_mut(&handle.id) {
            view.rect = rect;
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn remove(&mut self, handle: &NativeViewHandle) {
        if let Some(view) = self.views.remove(&handle.id) {
            if let Some(el) = view.element {
                el.remove();
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn remove(&mut self, handle: &NativeViewHandle) {
        self.views.remove(&handle.id);
    }
}

impl Default for NativeViewManager {
    fn default() -> Self {
        Self::new()
    }
}
