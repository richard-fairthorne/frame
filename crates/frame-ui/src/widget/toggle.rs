use frame_core::traits::widget::{RenderContext, Widget, WidgetOutput};
use frame_core::{Constraints, Size};

pub struct Toggle {
    is_on: bool,
    on_change: Option<Box<dyn FnMut(bool) + Send + Sync>>,
    track_width: f32,
    track_height: f32,
    #[allow(dead_code)]
    thumb_size: f32,
}

impl Toggle {
    pub fn new() -> Self {
        Self {
            is_on: false,
            on_change: None,
            track_width: 48.0,
            track_height: 24.0,
            thumb_size: 20.0,
        }
    }

    pub fn on(mut self, is_on: bool) -> Self {
        self.is_on = is_on;
        self
    }

    pub fn on_change(mut self, f: impl FnMut(bool) + Send + Sync + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn toggle(&mut self) {
        self.is_on = !self.is_on;
        if let Some(ref mut cb) = self.on_change {
            cb(self.is_on);
        }
    }

    pub fn set_on(&mut self, on: bool) {
        self.is_on = on;
        if let Some(ref mut cb) = self.on_change {
            cb(self.is_on);
        }
    }
}

impl Default for Toggle {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Toggle {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size::new(self.track_width, self.track_height))
    }

    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::None
    }
}
