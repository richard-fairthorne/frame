#[cfg(target_arch = "wasm32")]
use frame_core::{Size, WindowId};
#[cfg(target_arch = "wasm32")]
use frame_rendering::surface::{SurfaceError, SurfaceHandle, SurfaceProvider};
#[cfg(target_arch = "wasm32")]
use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use vello::Scene;
#[cfg(target_arch = "wasm32")]
use vello::util::RenderContext;
#[cfg(target_arch = "wasm32")]
use vello::wgpu;

#[cfg(target_arch = "wasm32")]
struct SurfaceEntry {
    handle: SurfaceHandle,
    surface: Option<wgpu::Surface<'static>>,
    config: Option<wgpu::SurfaceConfiguration>,
    dev_id: Option<usize>,
}

#[cfg(target_arch = "wasm32")]
pub struct WebSurfaceProvider {
    render_ctx: RenderContext,
    renderer: Option<vello::Renderer>,
    entries: HashMap<WindowId, SurfaceEntry>,
    surface_format: Option<wgpu::TextureFormat>,
}

#[cfg(target_arch = "wasm32")]
unsafe impl Send for WebSurfaceProvider {}
#[cfg(target_arch = "wasm32")]
unsafe impl Sync for WebSurfaceProvider {}

#[cfg(target_arch = "wasm32")]
impl WebSurfaceProvider {
    pub fn new() -> Self {
        Self {
            render_ctx: RenderContext::new(),
            renderer: None,
            entries: HashMap::new(),
            surface_format: None,
        }
    }

    pub async fn init_surface_from_canvas(
        &mut self,
        window_id: WindowId,
        canvas: &web_sys::HtmlCanvasElement,
        width: u32,
        height: u32,
    ) -> Result<(), SurfaceError> {
        use wasm_bindgen::JsCast;

        let entry = self
            .entries
            .get_mut(&window_id)
            .ok_or(SurfaceError::InvalidHandle)?;

        let surface = self
            .render_ctx
            .instance
            .create_surface_from_canvas(canvas)
            .map_err(|e| SurfaceError::SurfaceCreationFailed(format!("{e}")))?;

        let dev_id = self
            .render_ctx
            .device(Some(&surface))
            .await
            .ok_or(SurfaceError::SurfaceCreationFailed(
                "no compatible device".into(),
            ))?;

        let device_handle = &self.render_ctx.devices[dev_id];
        let capabilities = surface.get_capabilities(device_handle.adapter());
        let format = capabilities
            .formats
            .iter()
            .find(|f| {
                matches!(
                    f,
                    wgpu::TextureFormat::Rgba8Unorm
                        | wgpu::TextureFormat::Bgra8Unorm
                        | wgpu::TextureFormat::Rgba8UnormSrgb
                        | wgpu::TextureFormat::Bgra8UnormSrgb
                )
            })
            .copied()
            .ok_or(SurfaceError::SurfaceCreationFailed(
                "unsupported surface format".into(),
            ))?;

        self.surface_format = Some(format);

        if self.renderer.is_none() {
            self.renderer = Some(
                vello::Renderer::new(
                    &device_handle.device,
                    vello::RendererOptions {
                        surface_format: Some(format),
                        use_cpu: true,
                        antialiasing_support: vello::AaSupport::area_only(),
                        num_init_threads: std::num::NonZeroUsize::new(1),
                    },
                )
                .map_err(|e| SurfaceError::SurfaceCreationFailed(format!("{e}")))?,
            );
        }

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device_handle.device, &config);

        entry.surface = Some(surface);
        entry.config = Some(config);
        entry.dev_id = Some(dev_id);

        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for WebSurfaceProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_arch = "wasm32")]
impl SurfaceProvider for WebSurfaceProvider {
    fn create_surface(&mut self, id: WindowId, size: Size) -> Result<SurfaceHandle, SurfaceError> {
        let handle = SurfaceHandle {
            id,
            width: size.width,
            height: size.height,
            raw_handle: None,
        };
        self.entries.insert(
            id,
            SurfaceEntry {
                handle: handle.clone(),
                surface: None,
                config: None,
                dev_id: None,
            },
        );
        Ok(handle)
    }

    fn resize_surface(&mut self, handle: &SurfaceHandle, size: Size) {
        if let Some(entry) = self.entries.get_mut(&handle.id) {
            entry.handle.width = size.width;
            entry.handle.height = size.height;
            if let (Some(surface), Some(config), Some(dev_id)) =
                (&entry.surface, &mut entry.config, entry.dev_id)
            {
                config.width = size.width as u32;
                config.height = size.height as u32;
                let device = &self.render_ctx.devices[dev_id].device;
                surface.configure(device, config);
            }
        }
    }

    fn present(&mut self, handle: &SurfaceHandle, scene: &Scene) -> Result<(), SurfaceError> {
        let entry = self
            .entries
            .get_mut(&handle.id)
            .ok_or(SurfaceError::InvalidHandle)?;

        let surface = entry.surface.as_ref().ok_or(SurfaceError::RenderingFailed(
            "surface not initialized — call init_surface_from_canvas first".into(),
        ))?;
        let dev_id = entry
            .dev_id
            .ok_or(SurfaceError::RenderingFailed("no device".into()))?;
        let device_handle = &self.render_ctx.devices[dev_id];
        let renderer = self
            .renderer
            .as_mut()
            .ok_or(SurfaceError::RenderingFailed("no renderer".into()))?;

        let texture = surface
            .get_current_texture()
            .map_err(|e| SurfaceError::RenderingFailed(format!("get_current_texture: {e}")))?;

        renderer
            .render_to_surface(
                &device_handle.device,
                &device_handle.queue,
                scene,
                &texture,
                &vello::RenderParams {
                    base_color: vello::peniko::Color::from_rgba8(255, 255, 255, 255),
                    width: entry.handle.width as u32,
                    height: entry.handle.height as u32,
                    antialiasing_method: vello::AaConfig::Area,
                },
            )
            .map_err(|e| SurfaceError::RenderingFailed(format!("render: {e}")))?;

        texture.present();
        device_handle.queue.submit([]);
        Ok(())
    }

    fn destroy_surface(&mut self, handle: SurfaceHandle) {
        self.entries.remove(&handle.id);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct WebSurfaceProvider;

#[cfg(not(target_arch = "wasm32"))]
impl WebSurfaceProvider {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for WebSurfaceProvider {
    fn default() -> Self {
        Self::new()
    }
}
