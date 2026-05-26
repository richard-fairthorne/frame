use std::any::Any;
use std::rc::Rc;

use crate::{Color, Size, Constraints, Rect};
use crate::id::WidgetId;

pub struct RenderContext {
    pub id_counter: WidgetId,
}

pub enum LayoutDirection {
    Vertical,
    Horizontal,
}

pub type PaintFn = Box<dyn FnMut(&mut dyn Any)>;
pub type GpuFn = Box<dyn FnMut(&mut dyn Any)>;

pub enum WidgetOutput {
    None,
    Text {
        content: String,
        font_size: f32,
        color: Color,
    },
    Paint {
        paint_fn: PaintFn,
        rect: Rect,
    },
    Gpu {
        gpu_fn: GpuFn,
        rect: Rect,
    },
    Children {
        children: Vec<WidgetNode>,
    },
    Layout {
        direction: LayoutDirection,
        gap: f32,
        children: Vec<WidgetNode>,
    },
    Container {
        background: Option<Color>,
        border_radius: f32,
        padding: f32,
        children: Vec<WidgetNode>,
        on_click: Option<Rc<dyn Fn()>>,
    },
}

pub struct WidgetNode {
    pub id: WidgetId,
    pub widget: Box<dyn Widget>,
}

pub trait Widget: 'static {
    fn render(&mut self, ctx: &mut RenderContext) -> WidgetOutput;
    fn measure(&self, constraints: Constraints) -> Size;
}
