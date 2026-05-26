pub mod r#trait;
pub mod context;
pub mod host;
pub mod event;

pub use r#trait::Plugin;
pub use context::PluginContext;
pub use host::PluginHost;
pub use event::{EventEmitter, EventStream};
