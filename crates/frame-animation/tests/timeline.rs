use frame_animation::animation::Animation;
use frame_animation::timeline::Timeline;

#[test]
fn timeline_add_start_tick() {
    let mut timeline = Timeline::new();
    let id = timeline.add(Animation::tween(0.0, 100.0, 1000));
    timeline.start(id);
    assert!(timeline.is_any_running());
    timeline.tick(1000);
    assert!(!timeline.is_any_running());
}

#[test]
fn timeline_start_all() {
    let mut timeline = Timeline::new();
    let _a1 = timeline.add(Animation::tween(0.0, 50.0, 500));
    let _a2 = timeline.add(Animation::tween(0.0, 100.0, 1000));
    timeline.start_all();
    assert_eq!(timeline.running_count(), 2);
}

#[test]
fn timeline_clear_completed() {
    let mut timeline = Timeline::new();
    let id = timeline.add(Animation::tween(0.0, 100.0, 100));
    timeline.start(id);
    timeline.tick(100);
    timeline.clear_completed();
    assert!(timeline.animation(id).is_none());
}
