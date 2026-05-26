use frame_core::traits::widget::{RenderContext, Widget, WidgetOutput};
use frame_core::{Constraints, Size};

pub struct ProgressIndicator {
    progress: Option<f32>,
    track_height: f32,
    track_width: f32,
    indeterminate: bool,
}

impl ProgressIndicator {
    pub fn determinate(progress: f32) -> Self {
        Self {
            progress: Some(progress.clamp(0.0, 1.0)),
            track_height: 4.0,
            track_width: 200.0,
            indeterminate: false,
        }
    }

    pub fn indeterminate() -> Self {
        Self {
            progress: None,
            track_height: 4.0,
            track_width: 200.0,
            indeterminate: true,
        }
    }

    pub fn track_width(mut self, w: f32) -> Self {
        self.track_width = w;
        self
    }

    pub fn track_height(mut self, h: f32) -> Self {
        self.track_height = h;
        self
    }

    pub fn progress(&self) -> Option<f32> {
        self.progress
    }

    pub fn fraction(&self) -> f32 {
        self.progress.unwrap_or(0.0)
    }

    pub fn is_indeterminate(&self) -> bool {
        self.indeterminate
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = Some(progress.clamp(0.0, 1.0));
        self.indeterminate = false;
    }
}

impl Widget for ProgressIndicator {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size::new(self.track_width, self.track_height))
    }

    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::None
    }
}
