pub mod click;
pub mod framecoord;
pub mod pipeline;
pub mod render_thread;
pub mod renderer;
pub mod scene;
pub mod surface;
pub mod text;

pub use click::{clear_click_targets, register_click_target, dispatch_click};
pub use pipeline::{PaintResult, paint_widget_tree};
pub use render_thread::{RenderCommand, RenderThreadHandle, SceneReceiver, spawn_render_thread};
pub use renderer::{VelloRenderer, VelloSurface};
pub use scene::SceneBuilder;
pub use text::{TextShaper, TextLayout, draw_text_layout};
pub use surface::{FrameCapture, HeadlessSurface, SurfaceError, SurfaceHandle, SurfaceProvider};
