use frame_core::reactive::batch::{batch, is_batching};
use frame_core::reactive::signal::Signal;

#[test]
fn batch_groups_signal_changes() {
    let a = Signal::new(1);
    let b = Signal::new(2);

    batch(|| {
        a.set(10);
        b.set(20);
    });

    assert_eq!(a.get(), 10);
    assert_eq!(b.get(), 20);
}

#[test]
fn batch_is_reentrant() {
    let a = Signal::new(1);

    batch(|| {
        a.set(10);
        batch(|| {
            a.set(20);
        });
        assert_eq!(a.get(), 20);
    });

    assert_eq!(a.get(), 20);
}

#[test]
fn batch_is_batching() {
    assert!(!is_batching());
    batch(|| {
        assert!(is_batching());
    });
    assert!(!is_batching());
}
