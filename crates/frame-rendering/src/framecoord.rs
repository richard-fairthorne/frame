use frame_core::render_loop::RenderLoop;
use frame_core::traits::widget::Widget;
use frame_core::{Size, WindowId};
use crate::pipeline::paint_widget_tree;
use crate::render_thread::{RenderCommand, RenderThreadHandle, SceneReceiver, spawn_render_thread};
use crate::surface::{SurfaceError, SurfaceHandle, SurfaceProvider};
use std::collections::HashMap;

pub struct Frame<T: SurfaceProvider> {
    render_loop: RenderLoop,
    surface_provider: T,
    surfaces: HashMap<WindowId, SurfaceHandle>,
    render_handle: Option<RenderThreadHandle>,
    scene_rx: Option<SceneReceiver>,
}

impl<T: SurfaceProvider> Frame<T> {
    pub fn new(surface_provider: T) -> Self {
        Self {
            render_loop: RenderLoop::new(),
            surface_provider,
            surfaces: HashMap::new(),
            render_handle: None,
            scene_rx: None,
        }
    }

    pub fn enable_render_thread(&mut self) {
        let (handle, rx) = spawn_render_thread();
        self.render_handle = Some(handle);
        self.scene_rx = Some(rx);
    }

    pub fn disable_render_thread(&mut self) {
        if let Some(handle) = self.render_handle.take() {
            handle.shutdown();
        }
        self.scene_rx = None;
    }

    pub fn render_thread_enabled(&self) -> bool {
        self.render_handle.is_some()
    }

    pub fn create_surface(
        &mut self,
        id: WindowId,
        size: Size,
    ) -> Result<SurfaceHandle, SurfaceError> {
        let physical = self.render_loop.physical_size();
        let handle = self.surface_provider.create_surface(id, physical)?;
        self.surfaces.insert(id, handle.clone());
        self.render_loop.set_size(size);
        Ok(handle)
    }

    pub fn set_scale_factor(&mut self, factor: f32) {
        self.render_loop.set_scale_factor(factor);
    }

    pub fn resize(&mut self, id: WindowId, size: Size) {
        let physical = Size::new(
            size.width * self.render_loop.scale_factor(),
            size.height * self.render_loop.scale_factor(),
        );
        if let Some(handle) = self.surfaces.get(&id) {
            self.surface_provider.resize_surface(handle, physical);
        }
        self.render_loop.set_size(size);
        if let Some(ref rt) = self.render_handle {
            rt.resize(physical.width as u32, physical.height as u32);
        }
    }

    pub fn tick(&mut self, root: &mut dyn Widget) -> bool {
        if !self.render_loop.is_dirty() {
            return false;
        }

        let constraints = self.render_loop.constraints();
        let _ = root.measure(constraints);
        self.render_loop.clear_dirty();
        true
    }

    pub fn render(&mut self, id: WindowId, root: &mut dyn Widget) -> Result<(), SurfaceError> {
        let handle = self
            .surfaces
            .get(&id)
            .cloned()
            .ok_or(SurfaceError::InvalidHandle)?;

        let constraints = self.render_loop.constraints();
        let scale = self.render_loop.scale_factor();
        let result = paint_widget_tree(root, constraints, scale);

        if let Some(ref rt) = self.render_handle {
            rt.submit(
                result.scene,
                handle.width as u32,
                handle.height as u32,
            );
            Ok(())
        } else {
            self.surface_provider.present(&handle, &result.scene)?;
            Ok(())
        }
    }

    pub fn present_pending(&mut self, id: WindowId) -> Result<(), SurfaceError> {
        let rx = self
            .scene_rx
            .as_ref()
            .ok_or(SurfaceError::InvalidHandle)?;

        let handle = self
            .surfaces
            .get(&id)
            .cloned()
            .ok_or(SurfaceError::InvalidHandle)?;

        if let Some(RenderCommand::Render { scene, .. }) = rx.drain_latest() {
            self.surface_provider.present(&handle, &scene)?;
        }
        Ok(())
    }

    pub fn mark_dirty(&self) {
        self.render_loop.mark_dirty();
    }

    pub fn is_dirty(&self) -> bool {
        self.render_loop.is_dirty()
    }

    pub fn size(&self) -> Size {
        self.render_loop.size()
    }

    pub fn scale_factor(&self) -> f32 {
        self.render_loop.scale_factor()
    }

    pub fn surface_provider_mut(&mut self) -> &mut T {
        &mut self.surface_provider
    }
}
