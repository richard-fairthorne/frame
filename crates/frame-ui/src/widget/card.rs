use frame_core::traits::widget::{RenderContext, Widget, WidgetNode, WidgetOutput};
use frame_core::{Color, Constraints, Size};

use crate::style::Style;

pub struct Card {
    child: Option<Box<dyn Widget>>,
    style: Style,
    elevation: f32,
}

impl Card {
    pub fn new() -> Self {
        Self {
            child: None,
            style: Style {
                background_color: Some(Color::WHITE),
                border_radius: 12.0,
                padding: 16.0,
                ..Style::default()
            },
            elevation: 4.0,
        }
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    pub fn elevation(mut self, e: f32) -> Self {
        self.elevation = e;
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
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Card {
    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        let children = if let Some(child) = self.child.take() {
            vec![WidgetNode {
                id: frame_core::WidgetId::default(),
                widget: child,
            }]
        } else {
            vec![]
        };

        WidgetOutput::Container {
            background: self.style.background_color,
            border_radius: self.style.border_radius,
            padding: self.style.padding,
            children,
            on_click: None,
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
