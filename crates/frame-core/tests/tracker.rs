use frame_core::reactive::tracker::{Tracker, SubscriberId};

#[test]
fn tracker_records_dependencies() {
    let mut tracker = Tracker::new();
    let sub = SubscriberId(1);

    tracker.start_tracking(sub);
    tracker.record_dependency(0);
    tracker.record_dependency(1);
    let deps = tracker.stop_tracking();

    assert_eq!(deps, vec![0, 1]);
}

#[test]
fn tracker_notifies_subscribers_on_signal_change() {
    let mut tracker = Tracker::new();
    let sub1 = SubscriberId(1);
    let sub2 = SubscriberId(2);

    tracker.start_tracking(sub1);
    tracker.record_dependency(0);
    tracker.stop_tracking();

    tracker.start_tracking(sub2);
    tracker.record_dependency(1);
    tracker.stop_tracking();

    let notified = tracker.signal_changed(0);
    assert!(notified.contains(&sub1));
    assert!(!notified.contains(&sub2));
}

#[test]
fn tracker_removes_stale_dependencies() {
    let mut tracker = Tracker::new();
    let sub = SubscriberId(1);

    tracker.start_tracking(sub);
    tracker.record_dependency(0);
    tracker.record_dependency(1);
    tracker.stop_tracking();

    tracker.start_tracking(sub);
    tracker.record_dependency(0);
    tracker.stop_tracking();

    let notified = tracker.signal_changed(1);
    assert!(!notified.contains(&sub));

    let notified = tracker.signal_changed(0);
    assert!(notified.contains(&sub));
}
