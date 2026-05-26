use frame_core::interaction::{InteractionRegistry, InteractionEvent, InteractionId};
use frame_core::{Rect, Point, Size};
use std::sync::{Arc, Mutex};

#[test]
fn hit_test_finds_registered_area() {
    let mut registry = InteractionRegistry::new();
    let id = registry.register(Rect::new(Point::new(10.0, 10.0), Size::new(100.0, 50.0)));

    let hit = registry.hit_test(Point::new(50.0, 30.0));
    assert_eq!(hit, Some(id));

    let miss = registry.hit_test(Point::new(5.0, 5.0));
    assert_eq!(miss, None);
}

#[test]
fn process_mouse_tap() {
    let mut registry = InteractionRegistry::new();
    let _id = registry.register(Rect::new(Point::ZERO, Size::new(100.0, 100.0)));

    let events = registry.process_mouse_input(50.0, 50.0, true);
    assert!(events.iter().any(|e| matches!(e, InteractionEvent::Tap { .. })));
}

#[test]
fn process_mouse_hover_change() {
    let mut registry = InteractionRegistry::new();
    registry.register(Rect::new(Point::ZERO, Size::new(100.0, 100.0)));

    let events1 = registry.process_mouse_input(50.0, 50.0, false);
    let events2 = registry.process_mouse_input(150.0, 150.0, false);

    assert!(events1.iter().any(|e| matches!(e, InteractionEvent::Hover { .. })));
}

#[test]
fn dispatch_callback() {
    let mut registry = InteractionRegistry::new();
    let id = registry.register(Rect::new(Point::ZERO, Size::new(100.0, 100.0)));

    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();
    registry.set_callback(id, Box::new(move |_event| {
        *called_clone.lock().unwrap() = true;
    }));

    registry.dispatch_event(InteractionEvent::Tap { id, position: Point::new(50.0, 50.0) });
    assert!(*called.lock().unwrap());
}
