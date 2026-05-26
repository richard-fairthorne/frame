use frame_core::traits::renderer::{RenderSurface, Renderer, WindowHandle};
use vello::util::RenderContext;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};

pub struct VelloSurface {
    surface: Option<Surface<'static>>,
    #[allow(dead_code)]
    config: Option<SurfaceConfiguration>,
    device: Option<Device>,
    queue: Option<Queue>,
    width: u32,
    height: u32,
}

impl VelloSurface {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn device(&self) -> Option<&Device> {
        self.device.as_ref()
    }

    pub fn queue(&self) -> Option<&Queue> {
        self.queue.as_ref()
    }

    pub fn surface(&self) -> Option<&Surface<'static>> {
        self.surface.as_ref()
    }
}

impl RenderSurface for VelloSurface {
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

pub struct VelloRenderer {
    context: RenderContext,
}

impl VelloRenderer {
    pub fn new() -> Self {
        Self {
            context: RenderContext::new(),
        }
    }

    pub fn context(&self) -> &RenderContext {
        &self.context
    }
}

impl Default for VelloRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for VelloRenderer {
    type Surface = VelloSurface;

    fn create_surface(&mut self, _window: &WindowHandle) -> Self::Surface {
        VelloSurface {
            surface: None,
            config: None,
            device: None,
            queue: None,
            width: 800,
            height: 600,
        }
    }

    fn render_frame(&mut self, _surface: &mut Self::Surface) {}
}
