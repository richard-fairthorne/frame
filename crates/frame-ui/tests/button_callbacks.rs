use std::sync::{Arc, Mutex};

use frame_core::gesture::GestureEvent;
use frame_core::Point;
use frame_ui::Button;

#[test]
fn button_on_press_callback() {
    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();

    let mut button = Button::new("Click me", move || {
        *called_clone.lock().unwrap() = true;
    });

    button.process_gesture(&GestureEvent::Tap {
        position: Point::new(10.0, 10.0),
    });
    assert!(*called.lock().unwrap());
}

#[test]
fn button_on_double_tap_callback() {
    let double_tapped = Arc::new(Mutex::new(false));
    let dt_clone = double_tapped.clone();

    let mut button = Button::new("Double tap", || {}).on_double_tap(move || {
        *dt_clone.lock().unwrap() = true;
    });

    button.process_gesture(&GestureEvent::DoubleTap {
        position: Point::new(10.0, 10.0),
    });
    assert!(*double_tapped.lock().unwrap());
}

#[test]
fn button_on_long_press_callback() {
    let long_pressed = Arc::new(Mutex::new(false));
    let lp_clone = long_pressed.clone();

    let mut button = Button::new("Long press", || {}).on_long_press(move || {
        *lp_clone.lock().unwrap() = true;
    });

    button.process_gesture(&GestureEvent::LongPress {
        position: Point::new(10.0, 10.0),
    });
    assert!(*long_pressed.lock().unwrap());
}

#[test]
fn button_no_callback_no_panic() {
    let mut button = Button::new("No callback", || {});
    button.process_gesture(&GestureEvent::Tap {
        position: Point::ZERO,
    });
}
