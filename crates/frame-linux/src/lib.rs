#[cfg(target_os = "linux")]
pub mod platform;
#[cfg(target_os = "linux")]
pub mod window;
#[cfg(target_os = "linux")]
pub mod entrypoint;
#[cfg(target_os = "linux")]
pub mod wgpu_surface;
#[cfg(target_os = "linux")]
pub mod deep_link;

#[cfg(not(target_os = "linux"))]
pub mod platform;
#[cfg(not(target_os = "linux"))]
pub mod window;
#[cfg(not(target_os = "linux"))]
pub mod entrypoint;

pub mod native_view;

#[cfg(target_os = "linux")]
pub use platform::{LinuxPlatform, LinuxPlatformContext};
#[cfg(target_os = "linux")]
pub use window::LinuxWindowManager;
#[cfg(target_os = "linux")]
pub use entrypoint::{AppBuilder, run_app};
#[cfg(target_os = "linux")]
pub use wgpu_surface::WgpuSurfaceProvider;
#[cfg(target_os = "linux")]
pub use deep_link::{set_deep_link_handler, handle_deep_link};

#[cfg(not(target_os = "linux"))]
pub use platform::LinuxPlatform;
#[cfg(not(target_os = "linux"))]
pub use window::LinuxWindowManager;
#[cfg(not(target_os = "linux"))]
pub use entrypoint::{AppBuilder, run_app};
