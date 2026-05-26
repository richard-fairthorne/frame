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

#[cfg(target_os = "linux")]
pub fn register_uri_scheme(scheme: &str) {
    let _ = scheme;

    // Linux URI scheme associations are registered via a .desktop file installed
    // to ~/.local/share/applications/ (per-user) or /usr/share/applications/.
    // The `cargo-frame generate-association-files` command creates the .desktop
    // file with the required MimeType entry.
    //
    // Example .desktop file:
    //   [Desktop Entry]
    //   Type=Application
    //   Name=My Frame App
    //   Exec=/path/to/app %U
    //   MimeType=x-scheme-handler/myapp;
    //   NoDisplay=true
    //
    // Then register with:
    //   $ update-desktop-database ~/.local/share/applications/
    //   $ xdg-mime default myapp.desktop x-scheme-handler/myapp
    //
    // When the OS launches the app via this scheme, the URL is passed as a
    // command-line argument. The framework entrypoint should check args[1]
    // for a URL and call handle_deep_link(url_string).
    //
    // The scheme parameter is used by `cargo-frame generate-association-files`
    // to populate the .desktop file; runtime registration is not performed here
    // because it requires desktop-environment-specific tooling (xdg-mime,
    // update-desktop-database) that may not be available.
}

#[cfg(not(target_os = "linux"))]
pub fn register_uri_scheme(_scheme: &str) {}
