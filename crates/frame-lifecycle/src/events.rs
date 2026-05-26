#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LifecycleEvent {
    Started,
    Paused,
    Resumed,
    Stopped,
    Destroying,
}
