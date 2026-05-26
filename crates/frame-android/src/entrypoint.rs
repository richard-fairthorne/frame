use frame_core::traits::widget::Widget;

pub struct AppBuilder {
    pub(crate) title: String,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) plugins: Vec<Box<dyn frame_core::Plugin>>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            title: "Frame App".into(),
            width: 800.0,
            height: 600.0,
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
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run_app_with<F>(android_app: android_activity::AndroidApp, builder: AppBuilder, root_factory: F)
where
    F: Fn() -> Box<dyn Widget> + 'static,
{
    use android_activity::{InputStatus, MainEvent, PollEvent, input::InputEvent};
    use std::sync::Arc;
    use std::time::Duration;

    use crate::native_window::NativeWindow;
    use crate::wgpu_surface::AndroidSurfaceProvider;
    use frame_core::traits::window::{WindowConfig, WindowHost};
    use frame_core::WindowId;
    use frame_rendering::framecoord::Frame;
    use frame_rendering::SurfaceProvider;

    crate::deep_link::register_app_links(&[]);

    let mut frame_app_builder = frame_core::FrameApp::builder();
    for plugin in builder.plugins {
        frame_app_builder = frame_app_builder.plugin(plugin);
    }
    let mut frame_app = frame_app_builder.build().expect("failed to build FrameApp");

    let title = builder.title.clone();

    let density: f32 = android_app
        .config()
        .density()
        .map(|dpi| dpi as f32 / 160.0)
        .unwrap_or(2.625);
    eprintln!("frame-android: density={}", density);

    let mut window_manager = crate::platform::AndroidWindowManager::new();
    let mut frame: Option<Frame<AndroidSurfaceProvider>> = None;
    let mut window_id: Option<WindowId> = None;
    let mut native_window: Option<Arc<NativeWindow>> = None;
    let mut initialized = false;
    let mut frame_app_inited = false;
    let mut should_destroy = false;

    let init_frame = |our_nw: &Arc<NativeWindow>, w: u32, h: u32| -> Frame<AndroidSurfaceProvider> {
        let logical_w = w as f32 / density;
        let logical_h = h as f32 / density;

        let mut f = Frame::new(AndroidSurfaceProvider::new());
        f.set_scale_factor(density);
        f.create_surface(WindowId::default(), frame_core::Size::new(logical_w, logical_h))
            .expect("failed to create surface");
        f.surface_provider_mut()
            .init_surface_from_native_window(WindowId::default(), our_nw, w, h)
            .expect("failed to init surface from native window");
        eprintln!(
            "frame-android: surface initialized, physical={}x{}, logical={}x{}, density={}",
            w, h, logical_w, logical_h, density
        );
        f
    };

    eprintln!("frame-android: run_app_with started, checking native_window at startup");
    if let Some(nw) = android_app.native_window() {
        eprintln!("frame-android: native_window available at startup!");
        let ptr = nw.ptr().as_ptr();
        if let Some(our_nw) = unsafe { NativeWindow::from_ptr(ptr) } {
            let our_nw = our_nw.into_arc();
            let w = our_nw.width();
            let h = our_nw.height();

            let _handle = window_manager.create_window(WindowConfig {
                title: title.clone(),
                size: frame_core::Size::new(w as f32 / density, h as f32 / density),
                resizable: false,
            });

            let id = WindowId::default();
            let f = init_frame(&our_nw, w, h);

            window_id = Some(id);
            native_window = Some(our_nw);
            frame = Some(f);
            initialized = true;
            if !frame_app_inited {
                frame_app.init();
                frame_app_inited = true;
            }
        }
    } else {
        eprintln!("frame-android: native_window NOT available at startup, waiting for InitWindow");
    }

    while !should_destroy {
        android_app.poll_events(Some(Duration::from_millis(16)), |event| {
            match event {
                PollEvent::Main(MainEvent::InitWindow { .. }) => {
                    eprintln!("frame-android: InitWindow event received, initialized={}", initialized);
                    if !initialized {
                        if let Some(nw) = android_app.native_window() {
                            let ptr = nw.ptr().as_ptr();
                            if let Some(our_nw) = unsafe { NativeWindow::from_ptr(ptr) } {
                                let our_nw = our_nw.into_arc();
                                let w = our_nw.width();
                                let h = our_nw.height();

                                let _handle = window_manager.create_window(WindowConfig {
                                    title: title.clone(),
                                    size: frame_core::Size::new(w as f32 / density, h as f32 / density),
                                    resizable: false,
                                });

                                let id = WindowId::default();
                                let f = init_frame(&our_nw, w, h);

                                window_id = Some(id);
                                native_window = Some(our_nw);
                                frame = Some(f);
                                initialized = true;
                                if !frame_app_inited {
                                    frame_app.init();
                                    frame_app_inited = true;
                                }
                            }
                        }
                    }
                }
                PollEvent::Main(MainEvent::TerminateWindow { .. }) => {
                    if let Some(id) = window_id.take() {
                        if let Some(ref mut f) = frame {
                            f.surface_provider_mut().destroy_surface(
                                frame_rendering::SurfaceHandle {
                                    id,
                                    width: 0.0,
                                    height: 0.0,
                                    raw_handle: None,
                                },
                            );
                        }
                    }
                    native_window = None;
                    frame = None;
                    initialized = false;
                }
                PollEvent::Main(MainEvent::WindowResized { .. }) => {
                    if let (Some(ref nw), Some(id)) = (&native_window, window_id) {
                        let w = nw.width();
                        let h = nw.height();
                        if let Some(ref mut f) = frame {
                            let logical_w = w as f32 / density;
                            let logical_h = h as f32 / density;
                            f.resize(id, frame_core::Size::new(logical_w, logical_h));
                        }
                    }
                }
                PollEvent::Main(MainEvent::Pause) => {
                    frame_app.pause();
                    if let Some(ref mut f) = frame {
                        f.mark_dirty();
                    }
                }
                PollEvent::Main(MainEvent::Resume { .. }) => {
                    frame_app.resume();
                    if let Some(ref mut f) = frame {
                        f.mark_dirty();
                    }
                }
                PollEvent::Main(MainEvent::Destroy) => {
                    frame_app.destroy();
                    should_destroy = true;
                }
                _ => {}
            }
        });

        if let Ok(mut input_iter) = android_app.input_events_iter() {
            while input_iter.next(|event| {
                if let InputEvent::MotionEvent(motion) = event {
                    use android_activity::input::MotionAction;
                    if motion.action() == MotionAction::Up && motion.pointer_count() > 0 {
                        let pointer = motion.pointer_at_index(0);
                        let point = frame_core::Point::new(
                            pointer.x() / density,
                            pointer.y() / density,
                        );
                        frame_rendering::click::dispatch_click(point);
                    }
                }
                InputStatus::Handled
            }) {}
        }

        if initialized {
            if let (Some(id), Some(ref mut f)) = (window_id, &mut frame) {
                f.mark_dirty();
                let mut root = root_factory();
                f.tick(root.as_mut());
                let _ = f.render(id, root.as_mut());
            } else {
                eprintln!("frame-android: initialized but frame/window_id is None");
            }
        }
    }
}

#[cfg(not(target_os = "android"))]
pub fn run_app<F>(_builder: AppBuilder, _root_factory: F)
where
    F: Fn() -> Box<dyn Widget> + 'static,
{
    unimplemented!("Frame Android: can only run on Android");
}
