use frame_core::plugin::{Plugin, PluginContext};
use frame_core::traits::platform::{
    NativeViewHandle, NativeViewRequest, PlatformContext, PlatformWindowHandle,
};
use frame_core::traits::window::WindowConfig;
use frame_core::{Rect, Size, WindowId};
use std::any::Any;

use crate::native_view::NativeViewManager;

pub struct MacosPlatformContext {
    initialized: bool,
    native_view_manager: NativeViewManager,
}

impl MacosPlatformContext {
    pub fn new() -> Self {
        Self {
            initialized: false,
            native_view_manager: NativeViewManager::new(),
        }
    }

    pub fn run_on_main_thread(&self, task: Box<dyn FnOnce() + Send>) {
        task();
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn raw(&self) -> &dyn Any {
        self
    }
}

impl Default for MacosPlatformContext {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformContext for MacosPlatformContext {
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

pub struct MacosPlatform {
    context: MacosPlatformContext,
}

impl MacosPlatform {
    pub fn new() -> Self {
        Self {
            context: MacosPlatformContext::new(),
        }
    }
}

impl Default for MacosPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for MacosPlatform {
    fn init(&mut self, _ctx: &mut PluginContext<'_>) {
        self.context.initialized = true;
    }

    fn on_pause(&mut self) {}

    fn on_resume(&mut self) {}

    fn on_destroy(&mut self) {}
}
