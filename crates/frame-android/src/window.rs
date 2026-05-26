use frame_core::plugin::{Plugin, PluginContext};
use frame_core::traits::renderer::WindowHandle;
use frame_core::traits::window::{WindowConfig, WindowHost};
use frame_core::{Size, WindowId};
use slotmap::SlotMap;

struct AndroidWindow {
    title: String,
    size: Size,
}

pub struct AndroidWindowManager {
    windows: SlotMap<WindowId, AndroidWindow>,
}

impl AndroidWindowManager {
    pub fn new() -> Self {
        Self {
            windows: SlotMap::with_key(),
        }
    }
}

impl Default for AndroidWindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for AndroidWindowManager {
    fn init(&mut self, _ctx: &mut PluginContext<'_>) {}

    fn on_pause(&mut self) {}

    fn on_resume(&mut self) {}

    fn on_destroy(&mut self) {
        self.windows.clear();
    }
}

impl WindowHost for AndroidWindowManager {
    fn create_window(&mut self, config: WindowConfig) -> WindowHandle {
        let id = self.windows.insert(AndroidWindow {
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
