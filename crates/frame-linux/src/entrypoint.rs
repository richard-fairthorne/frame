use frame_core::traits::widget::Widget;

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

#[cfg(target_os = "linux")]
pub fn run_app<F>(builder: AppBuilder, root_factory: F)
where
    F: Fn() -> Box<dyn Widget> + 'static,
{
    use crate::wgpu_surface::WgpuSurfaceProvider;
    use frame_core::{Size, WindowId};
    use frame_rendering::framecoord::Frame;
    use glib::clone;
    use gtk::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    crate::deep_link::register_uri_scheme("");

    let mut frame_app_builder = frame_core::FrameApp::builder();
    for plugin in builder.plugins.drain(..) {
        frame_app_builder = frame_app_builder.plugin(plugin);
    }
    let mut frame_app = frame_app_builder.build().expect("failed to build FrameApp");
    frame_app.init();

    let app = gtk::Application::builder()
        .application_id("com.frame.app")
        .build();

    app.connect_activate(clone!(@strong builder, @strong root_factory => move |app| {
        let drawing_area = gtk::DrawingArea::builder()
            .content_width(builder.width as i32)
            .content_height(builder.height as i32)
            .build();

        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title(&builder.title)
            .default_width(builder.width as i32)
            .default_height(builder.height as i32)
            .resizable(builder.resizable)
            .child(&drawing_area)
            .build();

        window.show_all();

        let gdk_window = window.window().expect("widget must be realized");
        let scale: f32 = gdk_window.scale_factor() as f32;

        let surface_provider = WgpuSurfaceProvider::new();
        let mut frame_coord = Frame::new(surface_provider);
        let window_id = WindowId::default();
        let window_size = Size::new(builder.width, builder.height);

        frame_coord.set_scale_factor(scale);
        frame_coord
            .create_surface(window_id, window_size)
            .expect("failed to create surface");

        let physical_w = (builder.width * scale) as u32;
        let physical_h = (builder.height * scale) as u32;
        {
            let surface_provider = frame_coord.surface_provider_mut();
            surface_provider
                .init_surface_from_gdk(window_id, &gdk_window, physical_w, physical_h)
                .expect("failed to init wgpu surface");
        }

        window.connect_configure_event(clone!(@strong frame_coord => move |_, event| {
            let width = event.width() as f32;
            let height = event.height() as f32;
            let mut fc = frame_coord.borrow_mut();
            fc.resize(window_id, frame_core::Size::new(width, height));
            false
        }));

        let frame_coord = Rc::new(RefCell::new(frame_coord));
        let factory: Rc<dyn Fn() -> Box<dyn Widget>> = Rc::new(root_factory);

        drawing_area.add_events(gtk::gdk::EventMask::BUTTON_PRESS_MASK);
        drawing_area.connect_button_press_event(move |_widget, event| {
            let (x, y) = event.position();
            let point = frame_core::Point::new(x as f32, y as f32);
            frame_rendering::click::dispatch_click(point);
            gtk::Inhibit(false)
        });

        window.connect_key_press_event(|_widget, _event| {
            gtk::Inhibit(false)
        });

        {
            let frame_coord = frame_coord.clone();
            let factory = factory.clone();
            let window = window.clone();
            glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
                let mut root = factory();
                let mut fc = frame_coord.borrow_mut();
                fc.tick(root.as_mut());
                if fc.is_dirty() {
                    let _ = fc.render(window_id, root.as_mut());
                    window.queue_draw();
                }
                glib::ControlFlow::Continue
            });
        }
    }));

    app.run_with_args(&std::env::args().collect::<Vec<_>>());
    frame_app.destroy();
}

#[cfg(not(target_os = "linux"))]
pub fn run_app<F>(_builder: AppBuilder, _root_factory: F)
where
    F: Fn() -> Box<dyn Widget> + 'static,
{
    let _ = (_builder, _root_factory);
}
