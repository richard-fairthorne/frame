use frame_core::traits::widget::{RenderContext, Widget, WidgetNode, WidgetOutput};
use frame_core::{Constraints, Size};

use crate::style::Style;

pub struct Expanded {
    child: Option<Box<dyn Widget>>,
    flex: f32,
    style: Style,
}

impl Expanded {
    pub fn new() -> Self {
        Self {
            child: None,
            flex: 1.0,
            style: Style::default(),
        }
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    pub fn flex(mut self, flex: f32) -> Self {
        self.flex = flex;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn flex_val(&self) -> f32 {
        self.flex
    }
}

impl Default for Expanded {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Expanded {
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
        constraints.constrain(constraints.max())
    }
}
