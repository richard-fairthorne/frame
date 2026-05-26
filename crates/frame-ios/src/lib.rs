#[cfg(target_os = "ios")]
pub mod app;
#[cfg(target_os = "ios")]
pub mod entrypoint;
#[cfg(target_os = "ios")]
pub mod event_loop;
#[cfg(target_os = "ios")]
pub mod platform;
#[cfg(target_os = "ios")]
pub mod render_callback;
#[cfg(target_os = "ios")]
pub mod wgpu_surface;
#[cfg(target_os = "ios")]
pub mod window;
#[cfg(target_os = "ios")]
pub mod deep_link;
#[cfg(target_os = "ios")]
pub mod native_view;

#[cfg(not(target_os = "ios"))]
pub mod platform;
#[cfg(not(target_os = "ios"))]
pub mod native_view;

#[cfg(target_os = "ios")]
pub use app::IosApp;
#[cfg(target_os = "ios")]
pub use entrypoint::{run_app, AppBuilder};
#[cfg(target_os = "ios")]
pub use event_loop::IosEventLoop;
#[cfg(target_os = "ios")]
pub use platform::{InterfaceOrientation, IosPlatform, IosPlatformContext, SafeAreaInsets};
#[cfg(target_os = "ios")]
pub use render_callback::request_render;
#[cfg(target_os = "ios")]
pub use wgpu_surface::WgpuSurfaceProvider;
#[cfg(target_os = "ios")]
pub use window::IosWindowManager;
#[cfg(target_os = "ios")]
pub use deep_link::{set_deep_link_handler, handle_deep_link};

#[cfg(not(target_os = "ios"))]
pub use platform::{InterfaceOrientation, IosPlatform, IosPlatformContext, SafeAreaInsets};

#[cfg(not(target_os = "ios"))]
pub struct AppBuilder {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub resizable: bool,
}

#[cfg(not(target_os = "ios"))]
impl AppBuilder {
    pub fn new() -> Self {
        Self {
            title: "Frame App".into(),
            width: 390.0,
            height: 844.0,
            resizable: false,
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.into();
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
}

#[cfg(not(target_os = "ios"))]
impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(target_os = "ios"))]
pub fn run_app<F>(_builder: AppBuilder, _root_factory: F)
where
    F: Fn() -> Box<dyn frame_core::traits::widget::Widget> + 'static,
{
}
