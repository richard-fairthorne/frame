use frame_core::traits::widget::Widget;
use frame_core::{Color, Constraints, Size};
use frame_ui::{Button, Column, Container, Row, Text};

#[test]
fn text_measure() {
    let text = Text::new("Hello").size(16.0);
    let size = text.measure(Constraints::loose(Size::new(1000.0, 1000.0)));
    assert!(size.width > 0.0);
    assert!(size.height > 0.0);
}

#[test]
fn text_styled() {
    let text = Text::new("Red").color(Color::RED).bold().size(24.0);
    assert_eq!(text.text_style().color, Color::RED);
    assert_eq!(text.text_style().size, 24.0);
}

#[test]
fn column_measure() {
    let col = Column::new()
        .gap(8.0)
        .child(Text::new("Line 1"))
        .child(Text::new("Line 2"));
    let size = col.measure(Constraints::loose(Size::new(1000.0, 1000.0)));
    assert!(size.height > 0.0);
}

#[test]
fn row_measure() {
    let row = Row::new()
        .gap(8.0)
        .child(Text::new("A"))
        .child(Text::new("B"));
    let size = row.measure(Constraints::loose(Size::new(1000.0, 1000.0)));
    assert!(size.width > 0.0);
}

#[test]
fn container_wraps_child() {
    let container = Container::new()
        .padding(16.0)
        .background(Color::WHITE)
        .child(Text::new("Inside"));
    let size = container.measure(Constraints::loose(Size::new(1000.0, 1000.0)));
    assert!(size.width > 0.0);
}

#[test]
fn button_creates() {
    let button = Button::new("Click me", || {});
    let size = button.measure(Constraints::loose(Size::new(1000.0, 1000.0)));
    assert!(size.width > 0.0);
}
