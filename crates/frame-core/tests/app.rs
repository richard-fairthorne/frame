use frame_core::app::FrameApp;

#[test]
fn frame_app_builder_creates_app() {
    let app = FrameApp::builder().build();
    assert!(app.is_ok());
}

#[test]
fn frame_app_lifecycle() {
    let mut app = FrameApp::builder().build().unwrap();
    app.init();
    app.pause();
    app.resume();
    app.destroy();
}
