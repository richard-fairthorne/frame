use frame_core::traits::widget::{RenderContext, Widget, WidgetOutput};
use frame_core::{Constraints, Rect, Size};

type PaintCallback = Box<dyn FnMut(&mut dyn std::any::Any)>;

pub struct PaintWidget {
    paint_fn: Option<PaintCallback>,
    width: f32,
    height: f32,
}

impl PaintWidget {
    pub fn new(paint_fn: impl FnMut(&mut dyn std::any::Any) + 'static) -> Self {
        Self {
            paint_fn: Some(Box::new(paint_fn)),
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

impl Widget for PaintWidget {
    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        let paint_fn = self.paint_fn.take().expect("paint_fn already consumed");
        WidgetOutput::Paint {
            paint_fn,
            rect: Rect::new(
                frame_core::Point::ZERO,
                Size::new(self.width, self.height),
            ),
        }
    }

    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size::new(self.width, self.height))
    }
}
