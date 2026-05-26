pub trait Plugin: 'static + Send + Sync {
    fn init(&mut self, ctx: &mut super::PluginContext<'_>);
    fn on_pause(&mut self) {}
    fn on_resume(&mut self) {}
    fn on_destroy(&mut self) {}
}
