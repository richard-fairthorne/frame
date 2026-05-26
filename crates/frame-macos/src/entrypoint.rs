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
            width: 800.0,
            height: 600.0,
            resizable: true,
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

#[cfg(target_os = "macos")]
pub fn run_app<F>(builder: AppBuilder, root_factory: F)
where
    F: Fn() -> Box<dyn frame_core::traits::widget::Widget> + 'static,
{
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};
    use std::rc::Rc;

    crate::deep_link::register_universal_links(&[]);

    let mtm = MainThreadMarker::new().expect("run_app must be called on the main thread");
    let ns_app = NSApplication::sharedApplication(mtm);
    ns_app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

    let mut frame_app = frame_core::FrameApp::builder();
    for plugin in builder.plugins {
        frame_app = frame_app.plugin(plugin);
    }
    let mut frame_app = frame_app.build().expect("failed to build FrameApp");

    let mut macos_app = crate::app::MacosApp::new()
        .with_title(&builder.title)
        .with_size(builder.width, builder.height);
    macos_app.create_window();

    frame_app.init();

    let window_height = builder.height;

    let factory: Rc<dyn Fn() -> Box<dyn frame_core::traits::widget::Widget>> = Rc::new(root_factory);

    let app = Rc::new(std::cell::RefCell::new(macos_app));

    let render_cb = {
        let app = app.clone();
        let factory = factory.clone();
        Rc::new(move || {
            let mut root = factory();
            let mut app = app.borrow_mut();
            app.check_window_resize();
            app.tick(root.as_mut());
            app.render(root.as_mut());
        }) as Rc<dyn Fn()>
    };

    crate::render_callback::set_render_callback(render_cb.clone());

    (render_cb)();

    install_mouse_monitor(window_height);
    install_keyboard_monitor();

    ns_app.activate();
    ns_app.run();

    frame_app.destroy();
}

#[cfg(target_os = "macos")]
fn install_mouse_monitor(window_height: f32) {
    use std::ptr::NonNull;
    use objc2_app_kit::{NSEvent, NSEventMask, NSEventType};
    use frame_core::Point;
    use frame_rendering::dispatch_click;

    let block = block2::StackBlock::new(move |event: NonNull<NSEvent>| -> *mut NSEvent {
        unsafe {
            let event: &NSEvent = &*event.as_ptr();
            if event.r#type() == NSEventType::LeftMouseDown {
                let loc = event.locationInWindow();
                let point = Point::new(loc.x as f32, (window_height - loc.y as f32).max(0.0));
                dispatch_click(point);
            }
        }
        event.as_ptr()
    });

    unsafe {
        NSEvent::addLocalMonitorForEventsMatchingMask_handler(
            NSEventMask::LeftMouseDown,
            &block,
        );
    }
}

#[cfg(target_os = "macos")]
fn install_keyboard_monitor() {
    use std::ptr::NonNull;
    use objc2_app_kit::{NSEvent, NSEventMask};

    let block = block2::StackBlock::new(move |event: NonNull<NSEvent>| -> *mut NSEvent {
        event.as_ptr()
    });

    unsafe {
        NSEvent::addLocalMonitorForEventsMatchingMask_handler(
            NSEventMask::KeyDown,
            &block,
        );
    }
}

#[cfg(not(target_os = "macos"))]
pub fn run_app<F>(builder: AppBuilder, root_factory: F)
where
    F: Fn() -> Box<dyn frame_core::traits::widget::Widget> + 'static,
{
    let mut frame_app = frame_core::FrameApp::builder();
    for plugin in builder.plugins {
        frame_app = frame_app.plugin(plugin);
    }
    let mut frame_app = frame_app.build().expect("failed to build FrameApp");

    let mut macos_app = crate::app::MacosApp::new().with_size(builder.width, builder.height);
    macos_app.create_window();

    frame_app.init();

    let mut root = root_factory();
    macos_app.tick(root.as_mut());
    macos_app.render(root.as_mut());

    frame_app.destroy();
}
