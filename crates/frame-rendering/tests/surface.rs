use frame_core::{Size, WindowId};
use frame_rendering::surface::{HeadlessSurface, SurfaceProvider};
use frame_rendering::framecoord::Frame;
use frame_ui::Text;
use slotmap::SlotMap;

fn new_key() -> WindowId {
    let mut sm: SlotMap<WindowId, ()> = SlotMap::with_key();
    sm.insert(())
}

fn two_keys() -> (WindowId, WindowId) {
    let mut sm: SlotMap<WindowId, ()> = SlotMap::with_key();
    let a = sm.insert(());
    let b = sm.insert(());
    (a, b)
}

#[test]
fn headless_surface_create_destroy() {
    let mut surface = HeadlessSurface::new();
    let id = new_key();
    let handle = surface
        .create_surface(id, Size::new(800.0, 600.0))
        .unwrap();
    assert_eq!(handle.width, 800.0);
    assert_eq!(handle.height, 600.0);
    surface.destroy_surface(handle);
}

#[test]
fn headless_surface_resize() {
    let mut surface = HeadlessSurface::new();
    let id = new_key();
    let handle = surface
        .create_surface(id, Size::new(800.0, 600.0))
        .unwrap();
    surface.resize_surface(&handle, Size::new(1024.0, 768.0));
}

#[test]
fn headless_present_captures_frame() {
    let mut surface = HeadlessSurface::new();
    let id = new_key();
    let handle = surface
        .create_surface(id, Size::new(800.0, 600.0))
        .unwrap();
    let scene = vello::Scene::new();
    surface.present(&handle, &scene).unwrap();

    let frames = surface.captured_frames();
    assert_eq!(frames.len(), 1);
    assert_eq!(frames[0].width, 800);
    assert_eq!(frames[0].height, 600);
}

#[test]
fn headless_present_unknown_handle_errors() {
    let mut surface = HeadlessSurface::new();
    let (id, id2) = two_keys();
    let _handle = surface
        .create_surface(id, Size::new(800.0, 600.0))
        .unwrap();
    let bad_handle = frame_rendering::SurfaceHandle {
        id: id2,
        width: 100.0,
        height: 100.0,
        raw_handle: None,
    };
    let scene = vello::Scene::new();
    let result = surface.present(&bad_handle, &scene);
    assert!(result.is_err());
}

#[test]
fn frame_render_cycle() {
    let surface = HeadlessSurface::new();
    let mut frame = Frame::new(surface);

    let id = new_key();
    frame.create_surface(id, Size::new(800.0, 600.0)).unwrap();

    let mut widget = Text::new("Hello").size(16.0);
    assert!(frame.is_dirty());

    let rendered = frame.tick(&mut widget);
    assert!(rendered);
    assert!(!frame.is_dirty());

    frame.render(id, &mut widget).unwrap();
}

#[test]
fn frame_mark_dirty() {
    let surface = HeadlessSurface::new();
    let mut frame = Frame::new(surface);

    let id = new_key();
    frame.create_surface(id, Size::new(800.0, 600.0)).unwrap();

    let mut widget = Text::new("Test");
    frame.tick(&mut widget);
    assert!(!frame.is_dirty());

    frame.mark_dirty();
    assert!(frame.is_dirty());
}

#[test]
fn frame_render_unknown_surface_errors() {
    let surface = HeadlessSurface::new();
    let mut frame = Frame::new(surface);

    let (id, missing) = two_keys();
    frame.create_surface(id, Size::new(800.0, 600.0)).unwrap();

    let mut widget = Text::new("Test");
    let result = frame.render(missing, &mut widget);
    assert!(result.is_err());
}

#[test]
fn frame_size_tracks_render_loop() {
    let surface = HeadlessSurface::new();
    let mut frame = Frame::new(surface);

    let id = new_key();
    frame.create_surface(id, Size::new(800.0, 600.0)).unwrap();
    assert_eq!(frame.size(), Size::new(800.0, 600.0));

    frame.resize(id, Size::new(1024.0, 768.0));
    assert_eq!(frame.size(), Size::new(1024.0, 768.0));
}
