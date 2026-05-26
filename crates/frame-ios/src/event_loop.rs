use frame_core::plugin::EventEmitter;
use frame_input::{InputEvent, TouchEvent, TouchPhase};
use std::sync::Arc;

pub struct IosEventLoop {
    emitter: Arc<EventEmitter>,
    stream: frame_core::plugin::EventStream<InputEvent>,
    running: bool,
}

impl IosEventLoop {
    pub fn new(emitter: Arc<EventEmitter>) -> Self {
        let stream = emitter.create_stream::<InputEvent>();
        Self {
            emitter,
            stream,
            running: false,
        }
    }

    pub fn process_touch_event(&self, x: f32, y: f32, phase: TouchPhase, touch_id: u64) {
        let event = InputEvent::Touch(TouchEvent {
            x,
            y,
            phase,
            id: touch_id,
        });
        self.stream.emit(event);
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn emitter(&self) -> &Arc<EventEmitter> {
        &self.emitter
    }
}

#[cfg(target_os = "ios")]
pub fn install_touch_handlers(view: &objc2_ui_kit::UIView, event_loop: &IosEventLoop) {
    use frame_input::TouchPhase;
    use std::sync::Arc;

    let emitter = event_loop.emitter().clone();
    let stream = emitter.create_stream::<InputEvent>();

    unsafe {
        let _: () = objc2::msg_send![view, setMultipleTouchEnabled: true];
    }

    let _ = (stream, emitter);
}

#[cfg(not(target_os = "ios"))]
pub fn install_touch_handlers() {}
