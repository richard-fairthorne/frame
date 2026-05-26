use frame_core::gesture::{GestureEvent, LongPressRecognizer, PanRecognizer, TapRecognizer};
use frame_core::{Point, Rect, Size};

fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect::new(Point::new(x, y), Size::new(w, h))
}

#[test]
fn tap_recognizer_single_tap() {
    let mut rec = TapRecognizer::new(rect(0.0, 0.0, 100.0, 100.0));

    let events = rec.process(Point::new(50.0, 50.0), true);
    assert!(events.is_empty());

    let events = rec.process(Point::new(50.0, 50.0), false);
    assert_eq!(events.len(), 1);
    assert!(matches!(events[0], GestureEvent::Tap { .. }));
}

#[test]
fn tap_recognizer_double_tap() {
    let mut rec = TapRecognizer::new(rect(0.0, 0.0, 100.0, 100.0));

    rec.process(Point::new(50.0, 50.0), true);
    rec.process(Point::new(50.0, 50.0), false);

    rec.process(Point::new(50.0, 50.0), true);
    let events = rec.process(Point::new(50.0, 50.0), false);

    assert!(events.iter().any(|e| matches!(e, GestureEvent::DoubleTap { .. })));
}

#[test]
fn tap_recognizer_outside_bounds() {
    let mut rec = TapRecognizer::new(rect(0.0, 0.0, 100.0, 100.0));
    let events = rec.process(Point::new(200.0, 200.0), true);
    assert!(events.is_empty());
}

#[test]
fn long_press_recognizer() {
    let mut rec = LongPressRecognizer::new(rect(0.0, 0.0, 100.0, 100.0)).with_threshold(0);

    rec.process(Point::new(50.0, 50.0), true);
    let events = rec.process(Point::new(50.0, 50.0), true);
    assert!(events.iter().any(|e| matches!(e, GestureEvent::LongPress { .. })));
}

#[test]
fn pan_recognizer() {
    let mut rec = PanRecognizer::new(rect(0.0, 0.0, 200.0, 200.0));

    rec.process(Point::new(50.0, 50.0), true);
    let events = rec.process(Point::new(80.0, 50.0), true);
    assert!(events.iter().any(|e| matches!(e, GestureEvent::Pan { .. })));
}

#[test]
fn pan_recognizer_no_movement() {
    let mut rec = PanRecognizer::new(rect(0.0, 0.0, 200.0, 200.0));

    rec.process(Point::new(50.0, 50.0), true);
    let events = rec.process(Point::new(52.0, 52.0), true);
    assert!(events.is_empty());
}
