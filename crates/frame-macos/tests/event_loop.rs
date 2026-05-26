use std::sync::Arc;
use frame_core::plugin::EventEmitter;
use frame_macos::{MacosEventLoop, MacosMouseInput, MacosMouseKind, MacosKeyboardInput, MacosKeyboardKind};

#[test]
fn event_loop_creation() {
    let emitter = Arc::new(EventEmitter::new());
    let loop_ = MacosEventLoop::new(emitter);
    assert!(!loop_.is_running());
}

#[test]
fn event_loop_process_mouse() {
    let emitter = Arc::new(EventEmitter::new());
    let loop_ = MacosEventLoop::new(emitter);
    loop_.process_mouse_event(MacosMouseInput {
        kind: MacosMouseKind::Down,
        button: 0,
        x: 100.0,
        y: 200.0,
    });
}

#[test]
fn event_loop_process_keyboard() {
    let emitter = Arc::new(EventEmitter::new());
    let loop_ = MacosEventLoop::new(emitter);
    loop_.process_keyboard_event(MacosKeyboardInput {
        kind: MacosKeyboardKind::Down,
        keycode: 0x00,
    });
}
