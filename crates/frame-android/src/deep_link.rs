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
pub fn extract_intent_uri(
    vm: &jni::JavaVM,
    activity: &jni::objects::JObject,
) -> Option<String> {
    let mut env = vm.attach_current_thread().ok()?;

    let intent = env
        .call_method(activity, "getIntent", "()Landroid/content/Intent;", &[])
        .ok()?
        .l()
        .ok()?;
    if intent.is_null() {
        return None;
    }

    let action_obj = env
        .call_method(&intent, "getAction", "()Ljava/lang/String;", &[])
        .ok()?
        .l()
        .ok()?;
    if action_obj.is_null() {
        return None;
    }
    let action_str: String = env.get_string(&action_obj.into()).ok()?.into();
    if action_str != "android.intent.action.VIEW" {
        return None;
    }

    let uri = env
        .call_method(&intent, "getData", "()Landroid/net/Uri;", &[])
        .ok()?
        .l()
        .ok()?;
    if uri.is_null() {
        return None;
    }

    let str_obj = env
        .call_method(&uri, "toString", "()Ljava/lang/String;", &[])
        .ok()?
        .l()
        .ok()?;
    if str_obj.is_null() {
        return None;
    }

    let uri_string: String = env.get_string(&str_obj.into()).ok()?.into();
    Some(uri_string)
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
    // At runtime, the initial intent that launched the activity is extracted
    // in the entrypoint via `extract_intent_uri()` using JNI. For subsequent
    // intents delivered via onNewIntent, the android-activity crate (v0.6)
    // does not expose a MainEvent::NewIntent variant. To handle those, either:
    //   1. Upgrade to a newer version of android-activity that exposes it, or
    //   2. Poll the current intent periodically via JNI (not recommended), or
    //   3. Use a custom NativeActivity subclass that forwards onNewIntent
    //      through the NDK callback mechanism.
    //
    // The domains parameter is used by `cargo-frame generate-association-files`
    // to populate the manifest; no runtime OS registration is needed here.
}

#[cfg(not(target_os = "android"))]
pub fn register_app_links(_domains: &[String]) {}
