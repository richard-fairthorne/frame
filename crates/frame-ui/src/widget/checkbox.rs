use frame_core::traits::widget::{RenderContext, Widget, WidgetOutput};
use frame_core::{Constraints, Size};

pub struct Checkbox {
    checked: bool,
    label: String,
    on_change: Option<Box<dyn FnMut(bool) + Send + Sync>>,
    size: f32,
}

impl Checkbox {
    pub fn new(label: &str) -> Self {
        Self {
            checked: false,
            label: label.to_string(),
            on_change: None,
            size: 20.0,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn on_change(mut self, f: impl FnMut(bool) + Send + Sync + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn toggle(&mut self) {
        self.checked = !self.checked;
        if let Some(ref mut cb) = self.on_change {
            cb(self.checked);
        }
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
        if let Some(ref mut cb) = self.on_change {
            cb(self.checked);
        }
    }
}

impl Widget for Checkbox {
    fn measure(&self, constraints: Constraints) -> Size {
        let char_width = 14.0 * 0.6;
        let label_width = self.label.len() as f32 * char_width + 8.0;
        let width = self.size + label_width;
        constraints.constrain(Size::new(width, self.size))
    }

    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::None
    }
}
