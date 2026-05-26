use frame_input::{InputEvent, KeyEvent, MouseEvent, MouseButton, KeyCode};
use frame_core::plugin::EventEmitter;
use std::sync::Arc;

pub struct MacosEventLoop {
    emitter: Arc<EventEmitter>,
    stream: frame_core::plugin::EventStream<InputEvent>,
    running: bool,
}

impl MacosEventLoop {
    pub fn new(emitter: Arc<EventEmitter>) -> Self {
        let stream = emitter.create_stream::<InputEvent>();
        Self {
            emitter,
            stream,
            running: false,
        }
    }

    pub fn process_mouse_event(&self, event: MacosMouseInput) {
        let frame_event = match event.kind {
            MacosMouseKind::Down => InputEvent::Mouse(MouseEvent::pressed(
                MouseButton::from_u8(event.button),
                event.x,
                event.y,
            )),
            MacosMouseKind::Up => InputEvent::Mouse(MouseEvent::released(
                MouseButton::from_u8(event.button),
                event.x,
                event.y,
            )),
            MacosMouseKind::Move => InputEvent::Mouse(MouseEvent::moved(event.x, event.y)),
        };
        self.stream.emit(frame_event);
    }

    pub fn process_keyboard_event(&self, event: MacosKeyboardInput) {
        let frame_event = match event.kind {
            MacosKeyboardKind::Down => InputEvent::Key(KeyEvent::key_down(
                KeyCode::from_u32(event.keycode),
            )),
            MacosKeyboardKind::Up => InputEvent::Key(KeyEvent::key_up(
                KeyCode::from_u32(event.keycode),
            )),
        };
        self.stream.emit(frame_event);
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn emitter(&self) -> &Arc<EventEmitter> {
        &self.emitter
    }
}

pub struct MacosMouseInput {
    pub kind: MacosMouseKind,
    pub button: u8,
    pub x: f32,
    pub y: f32,
}

pub enum MacosMouseKind {
    Down,
    Up,
    Move,
}

pub struct MacosKeyboardInput {
    pub kind: MacosKeyboardKind,
    pub keycode: u32,
}

pub enum MacosKeyboardKind {
    Down,
    Up,
}
