use std::cell::RefCell;
use std::rc::Rc;

use frame_core::traits::widget::{RenderContext, Widget, WidgetNode, WidgetOutput};
use frame_core::{Color, Constraints, Size};

use crate::style::Style;
use crate::widget::Text;

type SelectCallback = Rc<RefCell<Box<dyn FnMut(usize)>>>;

pub struct Dropdown {
    options: Vec<String>,
    selected: Option<usize>,
    placeholder: Option<String>,
    on_select: Option<SelectCallback>,
    style: Style,
}

impl Dropdown {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            options,
            selected: None,
            placeholder: None,
            on_select: None,
            style: Style::default(),
        }
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    pub fn on_select(mut self, callback: impl FnMut(usize) + 'static) -> Self {
        self.on_select = Some(Rc::new(RefCell::new(Box::new(callback))));
        self
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    fn display_text(&self) -> &str {
        match self.selected {
            Some(idx) if idx < self.options.len() => &self.options[idx],
            _ => self.placeholder.as_deref().unwrap_or("Select..."),
        }
    }

    fn longest_option_width(&self) -> f32 {
        let font_size = 14.0;
        let all_texts: Vec<&str> = self
            .options
            .iter()
            .map(|s| s.as_str())
            .chain(self.placeholder.as_deref())
            .collect();
        all_texts
            .iter()
            .map(|t| t.len() as f32 * font_size * 0.6)
            .fold(0.0_f32, f32::max)
    }
}

impl Widget for Dropdown {
    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        let bg_color = self
            .style
            .background_color
            .unwrap_or_else(|| Color::from_u8(240, 240, 240, 255));

        let text_color = match self.selected {
            Some(_) => Color::BLACK,
            None => Color::from_u8(150, 150, 150, 255),
        };

        let on_click = self.on_select.take().map(|cell| {
            Rc::new(move || {
                let mut cb = cell.borrow_mut();
                cb(0);
            }) as Rc<dyn Fn()>
        });

        WidgetOutput::Container {
            background: Some(bg_color),
            border_radius: self.style.border_radius.max(4.0),
            padding: 12.0,
            children: vec![WidgetNode {
                id: frame_core::WidgetId::default(),
                widget: Box::new(Text::new(self.display_text()).size(14.0).color(text_color)),
            }],
            on_click,
        }
    }

    fn measure(&self, constraints: Constraints) -> Size {
        let text_width = self.longest_option_width();
        let text_height = 14.0 * 1.2;
        let padding = 12.0 * 2.0;
        constraints.constrain(Size::new(
            self.style.width.unwrap_or(text_width + padding),
            self.style.height.unwrap_or(text_height + padding),
        ))
    }
}
