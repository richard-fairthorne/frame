use frame_assets::{AssetBundle, ResourceBundle};

#[test]
fn asset_bundle_resolve() {
    let bundle = AssetBundle::new("/app/assets")
        .add("logo", "logo.png")
        .add("icon", "icons/app.png");

    assert_eq!(
        bundle.resolve("logo"),
        Some(std::path::PathBuf::from("/app/assets/logo.png"))
    );
    assert_eq!(
        bundle.resolve("icon"),
        Some(std::path::PathBuf::from("/app/assets/icons/app.png"))
    );
    assert_eq!(bundle.resolve("missing"), None);
}

#[test]
fn asset_bundle_names() {
    let bundle = AssetBundle::new("/assets").add("a", "a.png").add("b", "b.png");
    assert_eq!(bundle.asset_names(), vec!["a", "b"]);
}

#[test]
fn resource_bundle() {
    let bundle = ResourceBundle::new("/app")
        .image("logo", "logo.png")
        .font("inter", "inter.ttf");

    assert_eq!(bundle.image_count(), 1);
    assert_eq!(bundle.font_count(), 1);
    assert!(bundle.resolve_image("logo").is_some());
    assert!(bundle.resolve_font("inter").is_some());
}
