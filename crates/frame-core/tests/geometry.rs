use frame_core::{Constraints, Point, Rect, Size};

#[test]
fn size_new() {
    let s = Size::new(100.0, 200.0);
    assert_eq!(s.width, 100.0);
    assert_eq!(s.height, 200.0);
}

#[test]
fn size_zero() {
    let s = Size::ZERO;
    assert_eq!(s.width, 0.0);
    assert_eq!(s.height, 0.0);
}

#[test]
fn point_new() {
    let p = Point::new(10.0, 20.0);
    assert_eq!(p.x, 10.0);
    assert_eq!(p.y, 20.0);
}

#[test]
fn rect_new() {
    let r = Rect::new(Point::new(10.0, 20.0), Size::new(100.0, 200.0));
    assert_eq!(r.origin.x, 10.0);
    assert_eq!(r.origin.y, 20.0);
    assert_eq!(r.size.width, 100.0);
    assert_eq!(r.size.height, 200.0);
}

#[test]
fn rect_from_components() {
    let r = Rect::from_components(10.0, 20.0, 100.0, 200.0);
    assert_eq!(r.origin.x, 10.0);
    assert_eq!(r.origin.y, 20.0);
    assert_eq!(r.size.width, 100.0);
    assert_eq!(r.size.height, 200.0);
}

#[test]
fn constraints_tight() {
    let c = Constraints::tight(Size::new(100.0, 200.0));
    assert_eq!(c.min().width, 100.0);
    assert_eq!(c.max().width, 100.0);
    assert_eq!(c.min().height, 200.0);
    assert_eq!(c.max().height, 200.0);
}

#[test]
fn constraints_loose() {
    let c = Constraints::loose(Size::new(100.0, 200.0));
    assert_eq!(c.min().width, 0.0);
    assert_eq!(c.min().height, 0.0);
    assert_eq!(c.max().width, 100.0);
    assert_eq!(c.max().height, 200.0);
}

#[test]
fn constraints_constrain() {
    let c = Constraints::new(Size::new(50.0, 50.0), Size::new(200.0, 200.0));
    let constrained = c.constrain(Size::new(300.0, 30.0));
    assert_eq!(constrained.width, 200.0);
    assert_eq!(constrained.height, 50.0);
}

#[test]
fn constraints_is_tight() {
    let tight = Constraints::tight(Size::new(100.0, 100.0));
    let loose = Constraints::loose(Size::new(100.0, 100.0));
    assert!(tight.is_tight());
    assert!(!loose.is_tight());
}
