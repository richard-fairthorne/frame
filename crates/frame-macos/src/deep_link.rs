use std::sync::RwLock;
use std::sync::LazyLock;

type DeepLinkHandler = Box<dyn Fn(String) + Send + Sync>;

static DEEP_LINK_HANDLER: LazyLock<RwLock<Option<DeepLinkHandler>>> =
    LazyLock::new(|| RwLock::new(None));

pub fn set_deep_link_handler(handler: DeepLinkHandler) {
    *DEEP_LINK_HANDLER.write().unwrap() = Some(handler);
}

pub fn handle_deep_link(url: String) {
    if let Some(ref handler) = *DEEP_LINK_HANDLER.read().unwrap() {
        handler(url);
    }
}

#[cfg(target_os = "macos")]
pub fn register_universal_links(domains: &[String]) {
    let _ = domains;

    unsafe {
        install_url_event_handler();
    }
}

#[cfg(target_os = "macos")]
unsafe fn install_url_event_handler() {
    use objc2::runtime::{AnyClass, AnyObject, ClassBuilder, Sel};
    use objc2_foundation::NSAppleEventManager;

    let handler_class = match AnyClass::get(c"FrameDeepLinkHandler") {
        Some(cls) => cls,
        None => {
            let superclass = AnyClass::get(c"NSObject").unwrap();
            let mut builder = match ClassBuilder::new(c"FrameDeepLinkHandler", superclass) {
                Some(b) => b,
                None => return,
            };
            builder.add_method(
                Sel::register(c"handleURLEvent:withReplyEvent:"),
                url_event_handler as unsafe extern "C" fn(_, _, _, _),
            );
            builder.register()
        }
    };

    let handler: *mut AnyObject = objc2::msg_send![handler_class, new];
    if handler.is_null() {
        return;
    }

    let manager = NSAppleEventManager::sharedAppleEventManager();

    let event_class: u32 = 0x4755524C; // kInternetEventClass ('GURL')
    let event_id: u32 = 0x4755524C;    // kAEGetURL ('GURL')

    let _: () = objc2::msg_send![
        &manager,
        setEventHandler: handler,
        andSelector: Sel::register(c"handleURLEvent:withReplyEvent:"),
        forEventClass: event_class,
        andEventID: event_id
    ];

    let _: () = objc2::msg_send![handler, release];
}

#[cfg(target_os = "macos")]
unsafe extern "C" fn url_event_handler(
    _this: &objc2::runtime::AnyObject,
    _cmd: objc2::runtime::Sel,
    event: &objc2::runtime::AnyObject,
    _reply: &objc2::runtime::AnyObject,
) {
    let descriptor: *mut objc2::runtime::AnyObject = objc2::msg_send![
        event,
        paramDescriptorForKeyword: 0x2D2D2D2Du32 // keyDirectObject ('----')
    ];
    if descriptor.is_null() {
        return;
    }

    let url: *mut objc2::runtime::AnyObject = objc2::msg_send![descriptor, stringValue];
    if url.is_null() {
        let _: () = objc2::msg_send![descriptor, release];
        return;
    }

    let url_str: *mut std::ffi::c_char = objc2::msg_send![url, UTF8String];
    if !url_str.is_null() {
        if let Ok(s) = std::ffi::CStr::from_ptr(url_str).to_str() {
            handle_deep_link(s.to_string());
        }
    }

    let _: () = objc2::msg_send![url, release];
    let _: () = objc2::msg_send![descriptor, release];
}

#[cfg(not(target_os = "macos"))]
pub fn register_universal_links(_domains: &[String]) {}
