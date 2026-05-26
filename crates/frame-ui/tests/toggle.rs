use frame_core::traits::widget::Widget;
use frame_core::{Constraints, Size};
use frame_ui::Toggle;

#[test]
fn toggle_switch() {
    let mut t = Toggle::new();
    assert!(!t.is_on());
    t.toggle();
    assert!(t.is_on());
}

#[test]
fn toggle_set() {
    let mut t = Toggle::new().on(true);
    assert!(t.is_on());
    t.set_on(false);
    assert!(!t.is_on());
}

#[test]
fn toggle_measure() {
    let t = Toggle::new();
    let size = t.measure(Constraints::loose(Size::new(100.0, 100.0)));
    assert!(size.width > 0.0);
}
