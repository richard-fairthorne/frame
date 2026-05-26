use frame_core::plugin::{Plugin, PluginContext};
use frame_core::traits::renderer::WindowHandle;
use frame_core::traits::window::{WindowConfig, WindowHost};
use frame_core::{Size, WindowId};
use slotmap::SlotMap;

#[cfg(target_os = "linux")]
use gtk::prelude::*;
#[cfg(target_os = "linux")]
use gtk::Window as GtkWindow;

#[cfg(target_os = "linux")]
struct LinuxWindow {
    gtk_window: Option<gtk::Window>,
    title: String,
    size: Size,
}

#[cfg(not(target_os = "linux"))]
struct LinuxWindow {
    title: String,
    size: Size,
}

#[cfg(target_os = "linux")]
unsafe impl Send for LinuxWindow {}
#[cfg(target_os = "linux")]
unsafe impl Sync for LinuxWindow {}

pub struct LinuxWindowManager {
    windows: SlotMap<WindowId, LinuxWindow>,
}

impl LinuxWindowManager {
    pub fn new() -> Self {
        Self {
            windows: SlotMap::with_key(),
        }
    }

    #[cfg(target_os = "linux")]
    pub fn gtk_window(&self, handle: &WindowHandle) -> Option<gtk::Window> {
        self.windows
            .get(handle.id)
            .and_then(|w| w.gtk_window.clone())
    }
}

impl Default for LinuxWindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for LinuxWindowManager {
    fn init(&mut self, _ctx: &mut PluginContext<'_>) {}

    fn on_pause(&mut self) {}

    fn on_resume(&mut self) {}

    fn on_destroy(&mut self) {
        #[cfg(target_os = "linux")]
        {
            for (_, window) in self.windows.iter() {
                if let Some(ref gtk_win) = window.gtk_window {
                    gtk_win.close();
                }
            }
        }
        self.windows.clear();
    }
}

impl WindowHost for LinuxWindowManager {
    fn create_window(&mut self, config: WindowConfig) -> WindowHandle {
        #[cfg(target_os = "linux")]
        {
            let gtk_window = create_gtk_window(&config);
            let id = self.windows.insert(LinuxWindow {
                gtk_window: Some(gtk_window),
                title: config.title,
                size: config.size,
            });
            WindowHandle::new(id)
        }

        #[cfg(not(target_os = "linux"))]
        {
            let id = self.windows.insert(LinuxWindow {
                title: config.title,
                size: config.size,
            });
            WindowHandle::new(id)
        }
    }

    fn set_title(&mut self, handle: WindowHandle, title: &str) {
        if let Some(window) = self.windows.get_mut(handle.id) {
            window.title = title.to_string();
            #[cfg(target_os = "linux")]
            {
                if let Some(ref gtk_win) = window.gtk_window {
                    gtk_win.set_title(title);
                }
            }
        }
    }

    fn resize(&mut self, handle: WindowHandle, size: Size) {
        if let Some(window) = self.windows.get_mut(handle.id) {
            window.size = size;
            #[cfg(target_os = "linux")]
            {
                if let Some(ref gtk_win) = window.gtk_window {
                    gtk_win.resize(size.width as i32, size.height as i32);
                }
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn create_gtk_window(config: &WindowConfig) -> gtk::Window {
    let window = gtk::Window::builder()
        .title(&config.title)
        .default_width(config.size.width as i32)
        .default_height(config.size.height as i32)
        .resizable(config.resizable)
        .build();
    window
}
