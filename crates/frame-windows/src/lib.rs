#[cfg(target_os = "windows")]
pub mod platform;
#[cfg(target_os = "windows")]
pub mod window;
#[cfg(target_os = "windows")]
pub mod entrypoint;
#[cfg(target_os = "windows")]
pub mod wgpu_surface;
#[cfg(target_os = "windows")]
pub mod deep_link;
#[cfg(target_os = "windows")]
pub mod native_view;

#[cfg(target_os = "windows")]
pub use platform::{WindowsPlatform, WindowsPlatformContext};
#[cfg(target_os = "windows")]
pub use window::WindowsWindowManager;
#[cfg(target_os = "windows")]
pub use entrypoint::{AppBuilder, run_app};
#[cfg(target_os = "windows")]
pub use wgpu_surface::WgpuSurfaceProvider;
#[cfg(target_os = "windows")]
pub use deep_link::{set_deep_link_handler, handle_deep_link};
