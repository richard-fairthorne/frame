#[cfg(target_arch = "wasm32")]
use frame_core::traits::widget::Widget;
#[cfg(target_arch = "wasm32")]
use frame_core::Point;
#[cfg(target_arch = "wasm32")]
use frame_input::events::{
    InputEvent, KeyCode, KeyEvent, Modifiers, MouseButton, MouseEvent, TouchEvent, TouchPhase,
};
#[cfg(target_arch = "wasm32")]
use frame_input::InputPlugin;
#[cfg(target_arch = "wasm32")]
use frame_rendering::click::dispatch_click;
#[cfg(target_arch = "wasm32")]
use frame_rendering::framecoord::Frame;
#[cfg(target_arch = "wasm32")]
use frame_rendering::surface::SurfaceProvider;
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{HtmlCanvasElement, KeyboardEvent, MouseEvent as WebMouseEvent};

pub struct AppBuilder {
    pub title: String,
    pub width: f32,
    pub height: f32,
    #[allow(dead_code)]
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
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_arch = "wasm32")]
struct AppState<F>
where
    F: Fn() -> Box<dyn Widget> + 'static,
{
    frame: Frame<crate::wgpu_surface::WebSurfaceProvider>,
    root: Box<dyn Widget>,
    _root_factory: F,
    window_id: frame_core::WindowId,
}

#[cfg(target_arch = "wasm32")]
pub fn run_app<F>(builder: AppBuilder, root_factory: F)
where
    F: Fn() -> Box<dyn Widget> + 'static,
{
    use crate::wgpu_surface::WebSurfaceProvider;

    crate::deep_link::setup_history_routing();

    let mut frame_app_builder = frame_core::FrameApp::builder();
    for plugin in builder.plugins {
        frame_app_builder = frame_app_builder.plugin(plugin);
    }
    let mut frame_app = frame_app_builder.build().expect("failed to build FrameApp");
    frame_app.init();

    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");

    document.set_title(&builder.title);

    let canvas = document
        .create_element("canvas")
        .expect("failed to create canvas")
        .dyn_into::<HtmlCanvasElement>()
        .expect("failed to cast to HtmlCanvasElement");

    let dpr = web_sys::window()
        .expect("no window")
        .device_pixel_ratio() as f32;
    let physical_w = (builder.width * dpr) as u32;
    let physical_h = (builder.height * dpr) as u32;

    canvas.set_width(physical_w);
    canvas.set_height(physical_h);
    canvas
        .style()
        .set_property("width", &format!("{}px", builder.width as u32))
        .expect("failed to set width");
    canvas
        .style()
        .set_property("height", &format!("{}px", builder.height as u32))
        .expect("failed to set height");
    canvas
        .style()
        .set_property("display", "block")
        .expect("failed to set display");
    canvas
        .style()
        .set_property("margin", "0 auto")
        .expect("failed to set margin");

    document
        .body()
        .expect("no body")
        .append_child(&canvas)
        .expect("failed to append canvas");

    let input_plugin = InputPlugin::new();
    let emitter = frame_core::plugin::EventEmitter::new();
    let mut ctx = frame_core::plugin::PluginContext::new(&emitter);
    let _ = (&input_plugin, &mut ctx);

    let canvas_for_events = canvas.clone();
    let emitter_for_events = emitter.clone();
    setup_event_listeners(&canvas_for_events, &emitter_for_events);

    let mut surface_provider = WebSurfaceProvider::new();

    let window_id = frame_core::WindowId::default();
    let size = frame_core::Size::new(builder.width, builder.height);
    let physical_size = frame_core::Size::new(physical_w as f32, physical_h as f32);

    let handle = surface_provider
        .create_surface(window_id, physical_size)
        .expect("failed to create surface handle");

    let mut frame = Frame::new(surface_provider);
    frame.set_scale_factor(dpr);
    let _ = frame.create_surface(window_id, size);

    let root = root_factory();

    let state = Rc::new(RefCell::new(AppState {
        frame,
        root,
        _root_factory: root_factory,
        window_id,
    }));

    let canvas_for_resize = canvas.clone();
    let dpr_for_resize = dpr;
    let state_for_resize = state.clone();
    let handle_for_resize = handle.clone();
    setup_resize_listener(&canvas_for_resize, dpr_for_resize, state_for_resize, handle_for_resize);

    let state_for_destroy = state.clone();
    let mut frame_app_for_destroy = RefCell::new(frame_app);
    setup_destroy_handler(&mut frame_app_for_destroy, state_for_destroy);

    let canvas_for_init = canvas.clone();
    let state_for_render = state.clone();
    let state_for_init = state.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let result = {
            let mut s = state_for_init.borrow_mut();
            s.frame.surface_provider_mut()
                .init_surface_from_canvas(window_id, &canvas_for_init, physical_w, physical_h)
                .await
        };

        if let Err(e) = result {
            web_sys::window()
                .and_then(|w| w.document())
                .and_then(|doc| doc.body())
                .map(|body| {
                    let msg = format!("WebGPU initialization failed: {e}. Your browser may not support WebGPU.");
                    let _ = body.set_text_content(Some(&msg));
                });
            return;
        }

        start_render_loop(state_for_render);
    });
}

