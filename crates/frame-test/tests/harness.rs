use frame_test::TestHarness;
use frame_ui::{Text, Column};

#[test]
fn harness_measure_text() {
    let harness = TestHarness::tight(800.0, 600.0);
    let text = Text::new("Hello").size(16.0);
    let size = harness.measure(&text);
    assert!(size.width > 0.0);
    assert!(size.height > 0.0);
}

#[test]
fn harness_measure_column() {
    let harness = TestHarness::loose(800.0, 600.0);
    let col = Column::new()
        .gap(8.0)
        .child(Text::new("Line 1"))
        .child(Text::new("Line 2"));
    let size = harness.measure(&col);
    assert!(size.height > 0.0);
}

#[test]
fn harness_tight_constraints() {
    let harness = TestHarness::tight(400.0, 300.0);
    let col = Column::new();
    let size = harness.measure(&col);
    assert_eq!(size.width, 400.0);
    assert_eq!(size.height, 300.0);
}
