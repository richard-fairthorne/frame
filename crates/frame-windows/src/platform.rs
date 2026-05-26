use frame_core::plugin::{Plugin, PluginContext};
use frame_core::traits::platform::{
    NativeViewHandle, NativeViewRequest, PlatformContext, PlatformWindowHandle,
};
use frame_core::traits::window::WindowConfig;
use frame_core::{Rect, Size, WindowId};
use std::any::Any;

use crate::native_view::NativeViewManager;

pub struct WindowsPlatformContext {
    native_view_manager: NativeViewManager,
}

impl WindowsPlatformContext {
    pub fn new() -> Self {
        Self {
            native_view_manager: NativeViewManager::new(),
        }
    }
}

impl Default for WindowsPlatformContext {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformContext for WindowsPlatformContext {
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

pub struct WindowsPlatform {
    initialized: bool,
}

impl WindowsPlatform {
    pub fn new() -> Self {
        Self { initialized: false }
    }
}

impl Default for WindowsPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for WindowsPlatform {
    fn init(&mut self, _ctx: &mut PluginContext<'_>) {
        self.initialized = true;
    }

    fn on_pause(&mut self) {}

    fn on_resume(&mut self) {}

    fn on_destroy(&mut self) {
        self.initialized = false;
    }
}
