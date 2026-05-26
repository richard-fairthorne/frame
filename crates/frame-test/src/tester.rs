use frame_core::traits::widget::Widget;
use frame_core::{Size, Constraints};
use crate::harness::TestHarness;
use crate::assert::LayoutAssert;

pub struct WidgetTester {
    harness: TestHarness,
}

impl WidgetTester {
    pub fn new() -> Self {
        Self {
            harness: TestHarness::new(),
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.harness = self.harness.size(width, height);
        self
    }

    pub fn with_constraints(mut self, constraints: Constraints) -> Self {
        self.harness = self.harness.constraints(constraints);
        self
    }

    pub fn test<W: Widget>(self, widget: &W) -> LayoutAssert {
        let size = self.harness.measure(widget);
        LayoutAssert::new(size, self.harness.get_constraints())
    }

    pub fn test_render<W: Widget>(self, widget: &mut W) -> RenderAssert {
        let result = self.harness.measure_and_render(widget);
        RenderAssert {
            size: result.size,
        }
    }
}

impl Default for WidgetTester {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RenderAssert {
    pub size: Size,
}

impl RenderAssert {
    pub fn size(&self) -> Size {
        self.size
    }
    pub fn has_size(&self) -> bool {
        self.size.width > 0.0 || self.size.height > 0.0
    }
}