#[cfg(target_arch = "wasm32")]
fn setup_event_listeners(
    canvas: &HtmlCanvasElement,
    emitter: &frame_core::plugin::EventEmitter,
) {
    let mouse_down_stream = emitter.create_stream::<InputEvent>();
    let closure = Closure::wrap(Box::new(move |event: WebMouseEvent| {
        let button = MouseButton::from_u8(event.button() as u8);
        let target: HtmlCanvasElement = event.target().unwrap().dyn_into().unwrap();
        let rect = target.get_bounding_client_rect();
        let x = event.client_x() as f32 - rect.x() as f32;
        let y = event.client_y() as f32 - rect.y() as f32;

        dispatch_click(Point::new(x, y));

        let me = MouseEvent {
            x,
            y,
            button,
            pressed: true,
        };
        mouse_down_stream.emit(InputEvent::Mouse(me));
    }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
        .expect("failed to add mousedown listener");
    closure.forget();

    let mouse_move_stream = emitter.create_stream::<InputEvent>();
    let closure = Closure::wrap(Box::new(move |event: WebMouseEvent| {
        let target: HtmlCanvasElement = event.target().unwrap().dyn_into().unwrap();
        let rect = target.get_bounding_client_rect();
        let x = event.client_x() as f32 - rect.x() as f32;
        let y = event.client_y() as f32 - rect.y() as f32;
        let me = MouseEvent::moved(x, y);
        mouse_move_stream.emit(InputEvent::Mouse(me));
    }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
        .expect("failed to add mousemove listener");
    closure.forget();

    let mouse_up_stream = emitter.create_stream::<InputEvent>();
    let closure = Closure::wrap(Box::new(move |event: WebMouseEvent| {
        let button = MouseButton::from_u8(event.button() as u8);
        let target: HtmlCanvasElement = event.target().unwrap().dyn_into().unwrap();
        let rect = target.get_bounding_client_rect();
        let x = event.client_x() as f32 - rect.x() as f32;
        let y = event.client_y() as f32 - rect.y() as f32;
        let me = MouseEvent {
            x,
            y,
            button,
            pressed: false,
        };
        mouse_up_stream.emit(InputEvent::Mouse(me));
    }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
        .expect("failed to add mouseup listener");
    closure.forget();

    let key_down_stream = emitter.create_stream::<InputEvent>();
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        let key_code = map_key_code(&event.key());
        let modifiers = Modifiers {
            shift: event.shift_key(),
            ctrl: event.ctrl_key(),
            alt: event.alt_key(),
            meta: event.meta_key(),
        };
        let ke = KeyEvent {
            key: event.key(),
            key_code,
            pressed: true,
            modifiers,
        };
        key_down_stream.emit(InputEvent::Key(ke));
    }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
        .expect("failed to add keydown listener");
    closure.forget();

    let key_up_stream = emitter.create_stream::<InputEvent>();
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        let key_code = map_key_code(&event.key());
        let modifiers = Modifiers {
            shift: event.shift_key(),
            ctrl: event.ctrl_key(),
            alt: event.alt_key(),
            meta: event.meta_key(),
        };
        let ke = KeyEvent {
            key: event.key(),
            key_code,
            pressed: false,
            modifiers,
        };
        key_up_stream.emit(InputEvent::Key(ke));
    }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())
        .expect("failed to add keyup listener");
    closure.forget();

    let touch_stream = emitter.create_stream::<InputEvent>();
    let canvas_rect = canvas.get_bounding_client_rect();
    let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
        let touches = event.touches();
        for i in 0..touches.length() {
            if let Some(touch) = touches.get(i) {
                let x = touch.client_x() as f32 - canvas_rect.x() as f32;
                let y = touch.client_y() as f32 - canvas_rect.y() as f32;
                let te = TouchEvent {
                    x,
                    y,
                    phase: TouchPhase::Started,
                    id: touch.identifier() as u64,
                };
                touch_stream.emit(InputEvent::Touch(te));
            }
        }
    }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())
        .expect("failed to add touchstart listener");
    closure.forget();

    let touch_end_stream = emitter.create_stream::<InputEvent>();
    let canvas_rect_end = canvas.get_bounding_client_rect();
    let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
        let touches = event.changed_touches();
        for i in 0..touches.length() {
            if let Some(touch) = touches.get(i) {
                let x = touch.client_x() as f32 - canvas_rect_end.x() as f32;
                let y = touch.client_y() as f32 - canvas_rect_end.y() as f32;

                dispatch_click(Point::new(x, y));

                let te = TouchEvent {
                    x,
                    y,
                    phase: TouchPhase::Ended,
                    id: touch.identifier() as u64,
                };
                touch_end_stream.emit(InputEvent::Touch(te));
            }
        }
    }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())
        .expect("failed to add touchend listener");
    closure.forget();
}

