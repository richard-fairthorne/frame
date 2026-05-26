use std::cell::RefCell;
use std::rc::Rc;

use frame_core::traits::widget::{RenderContext, Widget, WidgetNode, WidgetOutput};
use frame_core::{Color, Constraints, Size};

use crate::widget::{Container, Sized};

type BoolCallback = Rc<RefCell<Box<dyn FnMut(bool) + Send + Sync>>>;

pub struct Switch {
    value: bool,
    on_change: Option<Box<dyn FnMut(bool) + Send + Sync>>,
    active_color: Color,
    inactive_color: Color,
    thumb_color: Color,
}

impl Switch {
    pub fn new(value: bool) -> Self {
        Self {
            value,
            on_change: None,
            active_color: Color::rgb(0.3, 0.7, 1.0),
            inactive_color: Color::rgb(0.7, 0.7, 0.7),
            thumb_color: Color::WHITE,
        }
    }

    pub fn on_change(mut self, f: impl FnMut(bool) + Send + Sync + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn active_color(mut self, color: Color) -> Self {
        self.active_color = color;
        self
    }

    pub fn inactive_color(mut self, color: Color) -> Self {
        self.inactive_color = color;
        self
    }

    pub fn thumb_color(mut self, color: Color) -> Self {
        self.thumb_color = color;
        self
    }

    pub fn is_on(&self) -> bool {
        self.value
    }

    pub fn toggle(&mut self) {
        self.value = !self.value;
        if let Some(ref mut cb) = self.on_change {
            cb(self.value);
        }
    }

    pub fn set_value(&mut self, value: bool) {
        self.value = value;
        if let Some(ref mut cb) = self.on_change {
            cb(self.value);
        }
    }
}

impl Widget for Switch {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size::new(48.0, 28.0))
    }

    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        let track_color = if self.value {
            self.active_color
        } else {
            self.inactive_color
        };

        let thumb_size = 24.0;
        let thumb = Container::new()
            .background(self.thumb_color)
            .border_radius(thumb_size / 2.0)
            .child(Sized::new().width(thumb_size).height(thumb_size));

        let thumb_node = WidgetNode {
            id: frame_core::WidgetId::default(),
            widget: Box::new(thumb),
        };

        let on_click = self.on_change.take().map(|cb| {
            let cell: BoolCallback = Rc::new(RefCell::new(cb));
            let value = self.value;
            Rc::new(move || {
                (cell.borrow_mut())(!value);
            }) as Rc<dyn Fn()>
        });

        WidgetOutput::Container {
            background: Some(track_color),
            border_radius: 16.0,
            padding: 2.0,
            children: vec![thumb_node],
            on_click,
        }
    }
}
