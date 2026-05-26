pub struct AppBuilder {
    pub(crate) title: String,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) resizable: bool,
    pub(crate) plugins: Vec<Box<dyn frame_core::Plugin>>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            title: "Frame App".into(),
            width: 390.0,
            height: 844.0,
            resizable: false,
            plugins: Vec::new(),
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.into();
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn plugin(mut self, plugin: Box<dyn frame_core::Plugin>) -> Self {
        self.plugins.push(plugin);
        self
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_resizable(&self) -> bool {
        self.resizable
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "ios")]
pub fn run_app<F>(builder: AppBuilder, root_factory: F)
where
    F: Fn() -> Box<dyn frame_core::traits::widget::Widget> + 'static,
{
    use objc2::MainThreadMarker;
    use std::rc::Rc;

    crate::deep_link::register_universal_links(&[]);

    let mtm = MainThreadMarker::new().expect("run_app must be called on the main thread");

    let mut frame_app_builder = frame_core::FrameApp::builder();
    for plugin in builder.plugins {
        frame_app_builder = frame_app_builder.plugin(plugin);
    }
    let mut frame_app = frame_app_builder.build().expect("failed to build FrameApp");
    frame_app.init();

    let mut ios_app = crate::app::IosApp::new()
        .with_title(&builder.title)
        .with_size(builder.width, builder.height);
    ios_app.create_window();

    if let Some(handle) = ios_app.window_handle() {
        ios_app.window_manager().install_tap_gesture(&handle);
    }

    let factory: Rc<dyn Fn() -> Box<dyn frame_core::traits::widget::Widget>> =
        Rc::new(root_factory);

    let app = Rc::new(std::cell::RefCell::new(ios_app));
    let last_size = Rc::new(std::cell::RefCell::new(frame_core::Size::new(builder.width, builder.height)));

    let render_cb = {
        let app = app.clone();
        let factory = factory.clone();
        let last_size = last_size.clone();
        Rc::new(move || {
            let mut root = factory();
            let mut app = app.borrow_mut();

            if let Some(handle) = app.window_handle() {
                if let Some(new_size) = app.window_manager().root_view_bounds(&handle) {
                    let mut last = last_size.borrow_mut();
                    if *last != new_size {
                        app.resize(new_size);
                        *last = new_size;
                    }
                }
            }

            app.tick(root.as_mut());
            app.render(root.as_mut());
        }) as Rc<dyn Fn()>
    };

    crate::render_callback::set_render_callback(render_cb.clone());

    (render_cb)();

    unsafe {
        let _ = mtm;
        objc2_ui_kit::UIApplicationMain(
            0,
            std::ptr::null(),
            None,
            Some(&*objc2_foundation::NSString::from_str("FrameAppDelegate")),
        );
    }

    frame_app.destroy();
}

#[cfg(not(target_os = "ios"))]
pub fn run_app<F>(_builder: AppBuilder, _root_factory: F)
where
    F: Fn() -> Box<dyn frame_core::traits::widget::Widget> + 'static,
{
}
