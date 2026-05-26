use frame_core::plugin::{Plugin, PluginContext};
use frame_core::traits::platform::{
    NativeViewHandle, NativeViewRequest, PlatformContext, PlatformWindowHandle,
};
use frame_core::traits::renderer::WindowHandle;
use frame_core::traits::window::{WindowConfig, WindowHost};
use frame_core::{Rect, Size, WindowId};
use slotmap::SlotMap;
use std::any::Any;

use crate::native_view_embed::NativeViewManager;

pub struct AndroidPlatformContext {
    native_view_manager: NativeViewManager,
}

impl AndroidPlatformContext {
    pub fn new() -> Self {
        Self {
            native_view_manager: NativeViewManager::new(),
        }
    }
}

impl Default for AndroidPlatformContext {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformContext for AndroidPlatformContext {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidLifecycleState {
    Created,
    Started,
    Resumed,
    Paused,
    Stopped,
    Destroyed,
}

#[allow(dead_code)]
struct AndroidWindow {
    title: String,
    size: Size,
}

pub struct AndroidPlatform {
    initialized: bool,
    lifecycle_state: AndroidLifecycleState,
    density_dpi: u32,
    has_navigation_bar: bool,
}

#[cfg(target_os = "android")]
use std::sync::{Arc, Mutex};

#[cfg(target_os = "android")]
static ANDROID_PLATFORM: std::sync::OnceLock<Arc<Mutex<AndroidPlatform>>> =
    std::sync::OnceLock::new();

#[cfg(target_os = "android")]
impl AndroidPlatform {
    pub fn shared() -> Arc<Mutex<Self>> {
        ANDROID_PLATFORM
            .get_or_init(|| Arc::new(Mutex::new(AndroidPlatform::new())))
            .clone()
    }
}

impl AndroidPlatform {
    pub fn new() -> Self {
        Self {
            initialized: false,
            lifecycle_state: AndroidLifecycleState::Created,
            density_dpi: 480,
            has_navigation_bar: true,
        }
    }

    pub fn lifecycle_state(&self) -> AndroidLifecycleState {
        self.lifecycle_state
    }
    pub fn density_dpi(&self) -> u32 {
        self.density_dpi
    }
    pub fn density(&self) -> f32 {
        self.density_dpi as f32 / 160.0
    }
    pub fn has_navigation_bar(&self) -> bool {
        self.has_navigation_bar
    }
    pub fn navigation_bar_height(&self) -> f32 {
        if self.has_navigation_bar {
            48.0 * self.density()
        } else {
            0.0
        }
    }
    pub fn status_bar_height(&self) -> f32 {
        24.0 * self.density()
    }

    pub fn set_lifecycle(&mut self, state: AndroidLifecycleState) {
        self.lifecycle_state = state;
    }

    pub fn set_density_dpi(&mut self, dpi: u32) {
        self.density_dpi = dpi;
    }
}

impl Default for AndroidPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for AndroidPlatform {
    fn init(&mut self, _ctx: &mut PluginContext<'_>) {
        self.initialized = true;
    }
    fn on_pause(&mut self) {
        self.lifecycle_state = AndroidLifecycleState::Paused;
    }
    fn on_resume(&mut self) {
        self.lifecycle_state = AndroidLifecycleState::Resumed;
    }
    fn on_destroy(&mut self) {
        self.lifecycle_state = AndroidLifecycleState::Destroyed;
    }
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
    fn on_destroy(&mut self) {
        self.windows.clear();
    }
}

impl WindowHost for AndroidWindowManager {
    fn create_window(&mut self, config: WindowConfig) -> WindowHandle {
        let id = self.windows.insert(AndroidWindow {
            title: config.title,
            size: config.size,
        });
        WindowHandle::new(id)
    }

    fn set_title(&mut self, _handle: WindowHandle, _title: &str) {}
    fn resize(&mut self, handle: WindowHandle, size: Size) {
        if let Some(window) = self.windows.get_mut(handle.id) {
            window.size = size;
        }
    }
}

#[cfg(target_os = "android")]
pub struct JniBridge {
    vm: Option<jni::JavaVM>,
    vm_attached: bool,
}

#[cfg(target_os = "android")]
impl JniBridge {
    pub fn new() -> Self {
        Self {
            vm: None,
            vm_attached: false,
        }
    }

    pub fn from_vm(vm: jni::JavaVM) -> Self {
        Self {
            vm: Some(vm),
            vm_attached: true,
        }
    }

    pub fn attach_to_vm(&mut self) {
        self.vm_attached = true;
    }

    pub fn is_attached(&self) -> bool {
        self.vm_attached
    }

    pub fn detach(&mut self) {
        self.vm_attached = false;
    }

    pub fn vm(&self) -> Option<&jni::JavaVM> {
        self.vm.as_ref()
    }

    pub fn env(&self) -> Option<Result<jni::JNIEnv<'_>, jni::errors::Error>> {
        self.vm.as_ref().map(|vm| vm.get_env())
    }
}

#[cfg(target_os = "android")]
impl Default for JniBridge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(target_os = "android"))]
pub struct JniBridge {
    vm_attached: bool,
}

#[cfg(not(target_os = "android"))]
impl JniBridge {
    pub fn new() -> Self {
        Self { vm_attached: false }
    }

    pub fn attach_to_vm(&mut self) {
        self.vm_attached = true;
    }
    pub fn is_attached(&self) -> bool {
        self.vm_attached
    }
    pub fn detach(&mut self) {
        self.vm_attached = false;
    }
}

#[cfg(not(target_os = "android"))]
impl Default for JniBridge {
    fn default() -> Self {
        Self::new()
    }
}
