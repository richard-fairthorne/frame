use frame_core::{NativeViewHandle, NativeViewKind, NativeViewRequest, Rect};
use std::collections::HashMap;

#[cfg(target_os = "linux")]
use gtk::prelude::*;

struct ManagedView {
    #[cfg(target_os = "linux")]
    gtk_widget: Option<gtk::Widget>,
    rect: Rect,
    kind: NativeViewKind,
}

pub struct NativeViewManager {
    next_id: u64,
    views: HashMap<u64, ManagedView>,
    #[cfg(target_os = "linux")]
    overlay: Option<gtk::Overlay>,
}

impl NativeViewManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            views: HashMap::new(),
            #[cfg(target_os = "linux")]
            overlay: None,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn set_overlay(&mut self, overlay: gtk::Overlay) {
        self.overlay = Some(overlay);
    }

    pub fn embed(&mut self, request: NativeViewRequest) -> NativeViewHandle {
        let id = self.next_id;
        self.next_id += 1;

        #[cfg(target_os = "linux")]
        {
            let widget = self.overlay.as_ref().and_then(|overlay| {
                create_and_add_widget(&request, overlay)
            });

            self.views.insert(
                id,
                ManagedView {
                    gtk_widget: widget,
                    rect: request.rect,
                    kind: request.kind,
                },
            );
        }

        #[cfg(not(target_os = "linux"))]
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

            #[cfg(target_os = "linux")]
            if let Some(ref widget) = view.gtk_widget {
                let fixed = widget
                    .ancestor(gtk::Fixed::static_type())
                    .and_then(|w| w.downcast::<gtk::Fixed>().ok());
                if let Some(ref fixed) = fixed {
                    fixed.move_(
                        widget,
                        rect.origin.x as i32,
                        rect.origin.y as i32,
                    );
                }
                widget.set_size_request(
                    rect.size.width as i32,
                    rect.size.height as i32,
                );
            }
        }
    }

    pub fn remove(&mut self, handle: &NativeViewHandle) {
        if let Some(view) = self.views.remove(&handle.id) {
            #[cfg(target_os = "linux")]
            if let Some(widget) = view.gtk_widget {
                if let Some(parent) = widget.parent() {
                    parent.remove(&widget);
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

#[cfg(target_os = "linux")]
fn create_and_add_widget(
    request: &NativeViewRequest,
    overlay: &gtk::Overlay,
) -> Option<gtk::Widget> {
    let fixed = gtk::Fixed::new();

    match &request.kind {
        NativeViewKind::Web => {
            let socket = gtk::Socket::new();
            socket.set_size_request(
                request.rect.size.width as i32,
                request.rect.size.height as i32,
            );
            fixed.put(&socket, request.rect.origin.x as i32, request.rect.origin.y as i32);
        }
        _ => {
            let event_box = gtk::EventBox::new();
            event_box.set_size_request(
                request.rect.size.width as i32,
                request.rect.size.height as i32,
            );
            fixed.put(&event_box, request.rect.origin.x as i32, request.rect.origin.y as i32);
        }
    }

    fixed.show_all();
    overlay.add_overlay(&fixed);

    Some(fixed.upcast())
}
