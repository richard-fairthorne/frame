pub mod platform;
pub mod deep_link;
pub mod native_view_embed;

#[cfg(target_os = "android")]
pub mod entrypoint;
#[cfg(target_os = "android")]
pub mod native_window;
#[cfg(target_os = "android")]
pub mod wgpu_surface;

pub use platform::{AndroidLifecycleState, AndroidPlatform, AndroidPlatformContext, AndroidWindowManager, JniBridge};
pub use deep_link::{set_deep_link_handler, handle_deep_link};

#[cfg(target_os = "android")]
pub use entrypoint::{run_app_with, AppBuilder};
#[cfg(target_os = "android")]
pub use native_window::NativeWindow;
#[cfg(target_os = "android")]
pub use wgpu_surface::AndroidSurfaceProvider;
