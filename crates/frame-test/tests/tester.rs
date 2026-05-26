use frame_test::WidgetTester;
use frame_ui::{Text, Column, Container};
use frame_core::{Size, Constraints};

#[test]
fn tester_measure_and_assert() {
    let result = WidgetTester::new()
        .with_size(800.0, 600.0)
        .test(&Text::new("Hello").size(16.0));

    result.has_size();
}

#[test]
fn tester_column_layout() {
    let col = Column::new()
        .gap(8.0)
        .child(Text::new("A"))
        .child(Text::new("B"));

    let result = WidgetTester::new()
        .with_constraints(Constraints::loose(Size::new(800.0, 600.0)))
        .test(&col);

    result.has_width().has_height().within_constraints();
}

#[test]
fn tester_tight_container() {
    let container = Container::new().padding(16.0);

    let result = WidgetTester::new()
        .with_size(200.0, 100.0)
        .test(&container);

    result.size_equals(32.0, 32.0);
}

#[test]
fn tester_render() {
    let mut text = Text::new("Test").size(14.0);
    let result = WidgetTester::new()
        .with_size(800.0, 600.0)
        .test_render(&mut text);

    assert!(result.has_size());
}
