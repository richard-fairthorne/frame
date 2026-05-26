use frame_assets::{AssetManager, ImageAsset, ImageFormat, FontId};

#[test]
fn asset_manager_register_and_state() {
    let mut mgr = AssetManager::new();
    let id = mgr.register(std::path::PathBuf::from("test.png"));
    assert_eq!(mgr.state(id), frame_assets::AssetState::NotLoaded);
}

#[test]
fn asset_manager_register_loaded() {
    let mut mgr = AssetManager::new();
    let img = ImageAsset::new(100, 100, ImageFormat::Png, vec![0u8; 100]);
    let id = mgr.register_bytes("test.png", Box::new(img));
    assert_eq!(mgr.state(id), frame_assets::AssetState::Loaded);
    assert_eq!(mgr.loaded_count(), 1);
}

#[test]
fn asset_manager_set_failed() {
    let mut mgr = AssetManager::new();
    let id = mgr.register(std::path::PathBuf::from("missing.png"));
    mgr.set_failed(id);
    assert_eq!(mgr.state(id), frame_assets::AssetState::Failed);
}

#[test]
fn asset_manager_font() {
    let mut mgr = AssetManager::new();
    mgr.register_font(FontId::new(1), vec![0u8; 100]);
    assert!(mgr.font_data(&FontId::new(1)).is_some());
    assert!(mgr.font_data(&FontId::new(99)).is_none());
}
