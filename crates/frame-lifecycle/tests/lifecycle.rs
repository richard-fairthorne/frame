use frame_core::plugin::Plugin;
use frame_lifecycle::{LifecyclePlugin, LifecycleEvent};

#[test]
fn lifecycle_plugin_init() {
    let mut plugin = LifecyclePlugin::new();
    let emitter = frame_core::plugin::EventEmitter::new();
    let mut ctx = frame_core::plugin::PluginContext::new(&emitter);
    plugin.init(&mut ctx);
}

#[test]
fn lifecycle_events_emit() {
    let mut plugin = LifecyclePlugin::new();
    let emitter = frame_core::plugin::EventEmitter::new();
    let mut ctx = frame_core::plugin::PluginContext::new(&emitter);
    plugin.init(&mut ctx);

    plugin.on_pause();
    plugin.on_resume();
    plugin.on_destroy();

    let stream = plugin.events().unwrap();
    assert_eq!(stream.try_recv(), Some(LifecycleEvent::Paused));
    assert_eq!(stream.try_recv(), Some(LifecycleEvent::Resumed));
    assert_eq!(stream.try_recv(), Some(LifecycleEvent::Destroying));
    assert_eq!(stream.try_recv(), None);
}
