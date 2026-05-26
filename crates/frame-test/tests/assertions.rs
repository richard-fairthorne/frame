use frame_test::LayoutAssert;
use frame_core::{Size, Constraints};

#[test]
fn assert_width_height() {
    let la = LayoutAssert::new(
        Size::new(100.0, 200.0),
        Constraints::loose(Size::new(800.0, 600.0)),
    );
    la.width_equals(100.0).height_equals(200.0);
}

#[test]
fn assert_size_helpers() {
    let la = LayoutAssert::new(
        Size::new(50.0, 75.0),
        Constraints::loose(Size::new(800.0, 600.0)),
    );
    la.has_size().width_at_least(10.0).height_at_least(10.0);
}

#[test]
fn assert_within_constraints() {
    let la = LayoutAssert::new(
        Size::new(400.0, 300.0),
        Constraints::tight(Size::new(800.0, 600.0)),
    );
    la.within_constraints();
}

#[test]
fn assert_less_than() {
    let la = LayoutAssert::new(
        Size::new(100.0, 200.0),
        Constraints::loose(Size::new(800.0, 600.0)),
    );
    la.width_less_than(200.0).height_less_than(300.0);
}

#[test]
fn assert_get_size() {
    let la = LayoutAssert::new(
        Size::new(100.0, 200.0),
        Constraints::loose(Size::new(800.0, 600.0)),
    );
    let size = la.get();
    assert_eq!(size, Size::new(100.0, 200.0));
}
