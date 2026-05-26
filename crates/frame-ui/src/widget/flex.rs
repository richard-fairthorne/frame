use frame_core::traits::widget::{LayoutDirection, RenderContext, Widget, WidgetNode, WidgetOutput};
use frame_core::{Constraints, Size};

use crate::style::Style;

pub enum FlexDirection {
    Horizontal,
    Vertical,
}

pub enum MainAxisAlignment {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

pub enum CrossAxisAlignment {
    Start,
    Center,
    End,
    Stretch,
}

pub struct Flex {
    children: Vec<WidgetNode>,
    direction: FlexDirection,
    gap: f32,
    main_axis_alignment: MainAxisAlignment,
    cross_axis_alignment: CrossAxisAlignment,
    style: Style,
}

impl Flex {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            direction: FlexDirection::Horizontal,
            gap: 0.0,
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Start,
            style: Style::default(),
        }
    }

    pub fn column() -> Self {
        Self {
            direction: FlexDirection::Vertical,
            ..Self::new()
        }
    }

    pub fn direction(mut self, dir: FlexDirection) -> Self {
        self.direction = dir;
        self
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

    pub fn main_axis_alignment(mut self, align: MainAxisAlignment) -> Self {
        self.main_axis_alignment = align;
        self
    }

    pub fn cross_axis_alignment(mut self, align: CrossAxisAlignment) -> Self {
        self.cross_axis_alignment = align;
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

    pub fn direction_val(&self) -> &FlexDirection {
        &self.direction
    }

    pub fn main_axis_alignment_val(&self) -> &MainAxisAlignment {
        &self.main_axis_alignment
    }

    pub fn cross_axis_alignment_val(&self) -> &CrossAxisAlignment {
        &self.cross_axis_alignment
    }

    fn layout_direction(&self) -> LayoutDirection {
        match self.direction {
            FlexDirection::Horizontal => LayoutDirection::Horizontal,
            FlexDirection::Vertical => LayoutDirection::Vertical,
        }
    }
}

impl Default for Flex {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Flex {
    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::Layout {
            direction: self.layout_direction(),
            gap: self.gap,
            children: std::mem::take(&mut self.children),
        }
    }

    fn measure(&self, constraints: Constraints) -> Size {
        let mut total_main: f32 = 0.0;
        let mut max_cross: f32 = 0.0;
        for child in &self.children {
            let size = child.widget.measure(constraints);
            match self.direction {
                FlexDirection::Horizontal => {
                    total_main += size.width;
                    max_cross = max_cross.max(size.height);
                }
                FlexDirection::Vertical => {
                    total_main += size.height;
                    max_cross = max_cross.max(size.width);
                }
            }
        }
        total_main += self.gap * (self.children.len().saturating_sub(1) as f32);
        let (width, height) = match self.direction {
            FlexDirection::Horizontal => (total_main, max_cross),
            FlexDirection::Vertical => (max_cross, total_main),
        };
        constraints.constrain(Size::new(
            self.style.width.unwrap_or(width),
            self.style.height.unwrap_or(height),
        ))
    }
}
