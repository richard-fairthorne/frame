use frame_core::traits::widget::{LayoutDirection, RenderContext, Widget, WidgetNode, WidgetOutput};
use frame_core::{Constraints, Size};

use crate::style::Style;

pub struct Column {
    children: Vec<WidgetNode>,
    gap: f32,
    style: Style,
}

impl Column {
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

impl Default for Column {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Column {
    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::Layout {
            direction: LayoutDirection::Vertical,
            gap: self.gap,
            children: std::mem::take(&mut self.children),
        }
    }

    fn measure(&self, constraints: Constraints) -> Size {
        let mut total_height: f32 = 0.0;
        let mut max_width: f32 = 0.0;
        for child in &self.children {
            let size = child.widget.measure(constraints);
            total_height += size.height;
            max_width = max_width.max(size.width);
        }
        total_height += self.gap * (self.children.len().saturating_sub(1) as f32);
        constraints.constrain(Size::new(
            self.style.width.unwrap_or(max_width),
            self.style.height.unwrap_or(total_height),
        ))
    }
}
