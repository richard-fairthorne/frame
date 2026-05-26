use frame_core::traits::widget::{RenderContext, Widget, WidgetNode, WidgetOutput};
use frame_core::{Constraints, Size};

use crate::style::Style;

pub struct Sized {
    child: Option<Box<dyn Widget>>,
    width: Option<f32>,
    height: Option<f32>,
    style: Style,
}

impl Sized {
    pub fn new() -> Self {
        Self {
            child: None,
            width: None,
            height: None,
            style: Style::default(),
        }
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Default for Sized {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Sized {
    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        if let Some(child) = self.child.take() {
            WidgetOutput::Children {
                children: vec![WidgetNode {
                    id: frame_core::WidgetId::default(),
                    widget: child,
                }],
            }
        } else {
            WidgetOutput::None
        }
    }

    fn measure(&self, constraints: Constraints) -> Size {
        let w = self
            .width
            .or(self.style.width)
            .unwrap_or_else(|| constraints.max().width);
        let h = self
            .height
            .or(self.style.height)
            .unwrap_or_else(|| constraints.max().height);
        constraints.constrain(Size::new(w, h))
    }
}
