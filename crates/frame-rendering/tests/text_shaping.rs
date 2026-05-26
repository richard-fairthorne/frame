use frame_rendering::text::TextShaper;

#[test]
fn shape_simple_text() {
    let mut shaper = TextShaper::new();
    let layout = shaper.shape("Hello", 16.0, None);
    assert!(!layout.runs.is_empty());
    assert!(layout.width > 0.0);
    assert!(layout.height > 0.0);
}

#[test]
fn shape_multiline() {
    let mut shaper = TextShaper::new();
    let layout = shaper.shape("Line 1\nLine 2", 16.0, None);
    assert!(layout.height > 16.0 * 1.2);
    assert!(!layout.runs.is_empty());
}

#[test]
fn shape_word_wrap() {
    let mut shaper = TextShaper::new();
    let layout = shaper.shape("hello world this is a longer text that should wrap", 16.0, Some(100.0));
    assert!(layout.width <= 100.0);
}

#[test]
fn measure_text() {
    let shaper = TextShaper::new();
    let mut shaper_mut = TextShaper::new();
    let layout = shaper_mut.shape("Test", 24.0, None);
    let (w, h) = shaper.measure(&layout);
    assert!(w > 0.0);
    assert!(h > 0.0);
}

#[test]
fn shape_empty_text() {
    let mut shaper = TextShaper::new();
    let layout = shaper.shape("", 16.0, None);
    assert_eq!(layout.width, 0.0);
}

#[test]
fn shape_with_zero_font_size_uses_default() {
    let mut shaper = TextShaper::new();
    let layout = shaper.shape("Test", 0.0, None);
    assert!(!layout.runs.is_empty());
    for run in &layout.runs {
        assert_eq!(run.font_size, 16.0);
    }
}

#[test]
fn shape_produces_glyphs_with_ids() {
    let mut shaper = TextShaper::new();
    let layout = shaper.shape("ABCD", 16.0, None);
    let total_glyphs: usize = layout.runs.iter().map(|r| r.glyphs.len()).sum();
    assert!(total_glyphs > 0);
    for run in &layout.runs {
        for glyph in &run.glyphs {
            assert!(glyph.id > 0);
        }
    }
}

#[test]
fn shape_larger_font_produces_wider_text() {
    let mut shaper = TextShaper::new();
    let small = shaper.shape("Hello", 12.0, None);
    let large = shaper.shape("Hello", 24.0, None);
    assert!(large.width > small.width);
}
