use frame_core::traits::widget::Widget;
use frame_core::{Constraints, Size};
use frame_ui::{BlendMode, Image, ImageFit, ImageSource};

#[test]
fn image_placeholder_loose() {
    let img = Image::placeholder(100, 200);
    let size = img.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert_eq!(size, Size::new(300.0, 600.0));
}

#[test]
fn image_placeholder_tight() {
    let img = Image::placeholder(100, 200);
    let size = img.measure(Constraints::tight(Size::new(300.0, 600.0)));
    assert_eq!(size, Size::new(300.0, 600.0));
}

#[test]
fn image_from_bytes() {
    let img = Image::from_bytes(vec![0u8; 100], 50, 50);
    assert_eq!(img.intrinsic_size(), Size::new(50.0, 50.0));
}

#[test]
fn image_fit_contain() {
    let img = Image::placeholder(200, 100).fit(ImageFit::Contain);
    let size = img.measure(Constraints::loose(Size::new(400.0, 400.0)));
    assert_eq!(size, Size::new(400.0, 200.0));
}

#[test]
fn image_fit_cover() {
    let img = Image::placeholder(200, 100).fit(ImageFit::Cover);
    let computed = ImageFit::Cover.compute_size(
        Size::new(200.0, 100.0),
        Constraints::loose(Size::new(300.0, 300.0)),
    );
    assert_eq!(computed, Size::new(600.0, 300.0));
    let size = img.measure(Constraints::loose(Size::new(300.0, 300.0)));
    assert_eq!(size, Size::new(300.0, 300.0));
}

#[test]
fn image_fit_fill() {
    let img = Image::placeholder(100, 50).fit(ImageFit::Fill);
    let size = img.measure(Constraints::tight(Size::new(300.0, 200.0)));
    assert_eq!(size, Size::new(300.0, 200.0));
}

#[test]
fn image_fit_none() {
    let img = Image::placeholder(100, 50).fit(ImageFit::None);
    let size = img.measure(Constraints::loose(Size::new(300.0, 200.0)));
    assert_eq!(size, Size::new(100.0, 50.0));
}

#[test]
fn image_fit_width() {
    let img = Image::placeholder(200, 100).fit(ImageFit::FitWidth);
    let size = img.measure(Constraints::loose(Size::new(400.0, 600.0)));
    assert_eq!(size.width, 400.0);
    assert_eq!(size.height, 200.0);
}

#[test]
fn image_fit_height() {
    let img = Image::placeholder(200, 100).fit(ImageFit::FitHeight);
    let size = img.measure(Constraints::loose(Size::new(600.0, 300.0)));
    assert_eq!(size.height, 300.0);
    assert_eq!(size.width, 600.0);
}

#[test]
fn image_opacity_clamp() {
    let img = Image::placeholder(50, 50).opacity(2.0);
    assert_eq!(img.get_opacity(), 1.0);
    let img = Image::placeholder(50, 50).opacity(-1.0);
    assert_eq!(img.get_opacity(), 0.0);
}

#[test]
fn image_asset_source() {
    let img = Image::asset("logo.png");
    match img.source() {
        ImageSource::Asset { name } => assert_eq!(name, "logo.png"),
        _ => panic!("Expected Asset source"),
    }
}

#[test]
fn image_explicit_dimensions() {
    let img = Image::new(ImageSource::None).width(300.0).height(200.0);
    let size = img.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert_eq!(size, Size::new(300.0, 200.0));
}

#[test]
fn image_fit_contain_offset() {
    let offset =
        ImageFit::Contain.compute_offset(Size::new(400.0, 200.0), Size::new(400.0, 400.0));
    assert_eq!(offset.x, 0.0);
    assert_eq!(offset.y, 100.0);
}

#[test]
fn image_blend_mode() {
    let img = Image::placeholder(50, 50).blend_mode(BlendMode::Multiply);
    assert_eq!(img.get_blend_mode(), BlendMode::Multiply);
}
