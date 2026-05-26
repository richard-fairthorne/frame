use frame_core::traits::widget::{RenderContext, Widget, WidgetNode, WidgetOutput};
use frame_core::{Constraints, Point, Size};

use crate::style::Style;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Vertical,
    Horizontal,
    Both,
}

#[derive(Debug, Clone)]
pub struct ScrollMetrics {
    pub offset: Point,
    pub content_size: Size,
    pub viewport_size: Size,
    pub max_offset: Point,
}

impl ScrollMetrics {
    pub fn new(content_size: Size, viewport_size: Size) -> Self {
        let max_x = (content_size.width - viewport_size.width).max(0.0);
        let max_y = (content_size.height - viewport_size.height).max(0.0);
        Self {
            offset: Point::ZERO,
            content_size,
            viewport_size,
            max_offset: Point::new(max_x, max_y),
        }
    }

    pub fn clamp_offset(&self, offset: Point) -> Point {
        Point::new(
            offset.x.clamp(0.0, self.max_offset.x),
            offset.y.clamp(0.0, self.max_offset.y),
        )
    }

    pub fn set_offset(&mut self, offset: Point) {
        self.offset = self.clamp_offset(offset);
    }

    pub fn scroll_by(&mut self, delta: Point) {
        let new_offset = Point::new(
            self.offset.x + delta.x,
            self.offset.y + delta.y,
        );
        self.set_offset(new_offset);
    }

    pub fn can_scroll_vertically(&self) -> bool {
        self.content_size.height > self.viewport_size.height
    }

    pub fn can_scroll_horizontally(&self) -> bool {
        self.content_size.width > self.viewport_size.width
    }

    pub fn scroll_fraction_x(&self) -> f32 {
        if self.max_offset.x > 0.0 {
            self.offset.x / self.max_offset.x
        } else {
            0.0
        }
    }

    pub fn scroll_fraction_y(&self) -> f32 {
        if self.max_offset.y > 0.0 {
            self.offset.y / self.max_offset.y
        } else {
            0.0
        }
    }
}

pub struct ScrollView {
    child: Option<Box<dyn Widget>>,
    direction: ScrollDirection,
    pub scroll_metrics: Option<ScrollMetrics>,
    show_indicator: bool,
    style: Style,
    deceleration_rate: f32,
    velocity: Point,
}

impl ScrollView {
    pub fn new() -> Self {
        Self {
            child: None,
            direction: ScrollDirection::Vertical,
            scroll_metrics: None,
            show_indicator: true,
            style: Style::default(),
            deceleration_rate: 0.95,
            velocity: Point::ZERO,
        }
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    pub fn direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn show_indicator(mut self, show: bool) -> Self {
        self.show_indicator = show;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn scroll_metrics(&self) -> Option<&ScrollMetrics> {
        self.scroll_metrics.as_ref()
    }

    pub fn scroll_offset(&self) -> Point {
        self.scroll_metrics.as_ref().map(|m| m.offset).unwrap_or(Point::ZERO)
    }

    pub fn scroll_to(&mut self, offset: Point) {
        if let Some(ref mut metrics) = self.scroll_metrics {
            metrics.set_offset(offset);
        }
    }

    pub fn scroll_by(&mut self, delta: Point) {
        if let Some(ref mut metrics) = self.scroll_metrics {
            let delta = match self.direction {
                ScrollDirection::Vertical => Point::new(0.0, delta.y),
                ScrollDirection::Horizontal => Point::new(delta.x, 0.0),
                ScrollDirection::Both => delta,
            };
            metrics.scroll_by(delta);
        }
    }

    pub fn apply_pan(&mut self, delta: Point) {
        self.scroll_by(delta);
    }

    pub fn set_velocity(&mut self, velocity: Point) {
        self.velocity = velocity;
    }

    pub fn tick_inertia(&mut self) -> bool {
        if self.velocity.x.abs() < 0.1 && self.velocity.y.abs() < 0.1 {
            self.velocity = Point::ZERO;
            return false;
        }

        let delta = self.velocity;
        self.apply_pan(delta);
        self.velocity = Point::new(
            self.velocity.x * self.deceleration_rate,
            self.velocity.y * self.deceleration_rate,
        );
        true
    }

    pub fn is_at_edge(&self) -> bool {
        if let Some(metrics) = &self.scroll_metrics {
            let at_top = metrics.offset.y <= 0.0;
            let at_bottom = metrics.offset.y >= metrics.max_offset.y;
            let at_left = metrics.offset.x <= 0.0;
            let at_right = metrics.offset.x >= metrics.max_offset.x;
            at_top || at_bottom || at_left || at_right
        } else {
            true
        }
    }
}

impl Default for ScrollView {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for ScrollView {
    fn measure(&self, constraints: Constraints) -> Size {
        let viewport = constraints.max();

        if let Some(ref child) = self.child {
            let child_constraints = match self.direction {
                ScrollDirection::Vertical => Constraints::new(
                    Size::ZERO,
                    Size::new(viewport.width, f32::INFINITY),
                ),
                ScrollDirection::Horizontal => Constraints::new(
                    Size::ZERO,
                    Size::new(f32::INFINITY, viewport.height),
                ),
                ScrollDirection::Both => Constraints::new(
                    Size::ZERO,
                    Size::new(f32::INFINITY, f32::INFINITY),
                ),
            };
            child.measure(child_constraints)
        } else {
            viewport
        }
    }

    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        let _ = self.show_indicator;
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
}
