use std::sync::RwLock;
use std::sync::LazyLock;

static DEEP_LINK_HANDLER: LazyLock<RwLock<Option<Box<dyn Fn(String) + Send + Sync>>>> =
    LazyLock::new(|| RwLock::new(None));

pub fn set_deep_link_handler(handler: Box<dyn Fn(String) + Send + Sync>) {
    *DEEP_LINK_HANDLER.write().unwrap() = Some(handler);
}

pub fn handle_deep_link(url: String) {
    if let Some(ref handler) = *DEEP_LINK_HANDLER.read().unwrap() {
        handler(url);
    }
}

#[cfg(target_os = "ios")]
pub fn register_universal_links(_domains: &[String]) {
    unsafe {
        install_app_delegate_methods();
    }
}

#[cfg(target_os = "ios")]
unsafe fn install_app_delegate_methods() {
    use objc2::runtime::{AnyClass, AnyObject, ClassBuilder, Sel};

    if AnyClass::get(c"FrameAppDelegate").is_some() {
        return;
    }

    let superclass = match AnyClass::get(c"UIResponder") {
        Some(cls) => cls,
        None => return,
    };

    let mut builder = match ClassBuilder::new(c"FrameAppDelegate", superclass) {
        Some(b) => b,
        None => return,
    };

    builder.add_method(
        Sel::register(c"application:openURL:options:"),
        open_url as unsafe extern "C" fn(_, _, _, _, _),
    );

    builder.add_method(
        Sel::register(c"application:continueUserActivity:restorationHandler:"),
        continue_user_activity as unsafe extern "C" fn(_, _, _, _, _),
    );

    builder.register();
}

#[cfg(target_os = "ios")]
unsafe extern "C" fn open_url(
    _this: &AnyObject,
    _cmd: objc2::Sel,
    _app: &AnyObject,
    url: &AnyObject,
    _options: &AnyObject,
) -> objc2::runtime::Bool {
    let ns_string: *mut AnyObject = objc2::msg_send![url, absoluteString];
    if !ns_string.is_null() {
        let utf8: *const std::ffi::c_char = objc2::msg_send![ns_string, UTF8String];
        if !utf8.is_null() {
            if let Ok(s) = std::ffi::CStr::from_ptr(utf8).to_str() {
                handle_deep_link(s.to_string());
            }
        }
    }
    objc2::runtime::Bool::new(true)
}

#[cfg(target_os = "ios")]
unsafe extern "C" fn continue_user_activity(
    _this: &AnyObject,
    _cmd: objc2::Sel,
    _app: &AnyObject,
    user_activity: &AnyObject,
    _restoration_handler: &AnyObject,
) -> objc2::runtime::Bool {
    let webpage_url: *mut AnyObject = objc2::msg_send![user_activity, webpageURL];
    if !webpage_url.is_null() {
        let ns_string: *mut AnyObject = objc2::msg_send![webpage_url, absoluteString];
        if !ns_string.is_null() {
            let utf8: *const std::ffi::c_char = objc2::msg_send![ns_string, UTF8String];
            if !utf8.is_null() {
                if let Ok(s) = std::ffi::CStr::from_ptr(utf8).to_str() {
                    handle_deep_link(s.to_string());
                }
            }
        }
    }
    objc2::runtime::Bool::new(true)
}

#[cfg(not(target_os = "ios"))]
pub fn register_universal_links(_domains: &[String]) {}
