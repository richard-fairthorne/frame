use frame_core::render_loop::RenderLoop;
use frame_core::Size;

#[test]
fn render_loop_dirty_tracking() {
    let loop_ = RenderLoop::new();
    assert!(loop_.is_dirty());
    loop_.clear_dirty();
    assert!(!loop_.is_dirty());
    loop_.mark_dirty();
    assert!(loop_.is_dirty());
}

#[test]
fn render_loop_size_change_marks_dirty() {
    let loop_ = RenderLoop::new();
    loop_.clear_dirty();
    loop_.set_size(Size::new(1024.0, 768.0));
    assert!(loop_.is_dirty());
    assert_eq!(loop_.size(), Size::new(1024.0, 768.0));
}
