use crate::plugin::PluginHost;

pub struct FrameApp {
    plugin_host: PluginHost,
}

impl FrameApp {
    pub fn builder() -> FrameAppBuilder {
        FrameAppBuilder {
            plugin_host: PluginHost::new(),
        }
    }

    pub fn init(&mut self) {
        self.plugin_host.init_all();
    }

    pub fn pause(&mut self) {
        self.plugin_host.pause_all();
    }

    pub fn resume(&mut self) {
        self.plugin_host.resume_all();
    }

    pub fn destroy(&mut self) {
        self.plugin_host.destroy_all();
    }
}

pub struct FrameAppBuilder {
    plugin_host: PluginHost,
}

impl FrameAppBuilder {
    pub fn plugin(mut self, plugin: Box<dyn crate::plugin::Plugin>) -> Self {
        self.plugin_host.add_plugin(plugin);
        self
    }

    pub fn build(self) -> Result<FrameApp, String> {
        Ok(FrameApp {
            plugin_host: self.plugin_host,
        })
    }
}
