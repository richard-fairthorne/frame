use frame_core::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorToken {
    pub value: Color,
    pub name: &'static str,
}

impl ColorToken {
    pub fn new(r: u8, g: u8, b: u8, a: u8, name: &'static str) -> Self {
        Self {
            value: Color::from_u8(r, g, b, a),
            name,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    pub primary: ColorToken,
    pub primary_variant: ColorToken,
    pub secondary: ColorToken,
    pub secondary_variant: ColorToken,
    pub background: ColorToken,
    pub surface: ColorToken,
    pub error: ColorToken,
    pub on_primary: ColorToken,
    pub on_secondary: ColorToken,
    pub on_background: ColorToken,
    pub on_surface: ColorToken,
    pub on_error: ColorToken,
}

#[derive(Debug, Clone, Copy)]
pub struct TypographyToken {
    pub font_size: f32,
    pub line_height: f32,
    pub font_weight: FontWeight,
    pub letter_spacing: f32,
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
}

impl FontWeight {
    pub fn to_scale(self) -> f32 {
        match self {
            FontWeight::Thin => 100.0,
            FontWeight::Light => 300.0,
            FontWeight::Regular => 400.0,
            FontWeight::Medium => 500.0,
            FontWeight::SemiBold => 600.0,
            FontWeight::Bold => 700.0,
            FontWeight::ExtraBold => 800.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TypographyScale {
    pub display_large: TypographyToken,
    pub display_medium: TypographyToken,
    pub display_small: TypographyToken,
    pub headline_large: TypographyToken,
    pub headline_medium: TypographyToken,
    pub headline_small: TypographyToken,
    pub title_large: TypographyToken,
    pub title_medium: TypographyToken,
    pub title_small: TypographyToken,
    pub body_large: TypographyToken,
    pub body_medium: TypographyToken,
    pub body_small: TypographyToken,
    pub label_large: TypographyToken,
    pub label_medium: TypographyToken,
    pub label_small: TypographyToken,
}

#[derive(Debug, Clone, Copy)]
pub struct SpacingToken {
    pub value: f32,
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct SpacingScale {
    pub none: SpacingToken,
    pub xs: SpacingToken,
    pub sm: SpacingToken,
    pub md: SpacingToken,
    pub lg: SpacingToken,
    pub xl: SpacingToken,
    pub xxl: SpacingToken,
}

#[derive(Debug, Clone, Copy)]
pub struct ShapeToken {
    pub corner_radius: f32,
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct ShapeScale {
    pub none: ShapeToken,
    pub extra_small: ShapeToken,
    pub small: ShapeToken,
    pub medium: ShapeToken,
    pub large: ShapeToken,
    pub extra_large: ShapeToken,
    pub full: ShapeToken,
}

#[derive(Debug, Clone, Copy)]
pub struct ElevationToken {
    pub level: u8,
    pub offset_y: f32,
    pub blur: f32,
    pub opacity: f32,
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct ElevationScale {
    pub level_0: ElevationToken,
    pub level_1: ElevationToken,
    pub level_2: ElevationToken,
    pub level_3: ElevationToken,
    pub level_4: ElevationToken,
    pub level_5: ElevationToken,
}

#[derive(Debug, Clone, Copy)]
pub struct DurationToken {
    pub value_ms: u64,
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct MotionScale {
    pub instant: DurationToken,
    pub fast: DurationToken,
    pub normal: DurationToken,
    pub slow: DurationToken,
    pub slower: DurationToken,
}
