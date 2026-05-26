use frame_core::{Color, Rect};
use vello::Scene;

pub struct SceneBuilder {
    scene: Scene,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
        }
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn into_scene(self) -> Scene {
        self.scene
    }

    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        use vello::peniko::{Brush, Color as VelloColor, Fill};

        let vello_color = VelloColor::from_rgba8(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        );
        self.scene.fill(
            Fill::NonZero,
            vello::kurbo::Affine::IDENTITY,
            &Brush::Solid(vello_color),
            None,
            &vello::kurbo::Rect::new(
                rect.x() as f64,
                rect.y() as f64,
                (rect.x() + rect.width()) as f64,
                (rect.y() + rect.height()) as f64,
            ),
        );
    }

    pub fn paint_container(&mut self, rect: Rect, background: Option<Color>, _border_radius: f32, _padding: f32) {
        if let Some(color) = background {
            self.fill_rect(rect, color);
        }
    }

    pub fn clear(&mut self) {
        self.scene = Scene::new();
    }
}

impl Default for SceneBuilder {
    fn default() -> Self {
        Self::new()
    }
}
