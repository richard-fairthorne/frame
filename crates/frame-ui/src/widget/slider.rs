use frame_core::traits::widget::{RenderContext, Widget, WidgetOutput};
use frame_core::{Constraints, Size};
use std::sync::{Arc, Mutex};

type SliderCallback = Arc<Mutex<Box<dyn FnMut(f32) + Send + Sync>>>;

pub struct Slider {
    value: f32,
    min: f32,
    max: f32,
    step: Option<f32>,
    on_change: Option<SliderCallback>,
    style: SliderStyle,
}

#[derive(Debug, Clone)]
pub struct SliderStyle {
    pub track_height: f32,
    pub thumb_size: f32,
    pub track_width: f32,
}

impl Default for SliderStyle {
    fn default() -> Self {
        Self {
            track_height: 4.0,
            thumb_size: 20.0,
            track_width: 200.0,
        }
    }
}

impl Slider {
    pub fn new(min: f32, max: f32, value: f32) -> Self {
        Self {
            value: min.max(max.min(value)),
            min,
            max,
            step: None,
            on_change: None,
            style: SliderStyle::default(),
        }
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn track_width(mut self, w: f32) -> Self {
        self.style.track_width = w;
        self
    }

    pub fn thumb_size(mut self, s: f32) -> Self {
        self.style.thumb_size = s;
        self
    }

    pub fn on_change(mut self, f: impl FnMut(f32) + Send + Sync + 'static) -> Self {
        self.on_change = Some(Arc::new(Mutex::new(Box::new(f))));
        self
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn fraction(&self) -> f32 {
        if (self.max - self.min).abs() < f32::EPSILON {
            0.0
        } else {
            (self.value - self.min) / (self.max - self.min)
        }
    }

    pub fn min(&self) -> f32 {
        self.min
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn set_value(&mut self, value: f32) {
        let mut v = value.clamp(self.min, self.max);
        if let Some(step) = self.step {
            v = self.min + ((v - self.min) / step).round() * step;
        }
        self.value = v;
        if let Some(ref cb) = self.on_change {
            cb.lock().unwrap()(self.value);
        }
    }

    pub fn set_from_position(&mut self, x: f32, track_width: f32) {
        let fraction = (x / track_width).clamp(0.0, 1.0);
        self.set_value(self.min + fraction * (self.max - self.min));
    }
}

impl Widget for Slider {
    fn measure(&self, constraints: Constraints) -> Size {
        let h = self.style.thumb_size.max(self.style.track_height);
        let w = self.style.track_width;
        constraints.constrain(Size::new(w, h))
    }

    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::None
    }
}
