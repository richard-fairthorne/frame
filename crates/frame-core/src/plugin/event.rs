use std::collections::VecDeque;
use std::sync::RwLock;

pub struct EventEmitter;

impl EventEmitter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_stream<T: 'static + Send + Sync>(&self) -> EventStream<T> {
        EventStream::new()
    }
}

impl Default for EventEmitter {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EventStream<T> {
    buffer: RwLock<VecDeque<T>>,
}

impl<T> EventStream<T> {
    fn new() -> Self {
        Self {
            buffer: RwLock::new(VecDeque::new()),
        }
    }

    pub fn emit(&self, value: T) {
        self.buffer.write().unwrap().push_back(value);
    }

    pub fn try_recv(&self) -> Option<T> {
        self.buffer.write().unwrap().pop_front()
    }
}
