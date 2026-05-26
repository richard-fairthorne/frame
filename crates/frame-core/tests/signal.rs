use frame_core::reactive::signal::Signal;

#[test]
fn signal_new_and_get() {
    let s = Signal::new(42);
    assert_eq!(s.get(), 42);
}

#[test]
fn signal_set_and_get() {
    let s = Signal::new(0);
    s.set(99);
    assert_eq!(s.get(), 99);
}

#[test]
fn signal_update() {
    let s = Signal::new(10);
    s.update(|v| *v + 5);
    assert_eq!(s.get(), 15);
}

#[test]
fn signal_with_fn() {
    let s = Signal::new(String::from("hello"));
    s.with(|v| assert_eq!(v, "hello"));
}

#[test]
fn signal_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Signal<i32>>();
}

#[test]
fn signal_has_id() {
    let s = Signal::new(1);
    let id = s.id();
    let _ = id;
}
