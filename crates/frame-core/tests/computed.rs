use frame_core::reactive::computed::Computed;
use frame_core::reactive::signal::Signal;

#[test]
fn computed_derives_from_signal() {
    let count = Signal::new(5);
    let doubled = Computed::new(move || count.get() * 2);
    assert_eq!(doubled.get(), 10);
}

#[test]
fn computed_updates_when_signal_changes() {
    let count = Signal::new(5);
    let count_clone = count.clone();
    let doubled = Computed::new(move || count_clone.get() * 2);
    assert_eq!(doubled.get(), 10);
    count.set(7);
    assert_eq!(doubled.get(), 14);
}

#[test]
fn computed_chain() {
    let a = Signal::new(1);
    let b = Signal::new(2);
    let a_c = a.clone();
    let b_c = b.clone();
    let sum = Computed::new(move || a_c.get() + b_c.get());
    let sum_c = sum.clone();
    let doubled = Computed::new(move || sum_c.get() * 2);
    assert_eq!(doubled.get(), 6);
    a.set(3);
    assert_eq!(doubled.get(), 10);
}

#[test]
fn computed_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Computed<i32>>();
}
