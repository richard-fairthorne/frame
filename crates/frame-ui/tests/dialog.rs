use std::sync::{Arc, Mutex};

use frame_ui::{Dialog, DialogConfig, DialogDisposition, DialogHost, DialogType, Overlay};

#[test]
fn dialog_alert_config() {
    let config = DialogConfig::alert("Error", "Something went wrong");
    assert_eq!(config.title, "Error");
    assert_eq!(config.dialog_type, DialogType::Alert);
}

#[test]
fn dialog_confirm_config() {
    let config = DialogConfig::confirm("Delete?", "Are you sure?").confirm_label("Delete");
    assert_eq!(config.dialog_type, DialogType::Confirm);
    assert_eq!(config.confirm_label, "Delete");
}

#[test]
fn dialog_prompt_config() {
    let config = DialogConfig::prompt("Name", "Enter name", "John");
    assert_eq!(config.dialog_type, DialogType::Prompt);
    assert_eq!(config.placeholder, "John");
}

#[test]
fn dialog_show_dismiss() {
    let mut dialog = Dialog::new(DialogConfig::alert("Test", "Message"));
    assert!(!dialog.is_visible());
    dialog.show();
    assert!(dialog.is_visible());
    dialog.dismiss(DialogDisposition::Confirmed);
    assert!(!dialog.is_visible());
}

#[test]
fn dialog_callback_fires() {
    let result = Arc::new(Mutex::new(None));
    let r = result.clone();
    let mut dialog = Dialog::new(DialogConfig::confirm("?", "Sure?"))
        .on_result(move |d| *r.lock().unwrap() = Some(d));
    dialog.show();
    dialog.confirm();
    assert_eq!(
        *result.lock().unwrap(),
        Some(DialogDisposition::Confirmed)
    );
}

#[test]
fn dialog_cancel_callback() {
    let result = Arc::new(Mutex::new(None));
    let r = result.clone();
    let mut dialog = Dialog::new(DialogConfig::confirm("?", "?"))
        .on_result(move |d| *r.lock().unwrap() = Some(d));
    dialog.show();
    dialog.cancel();
    assert_eq!(
        *result.lock().unwrap(),
        Some(DialogDisposition::Cancelled)
    );
}

#[test]
fn dialog_host_stack() {
    let mut host = DialogHost::new();
    assert!(!host.is_showing());

    host.show(Dialog::new(DialogConfig::alert("A", "a")));
    host.show(Dialog::new(DialogConfig::alert("B", "b")));
    assert_eq!(host.count(), 2);
    assert_eq!(host.top().unwrap().title(), "B");

    host.dismiss_top(DialogDisposition::Dismissed);
    assert_eq!(host.count(), 1);
    assert_eq!(host.top().unwrap().title(), "A");
}

#[test]
fn dialog_host_dismiss_all() {
    let mut host = DialogHost::new();
    host.show(Dialog::new(DialogConfig::alert("A", "a")));
    host.show(Dialog::new(DialogConfig::alert("B", "b")));
    host.dismiss_all();
    assert!(!host.is_showing());
}

#[test]
fn dialog_prompt_input() {
    let mut dialog = Dialog::new(DialogConfig::prompt("Name", "Enter", "Type here"));
    dialog.show();
    dialog.set_input("Hello World");
    assert_eq!(dialog.input_value(), "Hello World");
}

#[test]
fn overlay_show_dismiss() {
    let mut overlay = Overlay::new().with_opacity(0.7);
    assert!(!overlay.is_visible());
    overlay.show();
    assert!(overlay.is_visible());
    assert_eq!(overlay.opacity(), 0.7);
    overlay.dismiss();
    assert!(!overlay.is_visible());
}

#[test]
fn overlay_dismiss_on_tap() {
    let overlay = Overlay::new().with_dismiss_on_tap(false);
    assert!(!overlay.dismiss_on_tap());
    let overlay = Overlay::new();
    assert!(overlay.dismiss_on_tap());
}

#[test]
fn dialog_config_builder() {
    let config = DialogConfig::alert("Title", "Msg")
        .confirm_label("Got it")
        .backdrop_opacity(0.8)
        .dismiss_on_backdrop(false);
    assert_eq!(config.confirm_label, "Got it");
    assert_eq!(config.backdrop_opacity, 0.8);
    assert!(!config.dismiss_on_backdrop);
}
