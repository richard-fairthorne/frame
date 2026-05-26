use frame_core::traits::widget::{RenderContext, Widget, WidgetNode, WidgetOutput};
use frame_core::{Constraints, Size};

use crate::style::Style;

pub struct Stack {
    children: Vec<WidgetNode>,
    style: Style,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: Style::default(),
        }
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.children.push(WidgetNode {
            id: frame_core::WidgetId::default(),
            widget: Box::new(child),
        });
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn children(&self) -> &[WidgetNode] {
        &self.children
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Stack {
    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::Children {
            children: std::mem::take(&mut self.children),
        }
    }

    fn measure(&self, constraints: Constraints) -> Size {
        let mut max_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;
        for child in &self.children {
            let size = child.widget.measure(constraints);
            max_width = max_width.max(size.width);
            max_height = max_height.max(size.height);
        }
        constraints.constrain(Size::new(
            self.style.width.unwrap_or(max_width),
            self.style.height.unwrap_or(max_height),
        ))
    }
}
