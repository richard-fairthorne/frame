use frame_core::{Size, Constraints};

pub struct LayoutAssert {
    size: Size,
    constraints: Constraints,
}

impl LayoutAssert {
    pub fn new(size: Size, constraints: Constraints) -> Self {
        Self { size, constraints }
    }

    pub fn has_width(&self) -> &Self {
        assert!(
            self.size.width > 0.0,
            "Expected width > 0, got {}",
            self.size.width
        );
        self
    }

    pub fn has_height(&self) -> &Self {
        assert!(
            self.size.height > 0.0,
            "Expected height > 0, got {}",
            self.size.height
        );
        self
    }

    pub fn has_size(&self) -> &Self {
        self.has_width().has_height()
    }

    pub fn width_equals(&self, expected: f32) -> &Self {
        assert!(
            (self.size.width - expected).abs() < 0.01,
            "Expected width {}, got {}",
            expected,
            self.size.width
        );
        self
    }

    pub fn height_equals(&self, expected: f32) -> &Self {
        assert!(
            (self.size.height - expected).abs() < 0.01,
            "Expected height {}, got {}",
            expected,
            self.size.height
        );
        self
    }

    pub fn size_equals(&self, width: f32, height: f32) -> &Self {
        self.width_equals(width).height_equals(height)
    }

    pub fn width_less_than(&self, max: f32) -> &Self {
        assert!(
            self.size.width < max,
            "Expected width < {}, got {}",
            max,
            self.size.width
        );
        self
    }

    pub fn height_less_than(&self, max: f32) -> &Self {
        assert!(
            self.size.height < max,
            "Expected height < {}, got {}",
            max,
            self.size.height
        );
        self
    }

    pub fn within_constraints(&self) -> &Self {
        assert!(
            self.size.width <= self.constraints.max().width,
            "Width {} exceeds max {}",
            self.size.width,
            self.constraints.max().width
        );
        assert!(
            self.size.height <= self.constraints.max().height,
            "Height {} exceeds max {}",
            self.size.height,
            self.constraints.max().height
        );
        self
    }

    pub fn width_at_least(&self, min: f32) -> &Self {
        assert!(
            self.size.width >= min,
            "Expected width >= {}, got {}",
            min,
            self.size.width
        );
        self
    }

    pub fn height_at_least(&self, min: f32) -> &Self {
        assert!(
            self.size.height >= min,
            "Expected height >= {}, got {}",
            min,
            self.size.height
        );
        self
    }

    pub fn get(&self) -> Size {
        self.size
    }
}
