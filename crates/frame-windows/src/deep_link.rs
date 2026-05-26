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

#[cfg(target_os = "windows")]
pub fn register_protocol_scheme(scheme: &str) {
    let _ = scheme;

    // Windows custom protocol schemes are registered via the Windows Registry.
    // The `cargo-frame generate-association-files` command creates a .reg file
    // or an installer that writes the required registry keys.
    //
    // Required registry entries (requires elevation / admin for HKLM,
    // or use HKCU for per-user registration):
    //
    //   HKEY_CURRENT_USER\Software\Classes\myapp
    //     (Default) = "URL:My App Protocol"
    //     URL Protocol = ""
    //
    //   HKEY_CURRENT_USER\Software\Classes\myapp\shell\open\command
    //     (Default) = "\"C:\path\to\app.exe\" \"%1\""
    //
    // When the OS launches the app via this scheme, the URL is passed as a
    // command-line argument. The framework entrypoint should check args[1]
    // for a URL and call handle_deep_link(url_string).
    //
    // For development, you can also test by running:
    //   > start myapp://test/path
    //
    // The scheme parameter is used by `cargo-frame generate-association-files`
    // to populate the registry entries; runtime registration would require
    // admin privileges and is not performed here.
}

#[cfg(not(target_os = "windows"))]
pub fn register_protocol_scheme(_scheme: &str) {}
