use frame_core::plugin::{Plugin, PluginContext};
use frame_core::traits::renderer::WindowHandle;
use frame_core::traits::window::{WindowConfig, WindowHost};
use frame_core::{Size, WindowId};
use slotmap::SlotMap;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

struct WebWindow {
    title: String,
    size: Size,
    #[cfg(target_arch = "wasm32")]
    canvas: Option<HtmlCanvasElement>,
}

pub struct WebWindowManager {
    windows: SlotMap<WindowId, WebWindow>,
}

impl WebWindowManager {
    pub fn new() -> Self {
        Self {
            windows: SlotMap::with_key(),
        }
    }
}

impl Default for WebWindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for WebWindowManager {
    fn init(&mut self, _ctx: &mut PluginContext<'_>) {}

    fn on_pause(&mut self) {}

    fn on_resume(&mut self) {}

    fn on_destroy(&mut self) {
        self.windows.clear();
    }
}

#[cfg(target_arch = "wasm32")]
impl WindowHost for WebWindowManager {
    fn create_window(&mut self, config: WindowConfig) -> WindowHandle {
        let canvas = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|doc| doc.create_element("canvas").ok())
            .and_then(|el| el.dyn_into::<HtmlCanvasElement>().ok())
            .map(|canvas| {
                canvas.set_width(config.size.width as u32);
                canvas.set_height(config.size.height as u32);
                canvas.style().set_property("display", "block").ok();
                canvas.style().set_property("margin", "0 auto").ok();
                canvas
            });

        let id = self.windows.insert(WebWindow {
            title: config.title.clone(),
            size: config.size,
            canvas: canvas.clone(),
        });

        if let Some(ref canvas) = canvas {
            let _ = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|doc| doc.body())
                .map(|body| body.append_child(canvas));
        }

        WindowHandle::new(id)
    }

    fn set_title(&mut self, handle: WindowHandle, title: &str) {
        if let Some(window) = self.windows.get_mut(handle.id) {
            window.title = title.to_string();
            let _ = web_sys::window()
                .and_then(|w| w.document())
                .map(|doc| doc.set_title(title));
        }
    }

    fn resize(&mut self, handle: WindowHandle, size: Size) {
        if let Some(window) = self.windows.get_mut(handle.id) {
            window.size = size;
            if let Some(ref canvas) = window.canvas {
                canvas.set_width(size.width as u32);
                canvas.set_height(size.height as u32);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl WindowHost for WebWindowManager {
    fn create_window(&mut self, config: WindowConfig) -> WindowHandle {
        let id = self.windows.insert(WebWindow {
            title: config.title.clone(),
            size: config.size,
        });
        WindowHandle::new(id)
    }

    fn set_title(&mut self, handle: WindowHandle, title: &str) {
        if let Some(window) = self.windows.get_mut(handle.id) {
            window.title = title.to_string();
        }
    }

    fn resize(&mut self, handle: WindowHandle, size: Size) {
        if let Some(window) = self.windows.get_mut(handle.id) {
            window.size = size;
        }
    }
}
