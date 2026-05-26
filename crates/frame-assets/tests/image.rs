use frame_assets::{Asset, ImageAsset, ImageFormat, ImageLoader};

#[test]
fn image_dimensions() {
    let img = ImageAsset::new(800, 600, ImageFormat::Png, vec![0u8; 100]);
    assert_eq!(img.dimensions(), (800, 600));
    assert!((img.aspect_ratio() - 1.333).abs() < 0.01);
}

#[test]
fn image_format_detection() {
    let png_header = [
        137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 10,
    ];
    assert_eq!(ImageLoader::detect_format(&png_header), ImageFormat::Png);

    let jpeg_header = [0xFF, 0xD8, 0xFF, 0xE0];
    assert_eq!(ImageLoader::detect_format(&jpeg_header), ImageFormat::Jpeg);
}

#[test]
fn image_format_from_extension() {
    assert_eq!(ImageFormat::from_extension("png"), ImageFormat::Png);
    assert_eq!(ImageFormat::from_extension("jpg"), ImageFormat::Jpeg);
    assert_eq!(ImageFormat::from_extension("webp"), ImageFormat::WebP);
}

#[test]
fn image_load_from_memory() {
    let mut data = vec![137, 80, 78, 71, 13, 10, 26, 10];
    data.extend_from_slice(&[0; 16]);
    data[16..20].copy_from_slice(&10u32.to_be_bytes());
    data[20..24].copy_from_slice(&20u32.to_be_bytes());
    let img = ImageLoader::load_from_memory(&data, ImageFormat::Png).unwrap();
    assert_eq!(img.width, 10);
    assert_eq!(img.height, 20);
}

#[test]
fn image_asset_size() {
    let img = ImageAsset::new(10, 10, ImageFormat::Png, vec![42u8; 100]);
    assert_eq!(img.size_bytes(), 100);
}
