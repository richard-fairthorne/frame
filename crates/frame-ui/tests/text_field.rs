use frame_core::traits::widget::Widget;
use frame_core::{Constraints, Size};
use frame_ui::{InputState, TextField};
use std::sync::{Arc, Mutex};

#[test]
fn text_field_create() {
    let tf = TextField::new("Enter text");
    assert_eq!(tf.placeholder(), "Enter text");
    assert!(tf.is_empty());
}

#[test]
fn text_field_insert_and_backspace() {
    let mut tf = TextField::new("Type here");
    tf.insert_char('H');
    tf.insert_char('i');
    assert_eq!(tf.text(), "Hi");
    tf.backspace();
    assert_eq!(tf.text(), "H");
}

#[test]
fn text_field_cursor_movement() {
    let mut tf = TextField::new("").value("Hello");
    assert_eq!(tf.cursor_position(), 5);
    tf.move_cursor_left();
    tf.move_cursor_left();
    assert_eq!(tf.cursor_position(), 3);
    tf.move_cursor_home();
    assert_eq!(tf.cursor_position(), 0);
    tf.move_cursor_end();
    assert_eq!(tf.cursor_position(), 5);
}

#[test]
fn text_field_max_length() {
    let mut tf = TextField::new("").max_length(3);
    tf.insert_char('A');
    tf.insert_char('B');
    tf.insert_char('C');
    tf.insert_char('D');
    assert_eq!(tf.text(), "ABC");
}

#[test]
fn text_field_on_change() {
    let changes = Arc::new(Mutex::new(Vec::<String>::new()));
    let changes_clone = changes.clone();
    let mut tf = TextField::new("").on_change(move |s| {
        changes_clone.lock().unwrap().push(s.to_string());
    });
    tf.insert_char('a');
    tf.insert_char('b');
    assert_eq!(changes.lock().unwrap().len(), 2);
}

#[test]
fn text_field_focus_blur() {
    let mut tf = TextField::new("");
    assert_eq!(tf.state(), InputState::Idle);
    tf.focus();
    assert_eq!(tf.state(), InputState::Focused);
    tf.blur();
    assert_eq!(tf.state(), InputState::Idle);
}

#[test]
fn text_field_measure() {
    let tf = TextField::new("Test");
    let size = tf.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert!(size.width > 0.0);
    assert!(size.height > 0.0);
}