#[cfg(target_arch = "wasm32")]
fn map_key_code(key: &str) -> KeyCode {
    match key {
        "a" | "A" => KeyCode::A,
        "b" | "B" => KeyCode::B,
        "c" | "C" => KeyCode::C,
        "d" | "D" => KeyCode::D,
        "e" | "E" => KeyCode::E,
        "f" | "F" => KeyCode::F,
        "g" | "G" => KeyCode::G,
        "h" | "H" => KeyCode::H,
        "i" | "I" => KeyCode::I,
        "j" | "J" => KeyCode::J,
        "k" | "K" => KeyCode::K,
        "l" | "L" => KeyCode::L,
        "m" | "M" => KeyCode::M,
        "n" | "N" => KeyCode::N,
        "o" | "O" => KeyCode::O,
        "p" | "P" => KeyCode::P,
        "q" | "Q" => KeyCode::Q,
        "r" | "R" => KeyCode::R,
        "s" | "S" => KeyCode::S,
        "t" | "T" => KeyCode::T,
        "u" | "U" => KeyCode::U,
        "v" | "V" => KeyCode::V,
        "w" | "W" => KeyCode::W,
        "x" | "X" => KeyCode::X,
        "y" | "Y" => KeyCode::Y,
        "z" | "Z" => KeyCode::Z,
        "0" => KeyCode::Num0,
        "1" => KeyCode::Num1,
        "2" => KeyCode::Num2,
        "3" => KeyCode::Num3,
        "4" => KeyCode::Num4,
        "5" => KeyCode::Num5,
        "6" => KeyCode::Num6,
        "7" => KeyCode::Num7,
        "8" => KeyCode::Num8,
        "9" => KeyCode::Num9,
        "Enter" => KeyCode::Enter,
        "Escape" => KeyCode::Escape,
        "Backspace" => KeyCode::Backspace,
        "Tab" => KeyCode::Tab,
        " " => KeyCode::Space,
        "ArrowLeft" => KeyCode::Left,
        "ArrowRight" => KeyCode::Right,
        "ArrowUp" => KeyCode::Up,
        "ArrowDown" => KeyCode::Down,
        "Shift" => KeyCode::Shift,
        "Control" => KeyCode::Control,
        "Alt" => KeyCode::Alt,
        "Meta" => KeyCode::Meta,
        _ => KeyCode::Unknown,
    }
}

#[cfg(target_arch = "wasm32")]
fn setup_resize_listener<F>(
    canvas: &HtmlCanvasElement,
    dpr: f32,
    state: Rc<RefCell<AppState<F>>>,
    _handle: frame_rendering::surface::SurfaceHandle,
) where
    F: Fn() -> Box<dyn Widget> + 'static,
{
    let canvas = canvas.clone();
    let closure = Closure::wrap(Box::new(move || {
        let win = web_sys::window().expect("no window");
        let css_w = win.inner_width().expect("inner_width").as_f64().expect("not f64") as f32;
        let css_h = win.inner_height().expect("inner_height").as_f64().expect("not f64") as f32;
        let physical_w = (css_w * dpr) as u32;
        let physical_h = (css_h * dpr) as u32;
        canvas.set_width(physical_w);
        canvas.set_height(physical_h);
        canvas
            .style()
            .set_property("width", &format!("{}px", css_w as u32))
            .expect("failed to set width");
        canvas
            .style()
            .set_property("height", &format!("{}px", css_h as u32))
            .expect("failed to set height");

        let mut s = state.borrow_mut();
        s.frame.resize(s.window_id, frame_core::Size::new(css_w, css_h));
    }) as Box<dyn Fn()>);

    web_sys::window()
        .expect("no window")
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .expect("failed to add resize listener");
    closure.forget();
}

#[cfg(target_arch = "wasm32")]
fn start_render_loop<F>(state: Rc<RefCell<AppState<F>>>)
where
    F: Fn() -> Box<dyn Widget> + 'static,
{
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut s = state.borrow_mut();
        let id = s.window_id;

        if s.frame.tick(s.root.as_mut()) {
            let _ = s.frame.render(id, s.root.as_mut());
        }

        web_sys::window()
            .unwrap()
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut()>));

    web_sys::window()
        .unwrap()
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
}

#[cfg(target_arch = "wasm32")]
fn setup_destroy_handler<F>(
    frame_app: &RefCell<frame_core::FrameApp>,
    state: Rc<RefCell<AppState<F>>>,
) where
    F: Fn() -> Box<dyn Widget> + 'static,
{
    let frame_app = frame_app.clone();
    let closure = Closure::once(Box::new(move || {
        state.borrow_mut().frame.mark_dirty();
        frame_app.borrow_mut().destroy();
    }) as Box<dyn FnOnce()>);

    let _ = web_sys::window()
        .expect("no window")
        .add_event_listener_with_callback("beforeunload", closure.as_ref().unchecked_ref());
    closure.forget();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_app<F>(_builder: AppBuilder, _root_factory: F)
where
    F: Fn() -> Box<dyn frame_core::traits::widget::Widget> + 'static,
{
}
