use super::event::EventEmitter;

pub struct PluginContext<'a> {
    events: &'a EventEmitter,
}

impl<'a> PluginContext<'a> {
    pub fn new(events: &'a EventEmitter) -> Self {
        Self { events }
    }

    pub fn create_stream<T: 'static + Send + Sync>(&self) -> super::EventStream<T> {
        self.events.create_stream()
    }

    pub fn events(&self) -> &EventEmitter {
        self.events
    }
}
