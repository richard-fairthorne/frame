use frame_core::plugin::{Plugin, PluginContext};
use frame_core::traits::platform::{
    NativeViewHandle, NativeViewRequest, PlatformContext, PlatformWindowHandle,
};
use frame_core::traits::window::WindowConfig;
use frame_core::{Rect, Size, WindowId};
use std::any::Any;

use crate::native_view::NativeViewManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceOrientation {
    Portrait,
    PortraitUpsideDown,
    LandscapeLeft,
    LandscapeRight,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SafeAreaInsets {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl SafeAreaInsets {
    pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    pub fn with_notch(top: f32) -> Self {
        Self {
            top,
            bottom: 34.0,
            left: 0.0,
            right: 0.0,
        }
    }

    pub fn horizontal_total(&self) -> f32 {
        self.left + self.right
    }

    pub fn vertical_total(&self) -> f32 {
        self.top + self.bottom
    }
}

pub struct IosPlatformContext {
    initialized: bool,
    native_view_manager: NativeViewManager,
}

impl IosPlatformContext {
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

impl Default for IosPlatformContext {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformContext for IosPlatformContext {
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

pub struct IosPlatform {
    context: IosPlatformContext,
    interface_orientation: InterfaceOrientation,
}

impl IosPlatform {
    pub fn new() -> Self {
        Self {
            context: IosPlatformContext::new(),
            interface_orientation: InterfaceOrientation::Portrait,
        }
    }

    pub fn interface_orientation(&self) -> InterfaceOrientation {
        self.interface_orientation
    }

    pub fn set_interface_orientation(&mut self, orientation: InterfaceOrientation) {
        self.interface_orientation = orientation;
    }

    pub fn safe_area_insets(&self) -> SafeAreaInsets {
        SafeAreaInsets::default()
    }

    pub fn status_bar_height(&self) -> f32 {
        match self.interface_orientation {
            InterfaceOrientation::Portrait | InterfaceOrientation::PortraitUpsideDown => 44.0,
            InterfaceOrientation::LandscapeLeft | InterfaceOrientation::LandscapeRight => 0.0,
        }
    }
}

impl Default for IosPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for IosPlatform {
    fn init(&mut self, _ctx: &mut PluginContext<'_>) {
        self.context.initialized = true;
    }

    fn on_pause(&mut self) {}

    fn on_resume(&mut self) {}

    fn on_destroy(&mut self) {}
}
