use frame_core::Color;
use parley::{FontContext, Layout, LayoutContext, PositionedLayoutItem, StyleProperty};
use vello::peniko::{Brush, Color as VelloColor, Fill};
use vello::kurbo::Affine;
use vello::{Glyph, Scene};

type TextBrush = [u8; 4];

pub struct TextShaper {
    font_cx: FontContext,
    layout_cx: LayoutContext,
}

impl TextShaper {
    pub fn new() -> Self {
        Self {
            font_cx: FontContext::new(),
            layout_cx: LayoutContext::new(),
        }
    }

    pub fn shape(&mut self, text: &str, font_size: f32, max_width: Option<f32>) -> TextLayout {
        let font_size = if font_size > 0.0 { font_size } else { 16.0 };

        let mut builder = self
            .layout_cx
            .ranged_builder(&mut self.font_cx, text, 1.0, true);
        builder.push_default(StyleProperty::FontSize(font_size));

        let mut layout: Layout<TextBrush> = builder.build(text);
        layout.break_all_lines(max_width);

        #[cfg(target_os = "android")]
        {
            let family_count = self.font_cx.collection.family_names().count();
            eprintln!("frame-text: shaping '{}' ({}px, max_width={:?}), font families available: {}", text, font_size, max_width, family_count);
            for name in self.font_cx.collection.family_names().take(10) {
                eprintln!("  family: {}", name);
            }
        }

        let width = layout.width();
        let height = layout.height();

        let mut runs = Vec::new();
        for line in layout.lines() {
            for item in line.items() {
                match item {
                    PositionedLayoutItem::GlyphRun(glyph_run) => {
                        let font = glyph_run.run().font().clone();
                        let run_font_size = glyph_run.run().font_size();
                        let normalized_coords = glyph_run
                            .run()
                            .normalized_coords()
                            .to_vec();
                        let glyphs: Vec<PositionedGlyph> = glyph_run
                            .positioned_glyphs()
                            .map(|g| PositionedGlyph {
                                id: g.id,
                                x: g.x,
                                y: g.y,
                            })
                            .collect();
                        #[cfg(target_os = "android")]
                        eprintln!("  run: font_size={}, {} glyphs, font_data_len={}", run_font_size, glyphs.len(), font.data.len());
                        runs.push(GlyphRunData {
                            font,
                            font_size: run_font_size,
                            normalized_coords,
                            glyphs,
                        });
                    }
                    PositionedLayoutItem::InlineBox(_) => {}
                }
            }
        }

        TextLayout {
            width,
            height,
            runs,
        }
    }

    pub fn measure(&self, layout: &TextLayout) -> (f32, f32) {
        (layout.width, layout.height)
    }
}

impl Default for TextShaper {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TextLayout {
    pub width: f32,
    pub height: f32,
    pub runs: Vec<GlyphRunData>,
}

pub struct GlyphRunData {
    pub font: parley::FontData,
    pub font_size: f32,
    pub normalized_coords: Vec<i16>,
    pub glyphs: Vec<PositionedGlyph>,
}

#[derive(Debug, Clone)]
pub struct PositionedGlyph {
    pub id: u32,
    pub x: f32,
    pub y: f32,
}

pub fn draw_text_layout(
    scene: &mut Scene,
    layout: &TextLayout,
    origin_x: f64,
    origin_y: f64,
    color: Color,
) {
    let vello_color = VelloColor::from_rgba8(
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
        (color.a * 255.0) as u8,
    );
    let brush = Brush::Solid(vello_color);

    for run in &layout.runs {
        let vello_font = vello::peniko::Font::new(run.font.data.clone(), run.font.index);

        let transform = Affine::translate((origin_x, origin_y));

        let glyphs: Vec<Glyph> = run
            .glyphs
            .iter()
            .map(|g| Glyph {
                id: g.id,
                x: g.x,
                y: g.y,
            })
            .collect();

        if glyphs.is_empty() {
            continue;
        }

        let mut draw_glyphs = scene.draw_glyphs(&vello_font);
        draw_glyphs = draw_glyphs
            .transform(transform)
            .font_size(run.font_size)
            .brush(&brush)
            .normalized_coords(&run.normalized_coords);

        draw_glyphs.draw(Fill::NonZero, glyphs.into_iter());
    }
}
