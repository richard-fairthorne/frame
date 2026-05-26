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
pub fn register_universal_links(domains: &[String]) {
    let _ = domains;

    // iOS universal links and custom URL schemes are registered at build time
    // in the Info.plist. The `cargo-frame generate-association-files` command
    // generates the required entries.
    //
    // For universal links, add to Info.plist:
    //   <key>com.apple.developer.associated-domains</key>
    //   <array>
    //     <string>applinks:example.com</string>
    //   </array>
    //
    // For custom URL schemes:
    //   <key>CFBundleURLTypes</key>
    //   <array>
    //     <dict>
    //       <key>CFBundleURLSchemes</key>
    //       <array>
    //         <string>myapp</string>
    //       </array>
    //     </dict>
    //   </array>
    //
    // At runtime, universal links are delivered via the UIApplicationDelegate
    // method `application:continueUserActivity:restorationHandler:`. Custom
    // URL schemes arrive via `application:openURL:options:`.
    //
    // To receive these events in Rust, the framework must hook into the UIKit
    // delegate chain — either by subclassing UIApplicationDelegate or by
    // swizzling the delegate methods. This requires careful ObjC runtime work
    // and is deferred to a future implementation.
    //
    // The domains parameter is used by `cargo-frame generate-association-files`
    // to populate Info.plist; no runtime OS registration is needed here.
}

#[cfg(not(target_os = "ios"))]
pub fn register_universal_links(_domains: &[String]) {}
