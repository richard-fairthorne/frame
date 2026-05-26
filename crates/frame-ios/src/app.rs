use crate::native_view::NativeViewManager;
use crate::wgpu_surface::WgpuSurfaceProvider;
use crate::window::IosWindowManager;
use frame_core::traits::widget::Widget;
use frame_core::traits::window::WindowConfig;
use frame_core::WindowHost;
use frame_core::{Size, WindowId};
use frame_rendering::framecoord::Frame;

pub struct IosApp {
    frame: Frame<WgpuSurfaceProvider>,
    window_manager: IosWindowManager,
    native_view_manager: NativeViewManager,
    window_id: Option<WindowId>,
    window_handle: Option<frame_core::WindowHandle>,
    window_size: Size,
    title: String,
}

impl IosApp {
    pub fn new() -> Self {
        Self {
            frame: Frame::new(WgpuSurfaceProvider::new()),
            window_manager: IosWindowManager::new(),
            native_view_manager: NativeViewManager::new(),
            window_id: None,
            window_handle: None,
            window_size: Size::new(390.0, 844.0),
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
            resizable: false,
        });

        let scale = self.window_manager.scale_factor();
        let id = WindowId::default();
        self.frame.set_scale_factor(scale);
        self.frame
            .create_surface(id, self.window_size)
            .expect("failed to create surface");

        if let Some(layer) = self.window_manager.metal_layer(&handle) {
            self.frame
                .surface_provider_mut()
                .init_surface(
                    id,
                    layer,
                    (self.window_size.width * scale) as u32,
                    (self.window_size.height * scale) as u32,
                )
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

    pub fn window_handle(&self) -> Option<frame_core::traits::renderer::WindowHandle> {
        self.window_handle.clone()
    }

    pub fn window_manager(&self) -> &IosWindowManager {
        &self.window_manager
    }
}

impl Default for IosApp {
    fn default() -> Self {
        Self::new()
    }
}
