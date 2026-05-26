use frame_core::traits::widget::{RenderContext, Widget, WidgetNode, WidgetOutput};
use frame_core::{Color, Constraints, Size};

use crate::style::Style;

pub struct Container {
    child: Option<Box<dyn Widget>>,
    style: Style,
}

impl Container {
    pub fn new() -> Self {
        Self {
            child: None,
            style: Style::default(),
        }
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.style.padding = p;
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.style.background_color = Some(color);
        self
    }

    pub fn border_radius(mut self, r: f32) -> Self {
        self.style.border_radius = r;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn widget_style(&self) -> &Style {
        &self.style
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Container {
    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        let children = if let Some(child) = self.child.take() {
            vec![WidgetNode {
                id: frame_core::WidgetId::default(),
                widget: child,
            }]
        } else {
            vec![]
        };

        if self.style.background_color.is_some() || self.style.border_radius > 0.0 {
            WidgetOutput::Container {
                background: self.style.background_color,
                border_radius: self.style.border_radius,
                padding: self.style.padding,
                children,
                on_click: None,
            }
        } else if children.is_empty() {
            WidgetOutput::None
        } else {
            WidgetOutput::Children { children }
        }
    }

    fn measure(&self, constraints: Constraints) -> Size {
        let padding = self.style.padding * 2.0;
        if let Some(ref child) = self.child {
            let inner = Size::new(
                constraints.max().width - padding,
                constraints.max().height - padding,
            );
            let child_constraints = Constraints::loose(inner);
            let child_size = child.measure(child_constraints);
            Size::new(
                self.style.width.unwrap_or(child_size.width + padding),
                self.style.height.unwrap_or(child_size.height + padding),
            )
        } else {
            Size::new(
                self.style.width.unwrap_or(padding),
                self.style.height.unwrap_or(padding),
            )
        }
    }
}
