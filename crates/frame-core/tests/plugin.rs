use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use frame_core::plugin::{Plugin, PluginHost, PluginContext, EventEmitter, EventStream};

struct TestPlugin {
    initialized: Arc<AtomicBool>,
    paused: Arc<AtomicBool>,
    destroyed: Arc<AtomicBool>,
}

impl TestPlugin {
    fn new() -> (Self, TestPluginState) {
        let initialized = Arc::new(AtomicBool::new(false));
        let paused = Arc::new(AtomicBool::new(false));
        let destroyed = Arc::new(AtomicBool::new(false));

        let state = TestPluginState {
            initialized: initialized.clone(),
            paused: paused.clone(),
            destroyed: destroyed.clone(),
        };

        (Self { initialized, paused, destroyed }, state)
    }
}

struct TestPluginState {
    initialized: Arc<AtomicBool>,
    paused: Arc<AtomicBool>,
    destroyed: Arc<AtomicBool>,
}

impl Plugin for TestPlugin {
    fn init(&mut self, _ctx: &mut PluginContext<'_>) {
        self.initialized.store(true, Ordering::SeqCst);
    }

    fn on_pause(&mut self) {
        self.paused.store(true, Ordering::SeqCst);
    }

    fn on_resume(&mut self) {
        self.paused.store(false, Ordering::SeqCst);
    }

    fn on_destroy(&mut self) {
        self.destroyed.store(true, Ordering::SeqCst);
    }
}

#[test]
fn plugin_lifecycle() {
    let (plugin, state) = TestPlugin::new();
    let mut host = PluginHost::new();
    host.add_plugin(Box::new(plugin));

    assert!(!state.initialized.load(Ordering::SeqCst));

    host.init_all();
    assert!(state.initialized.load(Ordering::SeqCst));

    host.pause_all();
    assert!(state.paused.load(Ordering::SeqCst));

    host.resume_all();
    assert!(!state.paused.load(Ordering::SeqCst));

    host.destroy_all();
    assert!(state.destroyed.load(Ordering::SeqCst));
}

#[test]
fn event_stream_create_and_emit() {
    let emitter = EventEmitter::new();
    let stream: EventStream<i32> = emitter.create_stream();

    assert!(stream.try_recv().is_none());

    stream.emit(42);
    assert_eq!(stream.try_recv(), Some(42));
    assert!(stream.try_recv().is_none());
}

#[test]
fn event_stream_multiple_values() {
    let emitter = EventEmitter::new();
    let stream: EventStream<i32> = emitter.create_stream();

    stream.emit(1);
    stream.emit(2);
    stream.emit(3);

    assert_eq!(stream.try_recv(), Some(1));
    assert_eq!(stream.try_recv(), Some(2));
    assert_eq!(stream.try_recv(), Some(3));
    assert!(stream.try_recv().is_none());
}
