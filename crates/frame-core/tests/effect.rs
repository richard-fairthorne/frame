use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use frame_core::reactive::effect::Effect;

#[test]
fn effect_runs_immediately() {
    let counter = Arc::new(AtomicI32::new(0));
    let c = counter.clone();
    Effect::new(move || {
        c.fetch_add(1, Ordering::SeqCst);
    });
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[test]
fn effect_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Effect>();
}
