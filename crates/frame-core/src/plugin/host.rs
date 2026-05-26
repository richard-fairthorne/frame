use super::Plugin;
use super::context::PluginContext;
use super::event::EventEmitter;

pub struct PluginHost {
    plugins: Vec<Box<dyn Plugin>>,
    emitter: EventEmitter,
}

impl PluginHost {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            emitter: EventEmitter::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn init_all(&mut self) {
        let mut ctx = PluginContext::new(&self.emitter);
        for plugin in &mut self.plugins {
            plugin.init(&mut ctx);
        }
    }

    pub fn pause_all(&mut self) {
        for plugin in &mut self.plugins {
            plugin.on_pause();
        }
    }

    pub fn resume_all(&mut self) {
        for plugin in &mut self.plugins {
            plugin.on_resume();
        }
    }

    pub fn destroy_all(&mut self) {
        for plugin in &mut self.plugins {
            plugin.on_destroy();
        }
    }
}

impl Default for PluginHost {
    fn default() -> Self {
        Self::new()
    }
}
