use frame_core::{Color, Constraints, Size};
use frame_rendering::pipeline::paint_widget_tree;
use frame_ui::{Column, Container, Text};

#[test]
fn paint_text_widget() {
    let mut text = Text::new("Hello World").size(16.0).color(Color::BLACK);
    let result = paint_widget_tree(&mut text, Constraints::tight(Size::new(800.0, 600.0)), 1.0);
    assert!(result.dirty);
}

#[test]
fn paint_column_with_children() {
    let mut col = Column::new()
        .gap(8.0)
        .child(Text::new("Line 1"))
        .child(Text::new("Line 2"));
    let result = paint_widget_tree(&mut col, Constraints::tight(Size::new(800.0, 600.0)), 1.0);
    assert!(result.dirty);
}

#[test]
fn paint_styled_container() {
    let mut container = Container::new()
        .padding(16.0)
        .background(Color::WHITE)
        .child(Text::new("Inside"));
    let result = paint_widget_tree(&mut container, Constraints::tight(Size::new(800.0, 600.0)), 1.0);
    assert!(result.dirty);
}
