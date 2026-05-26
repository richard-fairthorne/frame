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

#[cfg(target_os = "android")]
pub fn register_app_links(domains: &[String]) {
    let _ = domains;

    // Android app links and deep links are registered at build time via the
    // AndroidManifest.xml. The `cargo-frame generate-association-files` command
    // generates the required intent-filter entries.
    //
    // For app links (universal links), add to AndroidManifest.xml:
    //   <intent-filter android:autoVerify="true">
    //     <action android:name="android.intent.action.VIEW"/>
    //     <category android:name="android.intent.category.DEFAULT"/>
    //     <category android:name="android.intent.category.BROWSABLE"/>
    //     <data android:scheme="https" android:host="example.com"/>
    //   </intent-filter>
    //
    // For custom URL schemes:
    //   <intent-filter>
    //     <action android:name="android.intent.action.VIEW"/>
    //     <category android:name="android.intent.category.DEFAULT"/>
    //     <category android:name="android.intent.category.BROWSABLE"/>
    //     <data android:scheme="myapp"/>
    //   </intent-filter>
    //
    // At runtime, the NativeActivity receives these intents via its
    // onNewIntent callback. The framework entrypoint must extract the
    // URI from the Intent and call handle_deep_link(uri_string).
    // This requires JNI interop with the Android activity lifecycle.
    //
    // The domains parameter is used by `cargo-frame generate-association-files`
    // to populate the manifest; no runtime OS registration is needed here.
}

#[cfg(not(target_os = "android"))]
pub fn register_app_links(_domains: &[String]) {}
