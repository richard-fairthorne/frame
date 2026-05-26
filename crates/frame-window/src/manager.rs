use frame_core::plugin::{Plugin, PluginContext};
use frame_core::traits::window::{WindowHost, WindowConfig};
use frame_core::traits::renderer::WindowHandle;
use frame_core::{Size, WindowId};
use slotmap::SlotMap;

struct WindowEntry {
    title: String,
    size: Size,
}

pub struct WindowManager {
    windows: SlotMap<WindowId, WindowEntry>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: SlotMap::with_key(),
        }
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for WindowManager {
    fn init(&mut self, _ctx: &mut PluginContext<'_>) {}

    fn on_destroy(&mut self) {
        self.windows.clear();
    }
}

impl WindowHost for WindowManager {
    fn create_window(&mut self, config: WindowConfig) -> WindowHandle {
        let id = self.windows.insert(WindowEntry {
            title: config.title,
            size: config.size,
        });
        WindowHandle::new(id)
    }

    fn set_title(&mut self, handle: WindowHandle, title: &str) {
        if let Some(entry) = self.windows.get_mut(handle.id) {
            entry.title = title.to_string();
        }
    }

    fn resize(&mut self, handle: WindowHandle, size: Size) {
        if let Some(entry) = self.windows.get_mut(handle.id) {
            entry.size = size;
        }
    }
}
