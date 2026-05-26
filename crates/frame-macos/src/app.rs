use crate::native_view::NativeViewManager;
use crate::window::MacosWindowManager;
use crate::wgpu_surface::WgpuSurfaceProvider;
use frame_core::traits::widget::Widget;
use frame_core::traits::window::WindowConfig;
use frame_core::WindowHost;
use frame_core::{Size, WindowId};
use frame_rendering::framecoord::Frame;

pub struct MacosApp {
    frame: Frame<WgpuSurfaceProvider>,
    window_manager: MacosWindowManager,
    native_view_manager: NativeViewManager,
    window_id: Option<WindowId>,
    window_handle: Option<frame_core::WindowHandle>,
    window_size: Size,
    title: String,
}

impl MacosApp {
    pub fn new() -> Self {
        Self {
            frame: Frame::new(WgpuSurfaceProvider::new()),
            window_manager: MacosWindowManager::new(),
            native_view_manager: NativeViewManager::new(),
            window_id: None,
            window_handle: None,
            window_size: Size::new(800.0, 600.0),
            title: "Frame App".into(),
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.window_size = Size::new(width, height);
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.into();
        self
    }

    pub fn create_window(&mut self) -> WindowId {
        let handle = self.window_manager.create_window(WindowConfig {
            title: self.title.clone(),
            size: self.window_size,
            resizable: true,
        });

        let scale = self.window_manager.backing_scale_factor(&handle);
        self.frame.set_scale_factor(scale);

        let id = WindowId::default();
        self.frame
            .create_surface(id, self.window_size)
            .expect("failed to create surface");

        if let Some(layer) = self.window_manager.metal_layer(&handle) {
            let physical_width = (self.window_size.width * scale) as u32;
            let physical_height = (self.window_size.height * scale) as u32;
            self.frame
                .surface_provider_mut()
                .init_surface(id, layer, physical_width, physical_height)
                .expect("failed to init wgpu surface");
        }

        self.window_id = Some(id);
        self.window_handle = Some(handle);
        id
    }

    pub fn tick(&mut self, root: &mut dyn Widget) -> bool {
        self.frame.tick(root)
    }

    pub fn render(&mut self, root: &mut dyn Widget) {
        if let Some(id) = self.window_id {
            let _ = self.frame.render(id, root);
        }
    }

    pub fn frame(&self) -> &Frame<WgpuSurfaceProvider> {
        &self.frame
    }

    pub fn frame_mut(&mut self) -> &mut Frame<WgpuSurfaceProvider> {
        &mut self.frame
    }

    pub fn window_size(&self) -> Size {
        self.window_size
    }

    pub fn resize(&mut self, size: Size) {
        self.window_size = size;
        if let Some(id) = self.window_id {
            self.frame.resize(id, size);
        }
    }

    pub fn check_window_resize(&mut self) {
        if let Some(handle) = &self.window_handle {
            let current = self.window_manager.content_size(handle);
            if current != self.window_size {
                self.resize(current);
            }
        }
    }
}

impl Default for MacosApp {
    fn default() -> Self {
        Self::new()
    }
}
