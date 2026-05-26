use frame_core::plugin::{Plugin, PluginContext, EventStream};
use crate::events::InputEvent;

pub struct InputPlugin {
    stream: Option<EventStream<InputEvent>>,
}

impl InputPlugin {
    pub fn new() -> Self {
        Self { stream: None }
    }

    pub fn events(&self) -> Option<&EventStream<InputEvent>> {
        self.stream.as_ref()
    }
}

impl Default for InputPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for InputPlugin {
    fn init(&mut self, ctx: &mut PluginContext<'_>) {
        self.stream = Some(ctx.create_stream::<InputEvent>());
    }
}
