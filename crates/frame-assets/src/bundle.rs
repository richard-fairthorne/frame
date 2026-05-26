use crate::asset::AssetManager;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AssetBundle {
    base_path: PathBuf,
    assets: Vec<(String, String)>,
}

impl AssetBundle {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
            assets: Vec::new(),
        }
    }

    pub fn add(mut self, name: &str, path: &str) -> Self {
        self.assets.push((name.to_string(), path.to_string()));
        self
    }

    pub fn resolve(&self, name: &str) -> Option<PathBuf> {
        self.assets
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, p)| self.base_path.join(p))
    }

    pub fn load_all(&self, manager: &mut AssetManager) {
        for (_, path) in &self.assets {
            let full_path = self.base_path.join(path);
            let _ = manager.register(full_path);
        }
    }

    pub fn asset_names(&self) -> Vec<&str> {
        self.assets.iter().map(|(n, _)| n.as_str()).collect()
    }

    pub fn asset_count(&self) -> usize {
        self.assets.len()
    }
}

#[derive(Debug, Clone)]
pub struct ResourceBundle {
    images: AssetBundle,
    fonts: AssetBundle,
}

impl ResourceBundle {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        let base = base_path.into();
        Self {
            images: AssetBundle::new(base.join("images")),
            fonts: AssetBundle::new(base.join("fonts")),
        }
    }

    pub fn image(mut self, name: &str, path: &str) -> Self {
        self.images = self.images.add(name, path);
        self
    }

    pub fn font(mut self, name: &str, path: &str) -> Self {
        self.fonts = self.fonts.add(name, path);
        self
    }

    pub fn resolve_image(&self, name: &str) -> Option<PathBuf> {
        self.images.resolve(name)
    }

    pub fn resolve_font(&self, name: &str) -> Option<PathBuf> {
        self.fonts.resolve(name)
    }

    pub fn load_all(&self, manager: &mut AssetManager) {
        self.images.load_all(manager);
        self.fonts.load_all(manager);
    }

    pub fn image_count(&self) -> usize {
        self.images.asset_count()
    }
    pub fn font_count(&self) -> usize {
        self.fonts.asset_count()
    }
}
