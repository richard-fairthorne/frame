use frame_core::traits::widget::{RenderContext, Widget, WidgetOutput};
use frame_core::{Color, Constraints, Size};

use crate::style::Style;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FontWeight {
    #[default]
    Normal,
    Bold,
}

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub size: f32,
    pub color: Color,
    pub font_family: Option<String>,
    pub weight: FontWeight,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            size: 14.0,
            color: Color::BLACK,
            font_family: None,
            weight: FontWeight::Normal,
        }
    }
}

pub struct Text {
    content: String,
    style: TextStyle,
    widget_style: Style,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: TextStyle::default(),
            widget_style: Style::default(),
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.style.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.style.color = color;
        self
    }

    pub fn bold(mut self) -> Self {
        self.style.weight = FontWeight::Bold;
        self
    }

    pub fn font_family(mut self, family: impl Into<String>) -> Self {
        self.style.font_family = Some(family.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.widget_style = style;
        self
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn text_style(&self) -> &TextStyle {
        &self.style
    }
}

impl Widget for Text {
    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::Text {
            content: self.content.clone(),
            font_size: self.style.size,
            color: self.style.color,
        }
    }

    fn measure(&self, constraints: Constraints) -> Size {
        let char_count = self.content.len() as f32;
        let width = char_count * self.style.size * 0.6;
        let height = self.style.size * 1.2;
        constraints.constrain(Size::new(
            self.widget_style.width.unwrap_or(width),
            self.widget_style.height.unwrap_or(height),
        ))
    }
}
