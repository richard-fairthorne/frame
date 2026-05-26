use frame_theme::*;

#[test]
fn light_theme_colors() {
    let theme = light_theme();
    assert!(!theme.is_dark);
    assert_eq!(theme.name, "light");
    assert!(theme.colors.primary.value.r > 0.0);
}

#[test]
fn dark_theme_colors() {
    let theme = dark_theme();
    assert!(theme.is_dark);
    assert_eq!(theme.name, "dark");
}

#[test]
fn theme_provider_get_set() {
    let provider = ThemeProvider::new(light_theme());
    assert!(!provider.is_dark());

    provider.set(dark_theme());
    assert!(provider.is_dark());
}

#[test]
fn theme_provider_toggle() {
    let provider = ThemeProvider::new(light_theme());
    provider.toggle();
    assert!(provider.is_dark());
    provider.toggle();
    assert!(!provider.is_dark());
}

#[test]
fn theme_provider_clone_shares_state() {
    let provider = ThemeProvider::new(light_theme());
    let cloned = provider.clone();

    cloned.set(dark_theme());
    assert!(provider.is_dark());
}

#[test]
fn typography_scale_defaults() {
    let theme = light_theme();
    assert_eq!(theme.typography.display_large.font_size, 57.0);
    assert_eq!(theme.typography.body_medium.font_size, 14.0);
    assert_eq!(theme.typography.label_small.font_size, 11.0);
}

#[test]
fn spacing_scale_defaults() {
    let theme = light_theme();
    assert_eq!(theme.spacing.none.value, 0.0);
    assert_eq!(theme.spacing.xs.value, 2.0);
    assert_eq!(theme.spacing.lg.value, 16.0);
}

#[test]
fn shape_scale_defaults() {
    let theme = light_theme();
    assert_eq!(theme.shapes.small.corner_radius, 4.0);
    assert_eq!(theme.shapes.full.corner_radius, f32::MAX);
}

#[test]
fn elevation_scale_defaults() {
    let theme = light_theme();
    assert_eq!(theme.elevation.level_0.blur, 0.0);
    assert!(theme.elevation.level_3.blur > 0.0);
}

#[test]
fn motion_scale_defaults() {
    let theme = light_theme();
    assert_eq!(theme.motion.fast.value_ms, 100);
    assert_eq!(theme.motion.slow.value_ms, 350);
}

#[test]
fn font_weight_scale() {
    assert_eq!(FontWeight::Thin.to_scale(), 100.0);
    assert_eq!(FontWeight::Bold.to_scale(), 700.0);
}

#[test]
fn theme_convenience_accessors() {
    let theme = light_theme();
    let _ = theme.primary();
    let _ = theme.secondary();
    let _ = theme.background();
    let _ = theme.surface();
    let _ = theme.error();
    let _ = theme.on_primary();
    let _ = theme.on_surface();
    let _ = theme.on_background();
}
