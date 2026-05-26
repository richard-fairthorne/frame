use crate::tokens::*;
use frame_core::Color;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub is_dark: bool,
    pub colors: ColorPalette,
    pub typography: TypographyScale,
    pub spacing: SpacingScale,
    pub shapes: ShapeScale,
    pub elevation: ElevationScale,
    pub motion: MotionScale,
}

impl Theme {
    pub fn primary(&self) -> Color {
        self.colors.primary.value
    }
    pub fn secondary(&self) -> Color {
        self.colors.secondary.value
    }
    pub fn background(&self) -> Color {
        self.colors.background.value
    }
    pub fn surface(&self) -> Color {
        self.colors.surface.value
    }
    pub fn error(&self) -> Color {
        self.colors.error.value
    }
    pub fn on_primary(&self) -> Color {
        self.colors.on_primary.value
    }
    pub fn on_surface(&self) -> Color {
        self.colors.on_surface.value
    }
    pub fn on_background(&self) -> Color {
        self.colors.on_background.value
    }
    pub fn on_error(&self) -> Color {
        self.colors.on_error.value
    }
}

pub struct ThemeProvider {
    current: Arc<RwLock<Theme>>,
}

impl ThemeProvider {
    pub fn new(theme: Theme) -> Self {
        Self {
            current: Arc::new(RwLock::new(theme)),
        }
    }

    pub fn get(&self) -> Theme {
        self.current.read().unwrap().clone()
    }

    pub fn set(&self, theme: Theme) {
        *self.current.write().unwrap() = theme;
    }

    pub fn is_dark(&self) -> bool {
        self.current.read().unwrap().is_dark
    }

    pub fn toggle(&self) {
        let mut current = self.current.write().unwrap();
        *current = if current.is_dark {
            light_theme()
        } else {
            dark_theme()
        };
    }
}

impl Clone for ThemeProvider {
    fn clone(&self) -> Self {
        Self {
            current: Arc::clone(&self.current),
        }
    }
}

pub fn light_theme() -> Theme {
    Theme {
        name: "light",
        is_dark: false,
        colors: ColorPalette {
            primary: ColorToken::new(98, 0, 238, 255, "primary"),
            primary_variant: ColorToken::new(55, 0, 179, 255, "primary_variant"),
            secondary: ColorToken::new(3, 218, 198, 255, "secondary"),
            secondary_variant: ColorToken::new(1, 151, 138, 255, "secondary_variant"),
            background: ColorToken::new(255, 255, 255, 255, "background"),
            surface: ColorToken::new(255, 255, 255, 255, "surface"),
            error: ColorToken::new(176, 0, 32, 255, "error"),
            on_primary: ColorToken::new(255, 255, 255, 255, "on_primary"),
            on_secondary: ColorToken::new(0, 0, 0, 255, "on_secondary"),
            on_background: ColorToken::new(0, 0, 0, 255, "on_background"),
            on_surface: ColorToken::new(0, 0, 0, 255, "on_surface"),
            on_error: ColorToken::new(255, 255, 255, 255, "on_error"),
        },
        typography: default_typography(),
        spacing: default_spacing(),
        shapes: default_shapes(),
        elevation: default_elevation(),
        motion: default_motion(),
    }
}

pub fn dark_theme() -> Theme {
    Theme {
        name: "dark",
        is_dark: true,
        colors: ColorPalette {
            primary: ColorToken::new(187, 134, 252, 255, "primary"),
            primary_variant: ColorToken::new(124, 77, 255, 255, "primary_variant"),
            secondary: ColorToken::new(3, 218, 198, 255, "secondary"),
            secondary_variant: ColorToken::new(0, 229, 255, 255, "secondary_variant"),
            background: ColorToken::new(18, 18, 18, 255, "background"),
            surface: ColorToken::new(30, 30, 30, 255, "surface"),
            error: ColorToken::new(207, 102, 121, 255, "error"),
            on_primary: ColorToken::new(0, 0, 0, 255, "on_primary"),
            on_secondary: ColorToken::new(0, 0, 0, 255, "on_secondary"),
            on_background: ColorToken::new(255, 255, 255, 255, "on_background"),
            on_surface: ColorToken::new(255, 255, 255, 255, "on_surface"),
            on_error: ColorToken::new(0, 0, 0, 255, "on_error"),
        },
        typography: default_typography(),
        spacing: default_spacing(),
        shapes: default_shapes(),
        elevation: default_elevation(),
        motion: default_motion(),
    }
}

