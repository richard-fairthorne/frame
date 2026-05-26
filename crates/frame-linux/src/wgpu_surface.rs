use frame_core::{Size, WindowId};
use frame_rendering::surface::{SurfaceError, SurfaceHandle, SurfaceProvider};
use std::collections::HashMap;
#[cfg(target_os = "linux")]
use vello::util::RenderContext;
#[cfg(target_os = "linux")]
use vello::wgpu;

#[cfg(target_os = "linux")]
use gtk::gdk;

pub struct WgpuSurfaceProvider {
    #[cfg(target_os = "linux")]
    render_ctx: RenderContext,
    #[cfg(target_os = "linux")]
    renderer: Option<vello::Renderer>,
    entries: HashMap<WindowId, SurfaceEntry>,
    #[cfg(target_os = "linux")]
    surface_format: Option<wgpu::TextureFormat>,
}

struct SurfaceEntry {
    handle: SurfaceHandle,
    #[cfg(target_os = "linux")]
    surface: Option<wgpu::Surface<'static>>,
    #[cfg(target_os = "linux")]
    config: Option<wgpu::SurfaceConfiguration>,
    #[cfg(target_os = "linux")]
    dev_id: Option<usize>,
}

unsafe impl Send for WgpuSurfaceProvider {}
unsafe impl Sync for WgpuSurfaceProvider {}

impl WgpuSurfaceProvider {
    pub fn new() -> Self {
        Self {
            #[cfg(target_os = "linux")]
            render_ctx: RenderContext::new(),
            #[cfg(target_os = "linux")]
            renderer: None,
            entries: HashMap::new(),
            #[cfg(target_os = "linux")]
            surface_format: None,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn init_surface_from_gdk(
        &mut self,
        window_id: WindowId,
        gdk_window: &gdk::Window,
        width: u32,
        height: u32,
    ) -> Result<(), SurfaceError> {
        use raw_window_handle::{
            RawDisplayHandle, RawWindowHandle, XlibDisplayHandle, XlibWindowHandle,
        };

        let entry = self
            .entries
            .get_mut(&window_id)
            .ok_or(SurfaceError::InvalidHandle)?;

        let xid = unsafe { get_x11_window_id(gdk_window) };
        let display = unsafe { get_x11_display() };

        let raw_window = RawWindowHandle::Xlib(XlibWindowHandle::new(xid));
        let raw_display = RawDisplayHandle::Xlib(XlibDisplayHandle::new(
            Some(std::ptr::NonNull::new(display).expect("display should not be null")),
            0,
        ));

        let surface = unsafe {
            self.render_ctx
                .instance
                .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_window_handle: &raw_window,
                    raw_display_handle: &raw_display,
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

#[cfg(target_os = "linux")]
unsafe fn get_x11_window_id(gdk_window: &gdk::Window) -> std::num::NonZero<u64> {
    use glib::translate::ToGlibPtr;
    let mut xid: u64 = 0;
    unsafe {
        let ptr: *mut u64 = &mut xid;
        gdk_window.to_glib_none().0;
        extern "C" {
            fn gdk_x11_window_get_xid(window: *mut std::ffi::c_void) -> u64;
        }
        let raw = gdk_window.to_glib_none();
        xid = gdk_x11_window_get_xid(raw.0 as *mut std::ffi::c_void);
    }
    std::num::NonZeroU64::new(xid).expect("xid should not be 0")
}

#[cfg(target_os = "linux")]
unsafe fn get_x11_display() -> *mut std::ffi::c_void {
    use glib::translate::ToGlibPtr;
    extern "C" {
        fn gdk_x11_display_get_xdisplay(display: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    }
    let display = gdk::Display::default().expect("no display");
    let raw_display = display.to_glib_none();
    gdk_x11_display_get_xdisplay(raw_display.0 as *mut std::ffi::c_void)
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
                #[cfg(target_os = "linux")]
                surface: None,
                #[cfg(target_os = "linux")]
                config: None,
                #[cfg(target_os = "linux")]
                dev_id: None,
            },
        );
        Ok(handle)
    }

    fn resize_surface(&mut self, handle: &SurfaceHandle, size: Size) {
        if let Some(entry) = self.entries.get_mut(&handle.id) {
            entry.handle.width = size.width;
            entry.handle.height = size.height;
            #[cfg(target_os = "linux")]
            {
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
    }

    fn present(
        &mut self,
        handle: &SurfaceHandle,
        scene: &vello::Scene,
    ) -> Result<(), SurfaceError> {
        #[cfg(target_os = "linux")]
        {
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

        #[cfg(not(target_os = "linux"))]
        {
            let _ = (handle, scene);
            Err(SurfaceError::UnsupportedPlatform)
        }
    }

    fn destroy_surface(&mut self, handle: SurfaceHandle) {
        self.entries.remove(&handle.id);
    }
}

#[cfg(target_os = "linux")]
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
