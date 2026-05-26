use frame_core::traits::widget::{LayoutDirection, RenderContext, Widget, WidgetNode, WidgetOutput};
use frame_core::{Constraints, Size};

use crate::style::Style;

pub struct Row {
    children: Vec<WidgetNode>,
    gap: f32,
    style: Style,
}

impl Row {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            gap: 0.0,
            style: Style::default(),
        }
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
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

    pub fn gap_val(&self) -> f32 {
        self.gap
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Row {
    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::Layout {
            direction: LayoutDirection::Horizontal,
            gap: self.gap,
            children: std::mem::take(&mut self.children),
        }
    }

    fn measure(&self, constraints: Constraints) -> Size {
        let mut total_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;
        for child in &self.children {
            let size = child.widget.measure(constraints);
            total_width += size.width;
            max_height = max_height.max(size.height);
        }
        total_width += self.gap * (self.children.len().saturating_sub(1) as f32);
        constraints.constrain(Size::new(
            self.style.width.unwrap_or(total_width),
            self.style.height.unwrap_or(max_height),
        ))
    }
}
