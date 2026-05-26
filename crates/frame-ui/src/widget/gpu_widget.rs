use frame_core::traits::widget::{RenderContext, Widget, WidgetOutput};
use frame_core::{Constraints, Rect, Size};

type GpuCallback = Box<dyn FnMut(&mut dyn std::any::Any)>;

pub struct GpuWidget {
    gpu_fn: Option<GpuCallback>,
    width: f32,
    height: f32,
}

impl GpuWidget {
    pub fn new(gpu_fn: impl FnMut(&mut dyn std::any::Any) + 'static) -> Self {
        Self {
            gpu_fn: Some(Box::new(gpu_fn)),
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

impl Widget for GpuWidget {
    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        let gpu_fn = self.gpu_fn.take().expect("gpu_fn already consumed");
        WidgetOutput::Gpu {
            gpu_fn,
            rect: Rect::new(
                frame_core::Point::ZERO,
                Size::new(self.width, self.height),
            ),
        }
    }

    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size::new(self.width, self.height))
    }
}
