use frame_core::{Size, WindowId};
use frame_rendering::surface::{SurfaceError, SurfaceHandle, SurfaceProvider};
use std::collections::HashMap;
use vello::util::RenderContext;
use vello::wgpu;

pub struct WgpuSurfaceProvider {
    render_ctx: RenderContext,
    renderer: Option<vello::Renderer>,
    entries: HashMap<WindowId, SurfaceEntry>,
    surface_format: Option<wgpu::TextureFormat>,
}

struct SurfaceEntry {
    handle: SurfaceHandle,
    surface: Option<wgpu::Surface<'static>>,
    config: Option<wgpu::SurfaceConfiguration>,
    dev_id: Option<usize>,
}

unsafe impl Send for WgpuSurfaceProvider {}
unsafe impl Sync for WgpuSurfaceProvider {}

impl WgpuSurfaceProvider {
    pub fn new() -> Self {
        Self {
            render_ctx: RenderContext::new(),
            renderer: None,
            entries: HashMap::new(),
            surface_format: None,
        }
    }

    pub fn init_surface(
        &mut self,
        window_id: WindowId,
        hwnd: *mut std::ffi::c_void,
        width: u32,
        height: u32,
    ) -> Result<(), SurfaceError> {
        let entry = self
            .entries
            .get_mut(&window_id)
            .ok_or(SurfaceError::InvalidHandle)?;

        let raw_window_handle = raw_window_handle::RawWindowHandle::Win32(
            raw_window_handle::Win32WindowHandle::new(
                std::num::NonZeroIsize::new_unchecked(hwnd as isize),
            ),
        );
        let raw_display_handle = raw_window_handle::RawDisplayHandle::Windows(
            raw_window_handle::WindowsDisplayHandle::new(),
        );
        let surface = unsafe {
            self.render_ctx
                .instance
                .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle,
                    raw_window_handle,
                })
                .map_err(|e| SurfaceError::SurfaceCreationFailed(format!("{e}")))?
        };

        let dev_id = block_on(self.render_ctx.device(Some(&surface)))
            .ok_or(SurfaceError::SurfaceCreationFailed("no compatible device".into()))?;

        let device_handle = &self.render_ctx.devices[dev_id];
        let capabilities = surface.get_capabilities(device_handle.adapter());
        let format = capabilities
            .formats
            .iter()
            .find(|f| {
                matches!(
                    f,
                    wgpu::TextureFormat::Rgba8Unorm | wgpu::TextureFormat::Bgra8Unorm
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
                        use_cpu: false,
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

impl Default for WgpuSurfaceProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfaceProvider for WgpuSurfaceProvider {
    fn create_surface(
        &mut self,
        id: WindowId,
        size: Size,
    ) -> Result<SurfaceHandle, SurfaceError> {
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

    fn present(
        &mut self,
        handle: &SurfaceHandle,
        scene: &vello::Scene,
    ) -> Result<(), SurfaceError> {
        let entry = self
            .entries
            .get_mut(&handle.id)
            .ok_or(SurfaceError::InvalidHandle)?;

        let surface = entry
            .surface
            .as_ref()
            .ok_or(SurfaceError::RenderingFailed("surface not initialized".into()))?;
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

fn block_on<F: std::future::Future>(fut: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        |_| RawWaker::new(std::ptr::null(), &VTABLE),
        |_| {},
        |_| {},
        |_| {},
    );
    let raw = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw) };
    let mut cx = Context::from_waker(&waker);

    let mut fut = std::pin::pin!(fut);
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(val) => return val,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}
