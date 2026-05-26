#[cfg(target_os = "android")]
use std::ptr::NonNull;
#[cfg(target_os = "android")]
use std::sync::Arc;

#[cfg(target_os = "android")]
pub struct NativeWindow {
    ptr: NonNull<ndk_sys::ANativeWindow>,
}

#[cfg(target_os = "android")]
unsafe impl Send for NativeWindow {}
#[cfg(target_os = "android")]
unsafe impl Sync for NativeWindow {}

#[cfg(target_os = "android")]
impl NativeWindow {
    pub unsafe fn from_ptr(ptr: *mut ndk_sys::ANativeWindow) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    pub fn ptr(&self) -> *mut ndk_sys::ANativeWindow {
        self.ptr.as_ptr()
    }

    pub fn width(&self) -> u32 {
        unsafe { ndk_sys::ANativeWindow_getWidth(self.ptr()) as u32 }
    }

    pub fn height(&self) -> u32 {
        unsafe { ndk_sys::ANativeWindow_getHeight(self.ptr()) as u32 }
    }

    pub fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        use std::ptr::NonNull;
        let handle = raw_window_handle::AndroidNdkWindowHandle::new(
            NonNull::new(self.ptr() as *mut std::ffi::c_void).unwrap(),
        );
        raw_window_handle::RawWindowHandle::AndroidNdk(handle)
    }

    pub fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

#[cfg(target_os = "android")]
impl Drop for NativeWindow {
    fn drop(&mut self) {
        unsafe {
            ndk_sys::ANativeWindow_release(self.ptr());
        }
    }
}
