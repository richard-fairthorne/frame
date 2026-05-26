use frame_core::traits::widget::Widget;
use frame_core::{Constraints, Size};
use frame_ui::Checkbox;
use std::sync::{Arc, Mutex};

#[test]
fn checkbox_toggle() {
    let mut cb = Checkbox::new("Accept");
    assert!(!cb.is_checked());
    cb.toggle();
    assert!(cb.is_checked());
    cb.toggle();
    assert!(!cb.is_checked());
}

#[test]
fn checkbox_set() {
    let mut cb = Checkbox::new("Terms");
    cb.set_checked(true);
    assert!(cb.is_checked());
}

#[test]
fn checkbox_on_change() {
    let states = Arc::new(Mutex::new(Vec::new()));
    let s = states.clone();
    let mut cb = Checkbox::new("Test").on_change(move |v| s.lock().unwrap().push(v));
    cb.toggle();
    cb.toggle();
    assert_eq!(states.lock().unwrap().len(), 2);
}

#[test]
fn checkbox_measure() {
    let cb = Checkbox::new("Label");
    let size = cb.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert!(size.width > 0.0);
}
