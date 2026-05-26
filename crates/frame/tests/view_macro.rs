use frame::view;
use frame::*;

#[test]
fn view_text_widget() {
    let text = view! {
        Text {
            text: "Hello",
            size: 16.0,
        }
    };
    let size = text.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert!(size.width > 0.0);
    assert!(size.height > 0.0);
}

#[test]
fn view_column_with_children() {
    let col = view! {
        Column {
            gap: 8.0,
            Text {
                text: "Line 1",
            }
            Text {
                text: "Line 2",
            }
        }
    };
    let size = col.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert!(size.height > 0.0);
}

#[test]
fn view_container_with_child() {
    let container = view! {
        Container {
            padding: 16.0,
            background: Color::WHITE,
            Text {
                text: "Inside container",
            }
        }
    };
    let size = container.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert!(size.width > 0.0);
}

#[test]
fn view_nested_layout() {
    let layout = view! {
        Column {
            gap: 12.0,
            Container {
                padding: 8.0,
                background: Color::from_u8(240, 240, 240, 255),
                Text {
                    text: "Header",
                    size: 20.0,
                }
            }
            Row {
                gap: 8.0,
                Text {
                    text: "Left",
                }
                Text {
                    text: "Right",
                }
            }
        }
    };
    let size = layout.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert!(size.width > 0.0 && size.height > 0.0);
}
