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

#[cfg(target_os = "windows")]
struct WindowState {
    frame: frame_rendering::framecoord::Frame<crate::wgpu_surface::WgpuSurfaceProvider>,
    window_id: frame_core::WindowId,
    scale_factor: f32,
}

#[cfg(target_os = "windows")]
pub fn run_app<F>(builder: AppBuilder, root_factory: F)
where
    F: Fn() -> Box<dyn frame_core::traits::widget::Widget> + 'static,
{
    use crate::wgpu_surface::WgpuSurfaceProvider;
    use frame_core::{Size, WindowId};
    use frame_rendering::framecoord::Frame;
    use std::rc::Rc;
    use windows::Win32::Foundation::*;
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows::Win32::UI::HiDpi::GetDpiForWindow;
    use windows::Win32::UI::WindowsAndMessaging::*;

    crate::deep_link::register_protocol_scheme("");

    let mut frame_app_builder = frame_core::FrameApp::builder();
    for plugin in builder.plugins {
        frame_app_builder = frame_app_builder.plugin(plugin);
    }
    let mut frame_app = frame_app_builder.build().expect("failed to build FrameApp");

    let class_name: Vec<u16> = "FrameWindowClass\0".encode_utf16().collect();
    let wide_title: Vec<u16> = builder.title.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        let h_instance = GetModuleHandleW(None).expect("GetModuleHandleW failed");

        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: h_instance.into(),
            lpszClassName: windows::core::PCWSTR(class_name.as_ptr()),
            style: CS_HREDRAW | CS_VREDRAW,
            ..Default::default()
        };

        let _ = RegisterClassW(&wnd_class);

        let mut style = WINDOW_STYLE::WS_OVERLAPPEDWINDOW;
        if !builder.resizable {
            style &= !(WS_THICKFRAME | WS_MAXIMIZEBOX);
        }

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            windows::core::PCWSTR(class_name.as_ptr()),
            windows::core::PCWSTR(wide_title.as_ptr()),
            style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            builder.width as i32,
            builder.height as i32,
            HWND::default(),
            HMENU::default(),
            h_instance.into(),
            None,
        );

        if hwnd.is_invalid() {
            panic!("CreateWindowExW failed");
        }

        let _ = ShowWindow(hwnd, SW_SHOW);
        let _ = UpdateWindow(hwnd);

        let surface_provider = WgpuSurfaceProvider::new();
        let mut frame = Frame::new(surface_provider);

        let dpi = GetDpiForWindow(hwnd);
        let scale = dpi as f32 / 96.0;
        frame.set_scale_factor(scale);

        let window_size = Size::new(builder.width, builder.height);
        let window_id = WindowId::default();

        frame
            .create_surface(window_id, window_size)
            .expect("failed to create surface");

        let physical_w = (builder.width * scale) as u32;
        let physical_h = (builder.height * scale) as u32;
        frame
            .surface_provider_mut()
            .init_surface(
                window_id,
                hwnd.0 as *mut std::ffi::c_void,
                physical_w,
                physical_h,
            )
            .expect("failed to init wgpu surface");

        let state = Box::new(WindowState {
            frame,
            window_id,
            scale_factor: scale,
        });
        let state_ptr = Box::into_raw(state);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, state_ptr as isize);

        frame_app.init();

        let factory: Rc<dyn Fn() -> Box<dyn frame_core::traits::widget::Widget>> = Rc::new(root_factory);

        let mut msg = MSG::default();
        loop {
            while PeekMessage(&mut msg, HWND::default(), 0, 0, PM_REMOVE).as_bool() {
                let _ = TranslateMessage(&msg);
                let _ = DispatchMessage(&msg);
                if msg.message == WM_QUIT {
                    frame_app.destroy();
                    return;
                }
            }

            let state = &mut *state_ptr;
            let mut root = factory();
            state.frame.tick(root.as_mut());
            let _ = state.frame.render(state.window_id, root.as_mut());
        }
    }
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
            if !ptr.is_null() {
                let _ = Box::from_raw(ptr as *mut WindowState);
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
            }
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let _ = BeginPaint(hwnd, &mut ps);
            let _ = EndPaint(hwnd, &ps);
            ValidateRect(hwnd, None);
            LRESULT(0)
        }
        WM_SIZE => {
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
            if !ptr.is_null() {
                let state = &mut *(ptr as *mut WindowState);
                let width = (lparam.0 & 0xFFFF) as u32 as f32;
                let height = ((lparam.0 >> 16) & 0xFFFF) as u32 as f32;
                state.frame.resize(state.window_id, frame_core::Size::new(width, height));
            }
            LRESULT(0)
        }
        WM_ERASEBKGND => {
            LRESULT(1)
        }
        WM_LBUTTONDOWN => {
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
            if !ptr.is_null() {
                let state = &mut *(ptr as *mut WindowState);
                let x = (lparam.0 & 0xFFFF) as f32;
                let y = ((lparam.0 >> 16) & 0xFFFF) as f32;
                let point = frame_core::Point::new(x / state.scale_factor, y / state.scale_factor);
                frame_rendering::click::dispatch_click(point);
            }
            LRESULT(0)
        }
        WM_LBUTTONUP => {
            LRESULT(0)
        }
        WM_MOUSEMOVE => {
            LRESULT(0)
        }
        WM_KEYDOWN | WM_KEYUP => {
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

#[cfg(not(target_os = "windows"))]
pub fn run_app<F>(_builder: AppBuilder, _root_factory: F)
where
    F: Fn() -> Box<dyn frame_core::traits::widget::Widget> + 'static,
{
}
