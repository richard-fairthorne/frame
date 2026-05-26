use std::any::Any;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::font::FontId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetId(u64);

impl AssetId {
    pub fn raw(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetState {
    NotLoaded,
    Loading,
    Loaded,
    Failed,
}

pub trait Asset: Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn size_bytes(&self) -> usize;
}

pub struct AssetEntry {
    pub id: AssetId,
    pub path: PathBuf,
    pub state: AssetState,
    pub data: Option<Box<dyn Asset>>,
}

pub struct AssetManager {
    entries: HashMap<AssetId, AssetEntry>,
    next_id: u64,
    fonts: HashMap<FontId, Vec<u8>>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            next_id: 1,
            fonts: HashMap::new(),
        }
    }

    pub fn register(&mut self, path: PathBuf) -> AssetId {
        let id = AssetId(self.next_id);
        self.next_id += 1;
        self.entries.insert(
            id,
            AssetEntry {
                id,
                path,
                state: AssetState::NotLoaded,
                data: None,
            },
        );
        id
    }

    pub fn register_bytes(&mut self, path: &str, data: Box<dyn Asset>) -> AssetId {
        let id = AssetId(self.next_id);
        self.next_id += 1;
        self.entries.insert(
            id,
            AssetEntry {
                id,
                path: PathBuf::from(path),
                state: AssetState::Loaded,
                data: Some(data),
            },
        );
        id
    }

    pub fn state(&self, id: AssetId) -> AssetState {
        self.entries
            .get(&id)
            .map(|e| e.state)
            .unwrap_or(AssetState::NotLoaded)
    }

    pub fn get(&self, id: AssetId) -> Option<&dyn Asset> {
        self.entries
            .get(&id)
            .and_then(|e| e.data.as_ref().map(|d| d.as_ref()))
    }

    pub fn set_loaded(&mut self, id: AssetId, data: Box<dyn Asset>) {
        if let Some(entry) = self.entries.get_mut(&id) {
            entry.state = AssetState::Loaded;
            entry.data = Some(data);
        }
    }

    pub fn set_failed(&mut self, id: AssetId) {
        if let Some(entry) = self.entries.get_mut(&id) {
            entry.state = AssetState::Failed;
        }
    }

    pub fn set_loading(&mut self, id: AssetId) {
        if let Some(entry) = self.entries.get_mut(&id) {
            entry.state = AssetState::Loading;
        }
    }

    pub fn register_font(&mut self, id: FontId, data: Vec<u8>) {
        self.fonts.insert(id, data);
    }

    pub fn font_data(&self, id: &FontId) -> Option<&[u8]> {
        self.fonts.get(id).map(|v| v.as_slice())
    }

    pub fn loaded_count(&self) -> usize {
        self.entries
            .values()
            .filter(|e| e.state == AssetState::Loaded)
            .count()
    }

    pub fn total_count(&self) -> usize {
        self.entries.len()
    }
}

impl Default for AssetManager {
    fn default() -> Self {
        Self::new()
    }
}
