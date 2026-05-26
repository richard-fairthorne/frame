use frame_core::plugin::{Plugin, PluginContext, EventStream};
use crate::events::LifecycleEvent;

pub struct LifecyclePlugin {
    stream: Option<EventStream<LifecycleEvent>>,
}

impl LifecyclePlugin {
    pub fn new() -> Self {
        Self { stream: None }
    }

    pub fn events(&self) -> Option<&EventStream<LifecycleEvent>> {
        self.stream.as_ref()
    }
}

impl Default for LifecyclePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for LifecyclePlugin {
    fn init(&mut self, ctx: &mut PluginContext<'_>) {
        self.stream = Some(ctx.create_stream::<LifecycleEvent>());
    }

    fn on_pause(&mut self) {
        if let Some(stream) = &self.stream {
            stream.emit(LifecycleEvent::Paused);
        }
    }

    fn on_resume(&mut self) {
        if let Some(stream) = &self.stream {
            stream.emit(LifecycleEvent::Resumed);
        }
    }

    fn on_destroy(&mut self) {
        if let Some(stream) = &self.stream {
            stream.emit(LifecycleEvent::Destroying);
        }
    }
}
