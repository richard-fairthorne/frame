use frame_assets::{FontAsset, FontId, FontRegistry};

#[test]
fn font_create() {
    let font = FontAsset::new("Inter", vec![0u8; 1000])
        .with_weight(700)
        .with_italic(true)
        .with_id(FontId::new(1));
    assert_eq!(font.family(), "Inter");
    assert!(font.is_bold());
    assert!(font.is_italic());
}

#[test]
fn font_registry() {
    let mut reg = FontRegistry::new();
    let id1 = reg.next_id();
    let id2 = reg.next_id();
    assert_ne!(id1, id2);
}

#[test]
fn font_default_id() {
    assert_eq!(FontId::DEFAULT, FontId(0));
}
