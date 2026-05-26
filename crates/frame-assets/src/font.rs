use crate::asset::Asset;
use std::any::Any;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontId(pub u32);

impl FontId {
    pub const DEFAULT: FontId = FontId(0);
    pub fn new(id: u32) -> Self {
        FontId(id)
    }
}

#[derive(Debug, Clone)]
pub struct FontAsset {
    pub id: FontId,
    pub family_name: String,
    pub weight: u16,
    pub italic: bool,
    pub data: Vec<u8>,
}

impl FontAsset {
    pub fn new(family_name: &str, data: Vec<u8>) -> Self {
        Self {
            id: FontId::DEFAULT,
            family_name: family_name.to_string(),
            weight: 400,
            italic: false,
            data,
        }
    }

    pub fn with_weight(mut self, weight: u16) -> Self {
        self.weight = weight;
        self
    }
    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }
    pub fn with_id(mut self, id: FontId) -> Self {
        self.id = id;
        self
    }

    pub fn family(&self) -> &str {
        &self.family_name
    }
    pub fn is_bold(&self) -> bool {
        self.weight >= 700
    }
    pub fn is_italic(&self) -> bool {
        self.italic
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Asset for FontAsset {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn size_bytes(&self) -> usize {
        self.data.len()
    }
}

pub struct FontRegistry {
    next_id: u32,
}

impl FontRegistry {
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    pub fn next_id(&mut self) -> FontId {
        let id = FontId(self.next_id);
        self.next_id += 1;
        id
    }
}

impl Default for FontRegistry {
    fn default() -> Self {
        Self::new()
    }
}
