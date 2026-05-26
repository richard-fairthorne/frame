use frame_core::plugin::{Plugin, PluginContext};
use frame_core::traits::renderer::WindowHandle;
use frame_core::traits::window::{WindowConfig, WindowHost};
use frame_core::{Size, WindowId};
use slotmap::SlotMap;

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::*;
#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Gdi::{UpdateWindow, GetDC, ReleaseDC};

struct WindowsWindow {
    hwnd: Option<isize>,
    title: String,
    size: Size,
}

unsafe impl Send for WindowsWindow {}
unsafe impl Sync for WindowsWindow {}

pub struct WindowsWindowManager {
    windows: SlotMap<WindowId, WindowsWindow>,
}

impl WindowsWindowManager {
    pub fn new() -> Self {
        Self {
            windows: SlotMap::with_key(),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn hwnd(&self, handle: &WindowHandle) -> Option<HWND> {
        self.windows
            .get(handle.id)
            .and_then(|w| w.hwnd)
            .map(|h| HWND(h))
    }

    #[cfg(not(target_os = "windows"))]
    pub fn hwnd(&self, _handle: &WindowHandle) -> Option<isize> {
        None
    }
}

impl Default for WindowsWindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for WindowsWindowManager {
    fn init(&mut self, _ctx: &mut PluginContext<'_>) {}

    fn on_pause(&mut self) {}

    fn on_resume(&mut self) {}

    fn on_destroy(&mut self) {
        #[cfg(target_os = "windows")]
        {
            for (_, window) in self.windows.iter() {
                if let Some(hwnd) = window.hwnd {
                    unsafe {
                        let _ = DestroyWindow(HWND(hwnd));
                    }
                }
            }
        }
        self.windows.clear();
    }
}

impl WindowHost for WindowsWindowManager {
    fn create_window(&mut self, config: WindowConfig) -> WindowHandle {
        let hwnd = create_win32_window(
            &config.title,
            config.size.width,
            config.size.height,
            config.resizable,
        );

        let id = self.windows.insert(WindowsWindow {
            hwnd,
            title: config.title,
            size: config.size,
        });
        WindowHandle::new(id)
    }

    fn set_title(&mut self, handle: WindowHandle, title: &str) {
        if let Some(window) = self.windows.get_mut(handle.id) {
            window.title = title.to_string();
            #[cfg(target_os = "windows")]
            if let Some(hwnd) = window.hwnd {
                use windows::core::PCWSTR;
                let wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
                unsafe {
                    let _ = SetWindowTextW(HWND(hwnd), PCWSTR(wide.as_ptr()));
                }
            }
        }
    }

    fn resize(&mut self, handle: WindowHandle, size: Size) {
        if let Some(window) = self.windows.get_mut(handle.id) {
            window.size = size;
            #[cfg(target_os = "windows")]
            if let Some(hwnd) = window.hwnd {
                use windows::Win32::Foundation::RECT;
                use windows::Win32::UI::WindowsAndMessaging::AdjustWindowRect;
                unsafe {
                    let mut rect = RECT {
                        left: 0,
                        top: 0,
                        right: size.width as i32,
                        bottom: size.height as i32,
                    };
                    let _ = AdjustWindowRect(&mut rect, WINDOW_STYLE::default(), false);
                    let _ = SetWindowPos(
                        HWND(hwnd),
                        HWND::default(),
                        0,
                        0,
                        rect.right - rect.left,
                        rect.bottom - rect.top,
                        SWP_NOMOVE | SWP_NOZORDER,
                    );
                }
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn create_win32_window(title: &str, width: f32, height: f32, resizable: bool) -> Option<isize> {
    use windows::core::PCWSTR;
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;

    let class_name: Vec<u16> = "FrameWindowClass\0".encode_utf16().collect();
    let wide_title: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        let h_instance = GetModuleHandleW(None).ok()?;

        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(def_window_proc),
            hInstance: h_instance.into(),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            style: CS_HREDRAW | CS_VREDRAW,
            ..Default::default()
        };

        let atom = RegisterClassW(&wnd_class);
        if atom == 0 {
            let _ = RegisterClassW(&wnd_class);
        }

        let mut style = WINDOW_STYLE::WS_OVERLAPPEDWINDOW;
        if !resizable {
            style &= !(WS_THICKFRAME | WS_MAXIMIZEBOX);
        }

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            PCWSTR(class_name.as_ptr()),
            PCWSTR(wide_title.as_ptr()),
            style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width as i32,
            height as i32,
            HWND::default(),
            HMENU::default(),
            h_instance.into(),
            None,
        );

        if hwnd.is_invalid() {
            return None;
        }

        let _ = ShowWindow(hwnd, SW_SHOW);
        let _ = UpdateWindow(hwnd);

        Some(hwnd.0)
    }
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn def_window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            let _ = EndPaint(hwnd, &ps);
            if hdc.is_invalid() {
                LRESULT(0)
            } else {
                LRESULT(0)
            }
        }
        WM_SIZE => {
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

#[cfg(not(target_os = "windows"))]
fn create_win32_window(_title: &str, _width: f32, _height: f32, _resizable: bool) -> Option<isize> {
    None
}
