use frame_core::{NativeViewHandle, NativeViewKind, NativeViewRequest, Rect};
use std::collections::HashMap;

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HWND;

struct ManagedView {
    #[cfg(target_os = "windows")]
    child_hwnd: Option<isize>,
    rect: Rect,
    kind: NativeViewKind,
}

#[cfg(target_os = "windows")]
unsafe impl Send for ManagedView {}
#[cfg(target_os = "windows")]
unsafe impl Sync for ManagedView {}

pub struct NativeViewManager {
    next_id: u64,
    views: HashMap<u64, ManagedView>,
    #[cfg(target_os = "windows")]
    parent_hwnd: Option<isize>,
}

impl NativeViewManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            views: HashMap::new(),
            #[cfg(target_os = "windows")]
            parent_hwnd: None,
        }
    }

    #[cfg(target_os = "windows")]
    pub fn set_parent_hwnd(&mut self, hwnd: isize) {
        self.parent_hwnd = Some(hwnd);
    }

    pub fn embed(&mut self, request: NativeViewRequest) -> NativeViewHandle {
        let id = self.next_id;
        self.next_id += 1;

        #[cfg(target_os = "windows")]
        {
            let child_hwnd = self.parent_hwnd.and_then(|parent| {
                create_child_hwnd(&request, HWND(parent))
            });

            self.views.insert(
                id,
                ManagedView {
                    child_hwnd,
                    rect: request.rect,
                    kind: request.kind,
                },
            );
        }

        #[cfg(not(target_os = "windows"))]
        {
            self.views.insert(
                id,
                ManagedView {
                    rect: request.rect,
                    kind: request.kind,
                },
            );
        }

        NativeViewHandle { id }
    }

    pub fn update_rect(&mut self, handle: &NativeViewHandle, rect: Rect) {
        if let Some(view) = self.views.get_mut(&handle.id) {
            view.rect = rect;

            #[cfg(target_os = "windows")]
            if let Some(child_hwnd) = view.child_hwnd {
                unsafe {
                    use windows::Win32::UI::WindowsAndMessaging::{
                        SetWindowPos, HWND, SWP_NOZORDER,
                    };
                    let _ = SetWindowPos(
                        HWND(child_hwnd),
                        HWND::default(),
                        rect.origin.x as i32,
                        rect.origin.y as i32,
                        rect.size.width as i32,
                        rect.size.height as i32,
                        SWP_NOZORDER,
                    );
                }
            }
        }
    }

    pub fn remove(&mut self, handle: &NativeViewHandle) {
        if let Some(view) = self.views.remove(&handle.id) {
            #[cfg(target_os = "windows")]
            if let Some(child_hwnd) = view.child_hwnd {
                unsafe {
                    use windows::Win32::UI::WindowsAndMessaging::DestroyWindow;
                    let _ = DestroyWindow(HWND(child_hwnd));
                }
            }
        }
    }
}

impl Default for NativeViewManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "windows")]
fn create_child_hwnd(request: &NativeViewRequest, parent: HWND) -> Option<isize> {
    use windows::core::PCWSTR;
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows::Win32::UI::WindowsAndMessaging::*;

    unsafe {
        let h_instance = GetModuleHandleW(None).ok()?;

        let class_name: Vec<u16> = "FrameNativeViewClass\0".encode_utf16().collect();

        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(DefWindowProcW),
            hInstance: h_instance.into(),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            style: CS_HREDRAW | CS_VREDRAW,
            ..Default::default()
        };

        let atom = RegisterClassW(&wnd_class);
        if atom == 0 {
            let _ = RegisterClassW(&wnd_class);
        }

        let style = WINDOW_STYLE::WS_CHILD | WINDOW_STYLE::WS_VISIBLE | WINDOW_STYLE::WS_CLIPCHILDREN;

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            PCWSTR(class_name.as_ptr()),
            PCWSTR::null(),
            style,
            request.rect.origin.x as i32,
            request.rect.origin.y as i32,
            request.rect.size.width as i32,
            request.rect.size.height as i32,
            parent,
            HMENU::default(),
            h_instance.into(),
            None,
        );

        if hwnd.is_invalid() {
            return None;
        }

        match &request.kind {
            NativeViewKind::Web => {
                use windows::Win32::Graphics::Gdi::{CreateSolidBrush, DeleteObject};
                let brush = CreateSolidBrush(0x00000000);
                let _ = SetClassLongPtrW(hwnd, GCLP_HBRBACKGROUND, brush.0 as isize);
            }
            _ => {}
        }

        Some(hwnd.0)
    }
}