fn default_typography() -> TypographyScale {
    TypographyScale {
        display_large: TypographyToken { font_size: 57.0, line_height: 64.0, font_weight: FontWeight::Regular, letter_spacing: -0.25, name: "display_large" },
        display_medium: TypographyToken { font_size: 45.0, line_height: 52.0, font_weight: FontWeight::Regular, letter_spacing: 0.0, name: "display_medium" },
        display_small: TypographyToken { font_size: 36.0, line_height: 44.0, font_weight: FontWeight::Regular, letter_spacing: 0.0, name: "display_small" },
        headline_large: TypographyToken { font_size: 32.0, line_height: 40.0, font_weight: FontWeight::Regular, letter_spacing: 0.0, name: "headline_large" },
        headline_medium: TypographyToken { font_size: 28.0, line_height: 36.0, font_weight: FontWeight::Regular, letter_spacing: 0.0, name: "headline_medium" },
        headline_small: TypographyToken { font_size: 24.0, line_height: 32.0, font_weight: FontWeight::Regular, letter_spacing: 0.0, name: "headline_small" },
        title_large: TypographyToken { font_size: 22.0, line_height: 28.0, font_weight: FontWeight::Regular, letter_spacing: 0.0, name: "title_large" },
        title_medium: TypographyToken { font_size: 16.0, line_height: 24.0, font_weight: FontWeight::Medium, letter_spacing: 0.15, name: "title_medium" },
        title_small: TypographyToken { font_size: 14.0, line_height: 20.0, font_weight: FontWeight::Medium, letter_spacing: 0.1, name: "title_small" },
        body_large: TypographyToken { font_size: 16.0, line_height: 24.0, font_weight: FontWeight::Regular, letter_spacing: 0.5, name: "body_large" },
        body_medium: TypographyToken { font_size: 14.0, line_height: 20.0, font_weight: FontWeight::Regular, letter_spacing: 0.25, name: "body_medium" },
        body_small: TypographyToken { font_size: 12.0, line_height: 16.0, font_weight: FontWeight::Regular, letter_spacing: 0.4, name: "body_small" },
        label_large: TypographyToken { font_size: 14.0, line_height: 20.0, font_weight: FontWeight::Medium, letter_spacing: 0.1, name: "label_large" },
        label_medium: TypographyToken { font_size: 12.0, line_height: 16.0, font_weight: FontWeight::Medium, letter_spacing: 0.5, name: "label_medium" },
        label_small: TypographyToken { font_size: 11.0, line_height: 16.0, font_weight: FontWeight::Medium, letter_spacing: 0.5, name: "label_small" },
    }
}

fn default_spacing() -> SpacingScale {
    SpacingScale {
        none: SpacingToken { value: 0.0, name: "none" },
        xs: SpacingToken { value: 2.0, name: "xs" },
        sm: SpacingToken { value: 4.0, name: "sm" },
        md: SpacingToken { value: 8.0, name: "md" },
        lg: SpacingToken { value: 16.0, name: "lg" },
        xl: SpacingToken { value: 24.0, name: "xl" },
        xxl: SpacingToken { value: 32.0, name: "xxl" },
    }
}

fn default_shapes() -> ShapeScale {
    ShapeScale {
        none: ShapeToken { corner_radius: 0.0, name: "none" },
        extra_small: ShapeToken { corner_radius: 2.0, name: "extra_small" },
        small: ShapeToken { corner_radius: 4.0, name: "small" },
        medium: ShapeToken { corner_radius: 8.0, name: "medium" },
        large: ShapeToken { corner_radius: 16.0, name: "large" },
        extra_large: ShapeToken { corner_radius: 28.0, name: "extra_large" },
        full: ShapeToken { corner_radius: f32::MAX, name: "full" },
    }
}

fn default_elevation() -> ElevationScale {
    ElevationScale {
        level_0: ElevationToken { level: 0, offset_y: 0.0, blur: 0.0, opacity: 0.0, name: "level_0" },
        level_1: ElevationToken { level: 1, offset_y: 1.0, blur: 3.0, opacity: 0.12, name: "level_1" },
        level_2: ElevationToken { level: 2, offset_y: 2.0, blur: 6.0, opacity: 0.12, name: "level_2" },
        level_3: ElevationToken { level: 3, offset_y: 4.0, blur: 8.0, opacity: 0.12, name: "level_3" },
        level_4: ElevationToken { level: 4, offset_y: 6.0, blur: 10.0, opacity: 0.14, name: "level_4" },
        level_5: ElevationToken { level: 5, offset_y: 8.0, blur: 12.0, opacity: 0.15, name: "level_5" },
    }
}

fn default_motion() -> MotionScale {
    MotionScale {
        instant: DurationToken { value_ms: 0, name: "instant" },
        fast: DurationToken { value_ms: 100, name: "fast" },
        normal: DurationToken { value_ms: 200, name: "normal" },
        slow: DurationToken { value_ms: 350, name: "slow" },
        slower: DurationToken { value_ms: 500, name: "slower" },
    }
}
