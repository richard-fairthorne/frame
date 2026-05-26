#[cfg(target_os = "macos")]
pub mod app;
#[cfg(target_os = "macos")]
pub mod platform;
#[cfg(target_os = "macos")]
pub mod window;
#[cfg(target_os = "macos")]
pub mod entrypoint;
#[cfg(target_os = "macos")]
pub mod event_loop;
#[cfg(target_os = "macos")]
pub mod wgpu_surface;
#[cfg(target_os = "macos")]
pub mod render_callback;
#[cfg(target_os = "macos")]
pub mod deep_link;
#[cfg(target_os = "macos")]
pub mod native_view;

#[cfg(target_os = "macos")]
pub use app::MacosApp;
#[cfg(target_os = "macos")]
pub use platform::{MacosPlatform, MacosPlatformContext};
#[cfg(target_os = "macos")]
pub use window::MacosWindowManager;
#[cfg(target_os = "macos")]
pub use entrypoint::{AppBuilder, run_app};
#[cfg(target_os = "macos")]
pub use event_loop::{MacosEventLoop, MacosMouseInput, MacosMouseKind, MacosKeyboardInput, MacosKeyboardKind};
#[cfg(target_os = "macos")]
pub use wgpu_surface::WgpuSurfaceProvider;
#[cfg(target_os = "macos")]
pub use render_callback::request_render;
#[cfg(target_os = "macos")]
pub use deep_link::{set_deep_link_handler, handle_deep_link};
