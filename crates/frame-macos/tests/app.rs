use frame_core::Size;
use frame_macos::{AppBuilder, MacosApp};
use frame_ui::{Container, Text};

#[test]
fn macos_app_create_and_tick() {
    let mut app = MacosApp::new().with_size(800.0, 600.0);
    app.create_window();

    let mut widget = Text::new("Hello").size(16.0);
    let dirty = app.tick(&mut widget);
    assert!(dirty);

    app.render(&mut widget);
}

#[test]
fn macos_app_dirty_clears_after_tick() {
    let mut app = MacosApp::new();
    app.create_window();

    let mut widget = Text::new("Test");
    app.tick(&mut widget);
    assert!(!app.frame().is_dirty());

    app.frame().mark_dirty();
    assert!(app.frame().is_dirty());
}

#[test]
fn macos_app_resize() {
    let mut app = MacosApp::new().with_size(800.0, 600.0);
    app.create_window();

    app.resize(Size::new(1024.0, 768.0));
    assert_eq!(app.window_size(), Size::new(1024.0, 768.0));
}

#[test]
fn app_builder_defaults() {
    let builder = AppBuilder::new();
    assert_eq!(builder.get_title(), "Frame App");
    assert_eq!(builder.get_width(), 800.0);
    assert_eq!(builder.get_height(), 600.0);
    assert!(builder.get_resizable());

    let custom = AppBuilder::new()
        .title("Custom")
        .size(1024.0, 768.0)
        .resizable(false);
    assert_eq!(custom.get_title(), "Custom");
    assert_eq!(custom.get_width(), 1024.0);
    assert_eq!(custom.get_height(), 768.0);
    assert!(!custom.get_resizable());
}

#[test]
fn macos_app_full_render_cycle() {
    let mut app = MacosApp::new().with_size(400.0, 300.0);
    app.create_window();

    let mut widget = frame_ui::Column::new()
        .gap(8.0)
        .child(Text::new("Header").size(20.0))
        .child(Text::new("Body text").size(14.0))
        .child(Container::new().padding(16.0).child(Text::new("Card")));

    assert!(app.tick(&mut widget));
    app.render(&mut widget);

    assert!(!app.tick(&mut widget));

    app.frame().mark_dirty();
    assert!(app.tick(&mut widget));
    app.render(&mut widget);
}
