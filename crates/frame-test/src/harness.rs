use frame_core::traits::widget::{Widget, RenderContext, WidgetOutput};
use frame_core::{Size, Constraints};
use frame_rendering::pipeline::paint_widget_tree;

pub struct TestHarness {
    constraints: Constraints,
}

impl TestHarness {
    pub fn new() -> Self {
        Self {
            constraints: Constraints::loose(Size::new(800.0, 600.0)),
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.constraints = Constraints::tight(Size::new(width, height));
        self
    }

    pub fn constraints(mut self, constraints: Constraints) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn loose(max_width: f32, max_height: f32) -> Self {
        Self {
            constraints: Constraints::loose(Size::new(max_width, max_height)),
        }
    }

    pub fn tight(width: f32, height: f32) -> Self {
        Self::new().size(width, height)
    }

    pub fn get_constraints(&self) -> Constraints {
        self.constraints
    }

    pub fn measure(&self, widget: &dyn Widget) -> Size {
        widget.measure(self.constraints)
    }

    pub fn measure_and_render(&self, widget: &mut dyn Widget) -> TestResult {
        let size = widget.measure(self.constraints);
        let mut ctx = RenderContext {
            id_counter: frame_core::WidgetId::default(),
        };
        let output = widget.render(&mut ctx);
        TestResult {
            size,
            output,
            constraints: self.constraints,
        }
    }

    pub fn paint(&self, widget: &mut dyn Widget) -> TestPaintResult {
        let result = paint_widget_tree(widget, self.constraints, 1.0);
        TestPaintResult {
            scene_present: true,
            dirty: result.dirty,
            width: self.constraints.max().width,
            height: self.constraints.max().height,
        }
    }
}

impl Default for TestHarness {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TestResult {
    pub size: Size,
    pub output: WidgetOutput,
    pub constraints: Constraints,
}

impl TestResult {
    pub fn size(&self) -> Size {
        self.size
    }
    pub fn width(&self) -> f32 {
        self.size.width
    }
    pub fn height(&self) -> f32 {
        self.size.height
    }
}

pub struct TestPaintResult {
    pub scene_present: bool,
    pub dirty: bool,
    pub width: f32,
    pub height: f32,
}
