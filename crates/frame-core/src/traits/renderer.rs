use crate::WindowId;

#[derive(Debug, Clone, Copy)]
pub struct WindowHandle {
    pub id: WindowId,
}

impl WindowHandle {
    pub fn new(id: WindowId) -> Self {
        Self { id }
    }
}

pub trait RenderSurface {
    fn resize(&mut self, width: u32, height: u32);
}

pub trait Renderer: 'static {
    type Surface: RenderSurface;

    fn create_surface(&mut self, window: &WindowHandle) -> Self::Surface;
    fn render_frame(&mut self, surface: &mut Self::Surface);
}
