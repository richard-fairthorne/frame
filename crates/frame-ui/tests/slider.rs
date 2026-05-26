use frame_core::traits::widget::Widget;
use frame_core::{Constraints, Size};
use frame_ui::Slider;
use std::sync::{Arc, Mutex};

#[test]
fn slider_create() {
    let slider = Slider::new(0.0, 100.0, 50.0);
    assert_eq!(slider.value(), 50.0);
    assert!((slider.fraction() - 0.5).abs() < 0.01);
}

#[test]
fn slider_clamp() {
    let mut slider = Slider::new(0.0, 100.0, 50.0);
    slider.set_value(150.0);
    assert_eq!(slider.value(), 100.0);
    slider.set_value(-50.0);
    assert_eq!(slider.value(), 0.0);
}

#[test]
fn slider_step() {
    let mut slider = Slider::new(0.0, 100.0, 0.0).step(25.0);
    slider.set_value(30.0);
    assert_eq!(slider.value(), 25.0);
    slider.set_value(80.0);
    assert_eq!(slider.value(), 75.0);
}

#[test]
fn slider_position() {
    let mut slider = Slider::new(0.0, 100.0, 0.0);
    slider.set_from_position(100.0, 200.0);
    assert_eq!(slider.value(), 50.0);
}

#[test]
fn slider_on_change() {
    let values = Arc::new(Mutex::new(Vec::new()));
    let v = values.clone();
    let mut slider = Slider::new(0.0, 100.0, 0.0).on_change(move |val| {
        v.lock().unwrap().push(val);
    });
    slider.set_value(50.0);
    assert_eq!(values.lock().unwrap()[0], 50.0);
}

#[test]
fn slider_measure() {
    let slider = Slider::new(0.0, 1.0, 0.5);
    let size = slider.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert!(size.width > 0.0);
}
