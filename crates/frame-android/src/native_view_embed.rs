use frame_core::{NativeViewHandle, NativeViewRequest, Rect};

#[cfg(target_os = "android")]
use frame_core::NativeViewKind;
#[cfg(target_os = "android")]
use std::collections::HashMap;

#[cfg(target_os = "android")]
struct ManagedView {
    rect: Rect,
    kind: NativeViewKind,
    java_view: Option<jni::objects::GlobalRef>,
}

#[cfg(target_os = "android")]
struct NativeViewManagerInner {
    next_id: u64,
    views: HashMap<u64, ManagedView>,
    vm_raw: *mut jni::sys::JavaVM,
    activity_raw: jni::sys::jobject,
}

#[cfg(target_os = "android")]
mod android_impl {
    use super::*;
    use std::ptr;
    use std::sync::{Arc, Mutex, OnceLock};

    static NATIVE_VIEW_MANAGER: OnceLock<Arc<Mutex<super::NativeViewManagerInner>>> =
        OnceLock::new();

    pub(super) fn get_or_init_inner() -> Arc<Mutex<super::NativeViewManagerInner>> {
        NATIVE_VIEW_MANAGER
            .get_or_init(|| {
                Arc::new(Mutex::new(super::NativeViewManagerInner {
                    next_id: 1,
                    views: HashMap::new(),
                    vm_raw: ptr::null_mut(),
                    activity_raw: ptr::null_mut(),
                }))
            })
            .clone()
    }

    pub(super) fn set_jni_context(
        vm_raw: *mut jni::sys::JavaVM,
        activity_raw: jni::sys::jobject,
    ) {
        let inner = get_or_init_inner();
        inner.lock().unwrap().vm_raw = vm_raw;
        inner.lock().unwrap().activity_raw = activity_raw;
    }

    pub(super) fn embed_inner(request: NativeViewRequest) -> NativeViewHandle {
        let inner = get_or_init_inner();
        let mut guard = inner.lock().unwrap();
        let id = guard.next_id;
        guard.next_id += 1;

        let java_view = if !guard.vm_raw.is_null() && !guard.activity_raw.is_null() {
            let vm =
                unsafe { jni::JavaVM::from_raw(guard.vm_raw).ok() };
            let activity_raw = guard.activity_raw;
            match vm {
                Some(ref vm) => {
                    let activity = unsafe { jni::objects::JObject::from_raw(activity_raw) };
                    match create_android_view(vm, &activity, &request) {
                        Ok(gref) => gref,
                        Err(e) => {
                            eprintln!("frame-android: JNI error creating view: {e}");
                            None
                        }
                    }
                }
                None => {
                    eprintln!("frame-android: failed to create JavaVM from raw ptr");
                    None
                }
            }
        } else {
            eprintln!(
                "frame-android: no JNI context, view {id} tracked without Java peer"
            );
            None
        };

        guard.views.insert(
            id,
            ManagedView {
                rect: request.rect,
                kind: request.kind,
                java_view,
            },
        );
        NativeViewHandle { id }
    }

    pub(super) fn update_rect_inner(handle: &NativeViewHandle, rect: Rect) {
        let inner = get_or_init_inner();
        let mut guard = inner.lock().unwrap();
        if let Some(view) = guard.views.get_mut(&handle.id) {
            if let Some(ref gref) = view.java_view {
                if let Some(view_obj) = gref.as_obj() {
                    if !guard.vm_raw.is_null() {
                        if let Ok(vm) =
                            unsafe { jni::JavaVM::from_raw(guard.vm_raw) }
                        {
                            if let Err(e) = update_view_layout(&vm, &view_obj, &rect) {
                                eprintln!("frame-android: failed to update view layout: {e}");
                            }
                        }
                    }
                }
            }
            view.rect = rect;
        }
    }

    pub(super) fn remove_inner(handle: &NativeViewHandle) {
        let inner = get_or_init_inner();
        let mut guard = inner.lock().unwrap();
        if let Some(view) = guard.views.remove(&handle.id) {
            if let Some(gref) = view.java_view {
                if let Some(view_obj) = gref.as_obj() {
                    if !guard.vm_raw.is_null() {
                        if let Ok(vm) =
                            unsafe { jni::JavaVM::from_raw(guard.vm_raw) }
                        {
                            if let Err(e) = remove_view_from_parent(&vm, &view_obj) {
                                eprintln!("frame-android: failed to remove view: {e}");
                            }
                        }
                    }
                }
            }
        }
    }

