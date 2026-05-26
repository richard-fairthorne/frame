use crate::asset::Asset;
use std::any::Any;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    WebP,
    Bmp,
    Unknown,
}

impl ImageFormat {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "png" => ImageFormat::Png,
            "jpg" | "jpeg" => ImageFormat::Jpeg,
            "webp" => ImageFormat::WebP,
            "bmp" => ImageFormat::Bmp,
            _ => ImageFormat::Unknown,
        }
    }

    pub fn from_mime(mime: &str) -> Self {
        match mime {
            "image/png" => ImageFormat::Png,
            "image/jpeg" => ImageFormat::Jpeg,
            "image/webp" => ImageFormat::WebP,
            "image/bmp" => ImageFormat::Bmp,
            _ => ImageFormat::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImageAsset {
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub data: Vec<u8>,
}

impl ImageAsset {
    pub fn new(width: u32, height: u32, format: ImageFormat, data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            format,
            data,
        }
    }

    pub fn from_raw_pixels(width: u32, height: u32, pixels: Vec<u8>) -> Self {
        Self {
            width,
            height,
            format: ImageFormat::Unknown,
            data: pixels,
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn pixel_count(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn aspect_ratio(&self) -> f32 {
        if self.height == 0 {
            1.0
        } else {
            self.width as f32 / self.height as f32
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Asset for ImageAsset {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn size_bytes(&self) -> usize {
        self.data.len()
    }
}

pub struct ImageLoader;

impl ImageLoader {
    pub fn load_from_memory(data: &[u8], format: ImageFormat) -> Result<ImageAsset, String> {
        if format == ImageFormat::Png
            && data.len() >= 24
            && data[0..8] == [137, 80, 78, 71, 13, 10, 26, 10]
        {
            let width = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
            let height = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
            return Ok(ImageAsset::new(width, height, format, data.to_vec()));
        }

        Ok(ImageAsset::new(1, 1, format, data.to_vec()))
    }

    pub fn detect_format(data: &[u8]) -> ImageFormat {
        if data.len() >= 8 && data[0..8] == [137, 80, 78, 71, 13, 10, 26, 10] {
            return ImageFormat::Png;
        }
        if data.len() >= 2 && data[0] == 0xFF && data[1] == 0xD8 {
            return ImageFormat::Jpeg;
        }
        if data.len() >= 4 && data[0..4] == *b"RIFF" {
            return ImageFormat::WebP;
        }
        ImageFormat::Unknown
    }
}
