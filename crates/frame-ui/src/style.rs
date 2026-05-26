use frame_core::Color;

#[derive(Debug, Clone)]
pub struct Style {
    pub padding: f32,
    pub margin: f32,
    pub background_color: Option<Color>,
    pub border_radius: f32,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            padding: 0.0,
            margin: 0.0,
            background_color: None,
            border_radius: 0.0,
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
        }
    }
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }

    pub fn margin(mut self, m: f32) -> Self {
        self.margin = m;
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn border_radius(mut self, r: f32) -> Self {
        self.border_radius = r;
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }
}
