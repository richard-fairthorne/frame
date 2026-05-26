use crate::Size;
use crate::traits::renderer::WindowHandle;
use crate::plugin::Plugin;

pub struct WindowConfig {
    pub title: String,
    pub size: Size,
    pub resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: String::from("Frame App"),
            size: Size::new(800.0, 600.0),
            resizable: true,
        }
    }
}

pub trait WindowHost: Plugin {
    fn create_window(&mut self, config: WindowConfig) -> WindowHandle;
    fn set_title(&mut self, handle: WindowHandle, title: &str);
    fn resize(&mut self, handle: WindowHandle, size: Size);
}
