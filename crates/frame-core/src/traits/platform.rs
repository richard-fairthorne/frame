use std::any::Any;

use crate::{Rect, Size, WindowId};
use crate::traits::window::WindowConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeViewHandle {
    pub id: u64,
}

#[derive(Debug, Clone)]
pub enum NativeViewKind {
    Map,
    Web,
    Video,
    Custom(&'static str),
}

#[derive(Debug)]
pub struct NativeViewRequest {
    pub kind: NativeViewKind,
    pub rect: Rect,
    pub params: Box<dyn Any + Send + Sync>,
}

pub struct PlatformWindowHandle {
    pub id: WindowId,
}

pub trait PlatformContext: Send + Sync {
    fn create_window(&mut self, config: WindowConfig) -> PlatformWindowHandle;
    fn set_window_title(&mut self, id: WindowId, title: &str);
    fn resize_window(&mut self, id: WindowId, size: Size);

    fn embed_native_view(&mut self, view: NativeViewRequest) -> NativeViewHandle;
    fn update_native_view_rect(&mut self, handle: &NativeViewHandle, rect: Rect);
    fn remove_native_view(&mut self, handle: &NativeViewHandle);

    fn on_deep_link(&mut self, handler: Box<dyn Fn(String) + Send + Sync>);
    fn run_on_main_thread(&self, task: Box<dyn FnOnce() + Send>);
    fn raw(&self) -> &dyn Any;
}
