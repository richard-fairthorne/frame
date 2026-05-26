use frame_core::traits::renderer::Renderer;
use frame_core::Color;
use frame_rendering::{SceneBuilder, VelloRenderer};

#[test]
fn vello_renderer_creates() {
    let renderer = VelloRenderer::new();
    let _ = renderer.context();
}

#[test]
fn vello_renderer_implements_trait() {
    let mut renderer = VelloRenderer::new();
    let handle = frame_core::traits::renderer::WindowHandle {
        id: frame_core::WindowId::default(),
    };
    let mut surface = renderer.create_surface(&handle);
    renderer.render_frame(&mut surface);
}

#[test]
fn scene_builder_fill_rect() {
    let mut builder = SceneBuilder::new();
    builder.fill_rect(
        frame_core::Rect::from_components(0.0, 0.0, 100.0, 100.0),
        Color::RED,
    );
    let _scene = builder.into_scene();
}

#[test]
fn scene_builder_clear() {
    let mut builder = SceneBuilder::new();
    builder.fill_rect(
        frame_core::Rect::from_components(0.0, 0.0, 100.0, 100.0),
        Color::RED,
    );
    builder.clear();
    let _scene = builder.into_scene();
}
