use frame_core::plugin::{Plugin, PluginContext};

#[cfg(target_os = "linux")]
use frame_core::traits::platform::{
    NativeViewHandle, NativeViewRequest, PlatformContext, PlatformWindowHandle,
};
#[cfg(target_os = "linux")]
use frame_core::traits::window::WindowConfig;
#[cfg(target_os = "linux")]
use frame_core::{Rect, Size, WindowId};
#[cfg(target_os = "linux")]
use std::any::Any;

#[cfg(target_os = "linux")]
use crate::native_view::NativeViewManager;

#[cfg(target_os = "linux")]
pub struct LinuxPlatformContext {
    initialized: bool,
    native_view_manager: NativeViewManager,
}

#[cfg(target_os = "linux")]
impl LinuxPlatformContext {
    pub fn new() -> Self {
        Self {
            initialized: false,
            native_view_manager: NativeViewManager::new(),
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn raw(&self) -> &dyn Any {
        self
    }
}

#[cfg(target_os = "linux")]
impl Default for LinuxPlatformContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "linux")]
impl PlatformContext for LinuxPlatformContext {
    fn create_window(&mut self, _config: WindowConfig) -> PlatformWindowHandle {
        PlatformWindowHandle {
            id: WindowId::default(),
        }
    }

    fn set_window_title(&mut self, _id: WindowId, _title: &str) {}

    fn resize_window(&mut self, _id: WindowId, _size: Size) {}

    fn embed_native_view(&mut self, view: NativeViewRequest) -> NativeViewHandle {
        self.native_view_manager.embed(view)
    }

    fn update_native_view_rect(&mut self, handle: &NativeViewHandle, rect: Rect) {
        self.native_view_manager.update_rect(handle, rect);
    }

    fn remove_native_view(&mut self, handle: &NativeViewHandle) {
        self.native_view_manager.remove(handle);
    }

    fn on_deep_link(&mut self, _handler: Box<dyn Fn(String) + Send + Sync>) {}

    fn run_on_main_thread(&self, task: Box<dyn FnOnce() + Send>) {
        task();
    }

    fn raw(&self) -> &dyn Any {
        self
    }
}

#[cfg(target_os = "linux")]
pub struct LinuxPlatform {
    context: LinuxPlatformContext,
}

#[cfg(target_os = "linux")]
impl LinuxPlatform {
    pub fn new() -> Self {
        Self {
            context: LinuxPlatformContext::new(),
        }
    }
}

#[cfg(target_os = "linux")]
impl Default for LinuxPlatform {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "linux")]
impl Plugin for LinuxPlatform {
    fn init(&mut self, _ctx: &mut PluginContext<'_>) {
        self.context.initialized = true;
    }

    fn on_pause(&mut self) {}

    fn on_resume(&mut self) {}

    fn on_destroy(&mut self) {}
}

#[cfg(not(target_os = "linux"))]
pub struct LinuxPlatform;

#[cfg(not(target_os = "linux"))]
impl LinuxPlatform {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(not(target_os = "linux"))]
impl Default for LinuxPlatform {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(target_os = "linux"))]
impl Plugin for LinuxPlatform {
    fn init(&mut self, _ctx: &mut PluginContext<'_>) {}
    fn on_pause(&mut self) {}
    fn on_resume(&mut self) {}
    fn on_destroy(&mut self) {}
}
