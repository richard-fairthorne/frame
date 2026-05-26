pub mod geometry;
pub mod color;
pub mod id;
pub mod reactive;
pub mod plugin;
pub mod traits;
pub mod app;
pub mod render_loop;
pub mod interaction;
pub mod gesture;

pub use geometry::{Rect, Size, Point, Constraints};
pub use color::Color;
pub use id::{WindowId, WidgetId};

pub use reactive::signal::Signal;
pub use reactive::computed::Computed;
pub use reactive::effect::Effect;
pub use reactive::batch::batch;
pub use reactive::runtime::{request_render, set_render_request_fn};

pub use plugin::{Plugin, PluginContext, PluginHost, EventEmitter, EventStream};

pub use traits::renderer::{Renderer, RenderSurface, WindowHandle};
pub use traits::layout::{Layout, LayoutNode, LayoutResult};
pub use traits::window::{WindowHost, WindowConfig};
pub use traits::widget::{Widget, WidgetOutput, WidgetNode, RenderContext};
pub use traits::platform::{
    PlatformContext, PlatformWindowHandle, NativeViewHandle, NativeViewKind, NativeViewRequest,
};

pub use app::{FrameApp, FrameAppBuilder};
pub use render_loop::{RenderLoop, VsyncMode};

pub use interaction::{InteractionRegistry, InteractionEvent, InteractionId};

pub use gesture::{TapRecognizer, LongPressRecognizer, PanRecognizer, GestureEvent, GestureState};
