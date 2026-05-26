use crate::{Constraints, Size};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VsyncMode {
    SyncToDisplay,
    OnDemand,
}

pub struct RenderLoop {
    dirty: AtomicBool,
    root_size: RwLock<Size>,
    scale_factor: RwLock<f32>,
    vsync_mode: RwLock<VsyncMode>,
}

impl RenderLoop {
    pub fn new() -> Self {
        Self {
            dirty: AtomicBool::new(true),
            root_size: RwLock::new(Size::new(800.0, 600.0)),
            scale_factor: RwLock::new(1.0),
            vsync_mode: RwLock::new(VsyncMode::SyncToDisplay),
        }
    }

    pub fn mark_dirty(&self) {
        self.dirty.store(true, Ordering::Relaxed);
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty.load(Ordering::Relaxed)
    }

    pub fn set_size(&self, size: Size) {
        *self.root_size.write().unwrap() = size;
        self.mark_dirty();
    }

    pub fn size(&self) -> Size {
        *self.root_size.read().unwrap()
    }

    pub fn set_scale_factor(&self, factor: f32) {
        *self.scale_factor.write().unwrap() = factor.max(1.0);
        self.mark_dirty();
    }

    pub fn scale_factor(&self) -> f32 {
        *self.scale_factor.read().unwrap()
    }

    pub fn constraints(&self) -> Constraints {
        Constraints::tight(self.size())
    }

    pub fn physical_size(&self) -> Size {
        let s = self.scale_factor();
        let logical = self.size();
        Size::new(logical.width * s, logical.height * s)
    }

    pub fn clear_dirty(&self) {
        self.dirty.store(false, Ordering::Relaxed);
    }

    pub fn set_vsync_mode(&self, mode: VsyncMode) {
        *self.vsync_mode.write().unwrap() = mode;
    }

    pub fn vsync_mode(&self) -> VsyncMode {
        *self.vsync_mode.read().unwrap()
    }

    pub fn should_render(&self) -> bool {
        match self.vsync_mode() {
            VsyncMode::SyncToDisplay => true,
            VsyncMode::OnDemand => self.is_dirty(),
        }
    }
}

impl Default for RenderLoop {
    fn default() -> Self {
        Self::new()
    }
}
