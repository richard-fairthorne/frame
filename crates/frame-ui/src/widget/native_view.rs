use frame_core::traits::widget::{RenderContext, Widget, WidgetOutput};
use frame_core::{Constraints, Point, Size};

pub struct NativeViewHandle {
    ptr: *mut std::ffi::c_void,
    platform_tag: &'static str,
}

impl NativeViewHandle {
    pub fn new(ptr: *mut std::ffi::c_void, platform_tag: &'static str) -> Self {
        Self { ptr, platform_tag }
    }

    pub fn as_ptr(&self) -> *mut std::ffi::c_void {
        self.ptr
    }

    pub fn platform_tag(&self) -> &'static str {
        self.platform_tag
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

unsafe impl Send for NativeViewHandle {}
unsafe impl Sync for NativeViewHandle {}

pub trait NativeViewFactory: Send + Sync {
    fn create_view(&self, size: Size, context: &mut NativeViewContext) -> NativeViewHandle;
    fn update_view(&self, handle: &NativeViewHandle, size: Size);
    fn destroy_view(&self, handle: NativeViewHandle);
}

pub struct NativeViewContext {
    pub parent_handle: Option<NativeViewHandle>,
}

impl NativeViewContext {
    pub fn new() -> Self {
        Self {
            parent_handle: None,
        }
    }

    pub fn with_parent(parent: NativeViewHandle) -> Self {
        Self {
            parent_handle: Some(parent),
        }
    }
}

impl Default for NativeViewContext {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NativeView {
    handle: Option<NativeViewHandle>,
    requested_size: Size,
    factory: Option<Box<dyn NativeViewFactory>>,
}

impl NativeView {
    pub fn new() -> Self {
        Self {
            handle: None,
            requested_size: Size::new(300.0, 200.0),
            factory: None,
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.requested_size = Size::new(width, height);
        self
    }

    pub fn factory(mut self, factory: Box<dyn NativeViewFactory>) -> Self {
        self.factory = Some(factory);
        self
    }

    pub fn handle(&self) -> Option<&NativeViewHandle> {
        self.handle.as_ref()
    }

    pub fn set_handle(&mut self, handle: NativeViewHandle) {
        self.handle = Some(handle);
    }

    pub fn requested_size(&self) -> Size {
        self.requested_size
    }
}

impl Default for NativeView {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for NativeView {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(self.requested_size)
    }

    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        if self.handle.is_none() {
            if let Some(factory) = &self.factory {
                let mut native_ctx = NativeViewContext::new();
                let handle = factory.create_view(self.requested_size, &mut native_ctx);
                self.handle = Some(handle);
            }
        }
        WidgetOutput::None
    }
}

pub struct NativeOverlay {
    handle: Option<NativeViewHandle>,
    position: Point,
    size: Size,
}

impl NativeOverlay {
    pub fn new(handle: NativeViewHandle) -> Self {
        Self {
            handle: Some(handle),
            position: Point::ZERO,
            size: Size::new(300.0, 200.0),
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = Point::new(x, y);
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = Size::new(width, height);
        self
    }

    pub fn handle(&self) -> Option<&NativeViewHandle> {
        self.handle.as_ref()
    }

    pub fn position_value(&self) -> Point {
        self.position
    }
}

impl Widget for NativeOverlay {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(self.size)
    }

    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::None
    }
}
