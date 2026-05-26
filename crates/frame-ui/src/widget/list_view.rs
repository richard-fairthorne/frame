use frame_core::traits::widget::{RenderContext, Widget, WidgetNode, WidgetOutput};
use frame_core::{Constraints, Size};

use crate::style::Style;

pub struct ListView {
    items: Vec<WidgetNode>,
    item_count: usize,
    item_height: f32,
    separator: f32,
    style: Style,
}

impl ListView {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            item_count: 0,
            item_height: 48.0,
            separator: 0.0,
            style: Style::default(),
        }
    }

    pub fn item(self, _builder: impl Fn(usize) -> Box<dyn Widget> + 'static) -> Self {
        self
    }

    pub fn items(mut self, builders: Vec<Box<dyn Widget>>) -> Self {
        self.item_count = builders.len();
        self.items = builders
            .into_iter()
            .map(|w| WidgetNode {
                id: frame_core::WidgetId::default(),
                widget: w,
            })
            .collect();
        self
    }

    pub fn item_count(mut self, n: usize) -> Self {
        self.item_count = n;
        self
    }

    pub fn item_height(mut self, h: f32) -> Self {
        self.item_height = h;
        self
    }

    pub fn separator(mut self, s: f32) -> Self {
        self.separator = s;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn item_count_val(&self) -> usize {
        self.item_count
    }

    pub fn item_height_val(&self) -> f32 {
        self.item_height
    }

    pub fn separator_val(&self) -> f32 {
        self.separator
    }
}

impl Default for ListView {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for ListView {
    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::Children {
            children: std::mem::take(&mut self.items),
        }
    }

    fn measure(&self, constraints: Constraints) -> Size {
        let count = self.item_count.max(self.items.len());
        let total_height = if count > 0 {
            (count as f32) * self.item_height
                + (count.saturating_sub(1) as f32) * self.separator
        } else {
            0.0
        };
        let max = constraints.max();
        constraints.constrain(Size::new(
            self.style.width.unwrap_or(max.width),
            self.style.height.unwrap_or(total_height),
        ))
    }
}
