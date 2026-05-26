use frame_core::{Size, WindowId};
use std::collections::HashMap;
use vello::Scene;

pub trait SurfaceProvider: Send + Sync {
    fn create_surface(&mut self, id: WindowId, size: Size) -> Result<SurfaceHandle, SurfaceError>;
    fn resize_surface(&mut self, handle: &SurfaceHandle, size: Size);
    fn present(&mut self, handle: &SurfaceHandle, scene: &Scene) -> Result<(), SurfaceError>;
    fn destroy_surface(&mut self, handle: SurfaceHandle);
}

#[derive(Debug, Clone)]
pub struct SurfaceHandle {
    pub id: WindowId,
    pub width: f32,
    pub height: f32,
    pub raw_handle: Option<*mut std::ffi::c_void>,
}

unsafe impl Send for SurfaceHandle {}
unsafe impl Sync for SurfaceHandle {}

#[derive(Debug)]
pub enum SurfaceError {
    UnsupportedPlatform,
    SurfaceCreationFailed(String),
    RenderingFailed(String),
    InvalidHandle,
}

impl std::fmt::Display for SurfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SurfaceError::UnsupportedPlatform => write!(f, "unsupported platform"),
            SurfaceError::SurfaceCreationFailed(msg) => {
                write!(f, "surface creation failed: {msg}")
            }
            SurfaceError::RenderingFailed(msg) => write!(f, "rendering failed: {msg}"),
            SurfaceError::InvalidHandle => write!(f, "invalid surface handle"),
        }
    }
}

impl std::error::Error for SurfaceError {}

#[derive(Debug, Clone)]
pub struct FrameCapture {
    pub window_id: WindowId,
    pub width: u32,
    pub height: u32,
    pub pixel_count: usize,
}

pub struct HeadlessSurface {
    surfaces: HashMap<WindowId, SurfaceHandle>,
    frames: Vec<FrameCapture>,
}

impl HeadlessSurface {
    pub fn new() -> Self {
        Self {
            surfaces: HashMap::new(),
            frames: Vec::new(),
        }
    }

    pub fn captured_frames(&self) -> &[FrameCapture] {
        &self.frames
    }

    pub fn clear_captures(&mut self) {
        self.frames.clear();
    }
}

impl Default for HeadlessSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfaceProvider for HeadlessSurface {
    fn create_surface(&mut self, id: WindowId, size: Size) -> Result<SurfaceHandle, SurfaceError> {
        let handle = SurfaceHandle {
            id,
            width: size.width,
            height: size.height,
            raw_handle: None,
        };
        self.surfaces.insert(id, handle.clone());
        Ok(handle)
    }

    fn resize_surface(&mut self, handle: &SurfaceHandle, size: Size) {
        if let Some(surface) = self.surfaces.get_mut(&handle.id) {
            surface.width = size.width;
            surface.height = size.height;
        }
    }

    fn present(&mut self, handle: &SurfaceHandle, _scene: &Scene) -> Result<(), SurfaceError> {
        if self.surfaces.contains_key(&handle.id) {
            self.frames.push(FrameCapture {
                window_id: handle.id,
                width: handle.width as u32,
                height: handle.height as u32,
                pixel_count: (handle.width * handle.height) as usize,
            });
            Ok(())
        } else {
            Err(SurfaceError::InvalidHandle)
        }
    }

    fn destroy_surface(&mut self, handle: SurfaceHandle) {
        self.surfaces.remove(&handle.id);
    }
}
