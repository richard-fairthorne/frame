use frame_core::traits::widget::{RenderContext, Widget, WidgetOutput};
use frame_core::{Color, Constraints, Point, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFit {
    None,
    Contain,
    Cover,
    Fill,
    FitWidth,
    FitHeight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
}

impl ImageFit {
    pub fn compute_size(&self, image_size: Size, constraints: Constraints) -> Size {
        match self {
            ImageFit::None => image_size,
            ImageFit::Fill => constraints.constrain(image_size),
            ImageFit::Contain => {
                let aspect = image_size.width / image_size.height.max(f32::EPSILON);
                let max_w = constraints.max().width;
                let max_h = constraints.max().height;
                let w = max_h * aspect;
                let h = max_w / aspect.max(f32::EPSILON);
                if w <= max_w {
                    Size::new(w, max_h)
                } else {
                    Size::new(max_w, h)
                }
            }
            ImageFit::Cover => {
                let aspect = image_size.width / image_size.height.max(f32::EPSILON);
                let max_w = constraints.max().width;
                let max_h = constraints.max().height;
                let w = max_h * aspect;
                let h = max_w / aspect.max(f32::EPSILON);
                if w >= max_w {
                    Size::new(w, max_h)
                } else {
                    Size::new(max_w, h)
                }
            }
            ImageFit::FitWidth => {
                let aspect = image_size.width / image_size.height.max(f32::EPSILON);
                let w = constraints.max().width;
                Size::new(w, w / aspect.max(f32::EPSILON))
            }
            ImageFit::FitHeight => {
                let aspect = image_size.width / image_size.height.max(f32::EPSILON);
                let h = constraints.max().height;
                Size::new(h * aspect, h)
            }
        }
    }

    pub fn compute_offset(&self, image_size: Size, draw_area: Size) -> Point {
        match self {
            ImageFit::None | ImageFit::Fill => Point::ZERO,
            ImageFit::Contain | ImageFit::Cover => Point::new(
                (draw_area.width - image_size.width) / 2.0,
                (draw_area.height - image_size.height) / 2.0,
            ),
            ImageFit::FitWidth => Point::new(0.0, (draw_area.height - image_size.height) / 2.0),
            ImageFit::FitHeight => Point::new((draw_area.width - image_size.width) / 2.0, 0.0),
        }
    }
}

pub struct Image {
    source: ImageSource,
    fit: ImageFit,
    opacity: f32,
    color_filter: Option<Color>,
    blend_mode: BlendMode,
    width: Option<f32>,
    height: Option<f32>,
}

#[derive(Debug, Clone)]
pub enum ImageSource {
    None,
    Asset { name: String },
    Bytes { data: Vec<u8>, width: u32, height: u32 },
    Placeholder { width: u32, height: u32, color: Color },
}

impl Image {
    pub fn new(source: ImageSource) -> Self {
        Self {
            source,
            fit: ImageFit::Contain,
            opacity: 1.0,
            color_filter: None,
            blend_mode: BlendMode::Normal,
            width: None,
            height: None,
        }
    }

    pub fn asset(name: &str) -> Self {
        Self::new(ImageSource::Asset {
            name: name.to_string(),
        })
    }

    pub fn placeholder(width: u32, height: u32) -> Self {
        Self::new(ImageSource::Placeholder {
            width,
            height,
            color: Color::from_u8(200, 200, 200, 255),
        })
    }

    pub fn from_bytes(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self::new(ImageSource::Bytes {
            data,
            width,
            height,
        })
    }

    pub fn fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
    pub fn color_filter(mut self, color: Color) -> Self {
        self.color_filter = Some(color);
        self
    }
    pub fn blend_mode(mut self, mode: BlendMode) -> Self {
        self.blend_mode = mode;
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

    pub fn source(&self) -> &ImageSource {
        &self.source
    }
    pub fn fit_mode(&self) -> ImageFit {
        self.fit
    }
    pub fn get_opacity(&self) -> f32 {
        self.opacity
    }
    pub fn get_color_filter(&self) -> Option<Color> {
        self.color_filter
    }
    pub fn get_blend_mode(&self) -> BlendMode {
        self.blend_mode
    }

    pub fn intrinsic_size(&self) -> Size {
        match &self.source {
            ImageSource::None => Size::ZERO,
            ImageSource::Asset { .. } => Size::ZERO,
            ImageSource::Bytes { width, height, .. } => Size::new(*width as f32, *height as f32),
            ImageSource::Placeholder { width, height, .. } => {
                Size::new(*width as f32, *height as f32)
            }
        }
    }
}

impl Widget for Image {
    fn measure(&self, constraints: Constraints) -> Size {
        let intrinsic = self.intrinsic_size();
        let size = if intrinsic.width > 0.0 && intrinsic.height > 0.0 {
            self.fit.compute_size(intrinsic, constraints)
        } else {
            let w = self.width.unwrap_or(constraints.max().width);
            let h = self.height.unwrap_or(constraints.max().height);
            Size::new(w, h)
        };
        constraints.constrain(size)
    }

    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::None
    }
}
