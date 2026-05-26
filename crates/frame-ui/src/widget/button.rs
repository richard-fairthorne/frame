use std::cell::RefCell;
use std::rc::Rc;

use frame_core::gesture::GestureEvent;
use frame_core::traits::widget::{RenderContext, Widget, WidgetNode, WidgetOutput};
use frame_core::{Color, Constraints, Rect, Size};

use crate::style::Style;
use crate::widget::Text;

pub struct Button {
    label: String,
    on_click: Option<Box<dyn FnMut() + 'static>>,
    on_long_press: Option<Box<dyn FnMut() + 'static>>,
    on_double_tap: Option<Box<dyn FnMut() + 'static>>,
    bounds: Option<Rect>,
    style: Style,
}

impl Button {
    pub fn new(label: impl Into<String>, on_click: impl FnMut() + 'static) -> Self {
        Self {
            label: label.into(),
            on_click: Some(Box::new(on_click)),
            on_long_press: None,
            on_double_tap: None,
            bounds: None,
            style: Style::default(),
        }
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

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn on_long_press(mut self, callback: impl FnMut() + 'static) -> Self {
        self.on_long_press = Some(Box::new(callback));
        self
    }

    pub fn on_double_tap(mut self, callback: impl FnMut() + 'static) -> Self {
        self.on_double_tap = Some(Box::new(callback));
        self
    }

    pub fn set_bounds(&mut self, rect: Rect) {
        self.bounds = Some(rect);
    }

    pub fn bounds(&self) -> Option<Rect> {
        self.bounds
    }

    pub fn trigger_click(&mut self) {
        if let Some(ref mut cb) = self.on_click {
            cb();
        }
    }

    pub fn process_gesture(&mut self, event: &GestureEvent) {
        match event {
            GestureEvent::Tap { .. } => {
                if let Some(ref mut cb) = self.on_click {
                    cb();
                }
            }
            GestureEvent::DoubleTap { .. } => {
                if let Some(ref mut cb) = self.on_double_tap {
                    cb();
                }
            }
            GestureEvent::LongPress { .. } => {
                if let Some(ref mut cb) = self.on_long_press {
                    cb();
                }
            }
            _ => {}
        }
    }
}

impl Widget for Button {
    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        let bg_color = self.style.background_color.unwrap_or_else(|| {
            Color::from_u8(0, 122, 255, 255)
        });

        let on_click = self.on_click.take().map(|cb| {
            let cell: Rc<RefCell<Box<dyn FnMut() + 'static>>> = Rc::new(RefCell::new(cb));
            Rc::new(move || {
                (cell.borrow_mut())();
            }) as Rc<dyn Fn()>
        });

        WidgetOutput::Container {
            background: Some(bg_color),
            border_radius: self.style.border_radius.max(6.0),
            padding: self.style.padding.max(8.0),
            children: vec![WidgetNode {
                id: frame_core::WidgetId::default(),
                widget: Box::new(Text::new(&self.label).size(14.0).color(Color::WHITE)),
            }],
            on_click,
        }
    }

    fn measure(&self, constraints: Constraints) -> Size {
        let char_count = self.label.len() as f32;
        let text_width = char_count * 14.0 * 0.6;
        let text_height = 14.0 * 1.2;
        let padding = self.style.padding * 2.0;
        constraints.constrain(Size::new(
            self.style.width.unwrap_or(text_width + padding),
            self.style.height.unwrap_or(text_height + padding),
        ))
    }
}
