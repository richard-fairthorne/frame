use frame_core::traits::widget::Widget;
use frame_core::{Constraints, Size};
use frame_ui::ProgressIndicator;

#[test]
fn progress_determinate() {
    let p = ProgressIndicator::determinate(0.75);
    assert!(!p.is_indeterminate());
    assert!((p.fraction() - 0.75).abs() < 0.01);
}

#[test]
fn progress_indeterminate() {
    let p = ProgressIndicator::indeterminate();
    assert!(p.is_indeterminate());
}

#[test]
fn progress_clamp() {
    let p = ProgressIndicator::determinate(1.5);
    assert!((p.fraction() - 1.0).abs() < 0.01);
}

#[test]
fn progress_set() {
    let mut p = ProgressIndicator::determinate(0.0);
    p.set_progress(0.5);
    assert!(!p.is_indeterminate());
    assert!((p.fraction() - 0.5).abs() < 0.01);
}

#[test]
fn progress_measure() {
    let p = ProgressIndicator::determinate(0.5);
    let size = p.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert!(size.width > 0.0);
}
