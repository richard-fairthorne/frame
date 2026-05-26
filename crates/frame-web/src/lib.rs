pub mod platform;
pub mod window;
pub mod entrypoint;
pub mod wgpu_surface;
pub mod deep_link;
pub mod native_view;

pub use platform::{WebPlatform, WebPlatformContext};
pub use window::WebWindowManager;
pub use entrypoint::{AppBuilder, run_app};
pub use wgpu_surface::WebSurfaceProvider;
pub use deep_link::{set_deep_link_handler, handle_deep_link};
