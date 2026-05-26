use frame_core::plugin::Plugin;
use frame_input::{InputPlugin, InputEvent, KeyEvent, TouchEvent, MouseEvent};

#[test]
fn input_plugin_init() {
    let mut plugin = InputPlugin::new();
    let emitter = frame_core::plugin::EventEmitter::new();
    let mut ctx = frame_core::plugin::PluginContext::new(&emitter);
    plugin.init(&mut ctx);
}

#[test]
fn input_event_variants() {
    let key = InputEvent::Key(KeyEvent { key: "a".into(), key_code: frame_input::KeyCode::A, pressed: true, modifiers: Default::default() });
    let touch = InputEvent::Touch(TouchEvent { x: 10.0, y: 20.0, phase: frame_input::TouchPhase::Started, id: 0 });
    let mouse = InputEvent::Mouse(MouseEvent { x: 5.0, y: 5.0, button: frame_input::MouseButton::Left, pressed: true });
    match key { InputEvent::Key(_) => {}, _ => panic!() }
    match touch { InputEvent::Touch(_) => {}, _ => panic!() }
    match mouse { InputEvent::Mouse(_) => {}, _ => panic!() }
}