    fn create_android_view(
        vm: &jni::JavaVM,
        activity: &jni::objects::JObject,
        request: &NativeViewRequest,
    ) -> Result<Option<jni::objects::GlobalRef>, jni::errors::Error> {
        let mut env = vm.attach_current_thread()?;

        let view_class_name = match &request.kind {
            NativeViewKind::Web => "android/webkit/WebView",
            NativeViewKind::Map => "android/widget/FrameLayout",
            NativeViewKind::Video => "android/widget/FrameLayout",
            NativeViewKind::Custom(name) => {
                eprintln!(
                    "frame-android: custom native view kind '{name}' not supported, using FrameLayout"
                );
                "android/widget/FrameLayout"
            }
        };

        let view_class = env.find_class(view_class_name)?;
        if view_class.is_null() {
            eprintln!("frame-android: could not find class {view_class_name}");
            return Ok(None);
        }

        let view_obj = env.new_object(
            &view_class,
            "(Landroid/content/Context;)V",
            &[jni::objects::JValue::Object(activity)],
        )?;

        let lp_class = env.find_class("android/widget/FrameLayout$LayoutParams")?;
        let x = request.rect.x() as jni::sys::jint;
        let y = request.rect.y() as jni::sys::jint;
        let w = request.rect.width() as jni::sys::jint;
        let h = request.rect.height() as jni::sys::jint;

        let lp_obj = env.new_object(
            &lp_class,
            "(IIII)V",
            &[
                jni::objects::JValue::Int(w),
                jni::objects::JValue::Int(h),
                jni::objects::JValue::Int(x),
                jni::objects::JValue::Int(y),
            ],
        )?;

        let window = env
            .call_method(activity, "getWindow", "()Landroid/view/Window;", &[])?
            .l()?;

        let decor = env
            .call_method(&window, "getDecorView", "()Landroid/view/View;", &[])?
            .l()?;

        let view_group_class = env.find_class("android/view/ViewGroup")?;
        let add_view_id = env.get_method_id(
            &view_group_class,
            "addView",
            "(Landroid/view/View;Landroid/view/ViewGroup$LayoutParams;)V",
        )?;

        unsafe {
            env.call_method_unchecked(
                &decor,
                add_view_id,
                jni::signature::ReturnType::Primitive(jni::signature::Primitive::Void),
                &[
                    jni::objects::JValue::Object(&view_obj).as_jni(),
                    jni::objects::JValue::Object(&lp_obj).as_jni(),
                ],
            )?;
        }

        let global_ref = env.new_global_ref(&view_obj)?;
        Ok(Some(global_ref))
    }

    fn update_view_layout(
        vm: &jni::JavaVM,
        view: &jni::objects::JObject,
        rect: &Rect,
    ) -> Result<(), jni::errors::Error> {
        let mut env = vm.attach_current_thread()?;

        let lp_class = env.find_class("android/widget/FrameLayout$LayoutParams")?;
        let x = rect.x() as jni::sys::jint;
        let y = rect.y() as jni::sys::jint;
        let w = rect.width() as jni::sys::jint;
        let h = rect.height() as jni::sys::jint;

        let lp_obj = env.new_object(
            &lp_class,
            "(IIII)V",
            &[
                jni::objects::JValue::Int(w),
                jni::objects::JValue::Int(h),
                jni::objects::JValue::Int(x),
                jni::objects::JValue::Int(y),
            ],
        )?;

        env.call_method(
            view,
            "setLayoutParams",
            "(Landroid/view/ViewGroup$LayoutParams;)V",
            &[jni::objects::JValue::Object(&lp_obj)],
        )?;

        Ok(())
    }

    fn remove_view_from_parent(
        vm: &jni::JavaVM,
        view: &jni::objects::JObject,
    ) -> Result<(), jni::errors::Error> {
        let mut env = vm.attach_current_thread()?;

        let parent = env
            .call_method(view, "getParent", "()Landroid/view/ViewParent;", &[])?
            .l()?;

        if parent.is_null() {
            return Ok(());
        }

        let view_group_class = env.find_class("android/view/ViewGroup")?;
        let remove_id =
            env.get_method_id(&view_group_class, "removeView", "(Landroid/view/View;)V")?;

        unsafe {
            env.call_method_unchecked(
                &parent,
                remove_id,
                jni::signature::ReturnType::Primitive(jni::signature::Primitive::Void),
                &[jni::objects::JValue::Object(view).as_jni()],
            )?;
        }

        Ok(())
    }
}

#[cfg(target_os = "android")]
pub use android_impl::set_jni_context;

pub struct NativeViewManager {
    #[cfg(target_os = "android")]
    inner: std::sync::Arc<std::sync::Mutex<NativeViewManagerInner>>,
    #[cfg(not(target_os = "android"))]
    next_id: u64,
}

impl NativeViewManager {
    pub fn new() -> Self {
        Self {
            #[cfg(target_os = "android")]
            inner: android_impl::get_or_init_inner(),
            #[cfg(not(target_os = "android"))]
            next_id: 1,
        }
    }

    pub fn embed(&mut self, request: NativeViewRequest) -> NativeViewHandle {
        #[cfg(target_os = "android")]
        {
            android_impl::embed_inner(request)
        }
        #[cfg(not(target_os = "android"))]
        {
            let id = self.next_id;
            self.next_id += 1;
            let _ = request;
            NativeViewHandle { id }
        }
    }

    pub fn update_rect(&mut self, handle: &NativeViewHandle, rect: Rect) {
        #[cfg(target_os = "android")]
        {
            android_impl::update_rect_inner(handle, rect);
        }
        #[cfg(not(target_os = "android"))]
        {
            let _ = (handle, rect);
        }
    }

    pub fn remove(&mut self, handle: &NativeViewHandle) {
        #[cfg(target_os = "android")]
        {
            android_impl::remove_inner(handle);
        }
        #[cfg(not(target_os = "android"))]
        {
            let _ = handle;
        }
    }
}

impl Default for NativeViewManager {
    fn default() -> Self {
        Self::new()
    }
}
