use std::cell::RefCell;

thread_local! {
    static BATCH_DEPTH: RefCell<usize> = const { RefCell::new(0) };
}

pub fn batch<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    BATCH_DEPTH.with(|d| {
        *d.borrow_mut() += 1;
    });
    let result = f();
    BATCH_DEPTH.with(|d| {
        *d.borrow_mut() -= 1;
        if *d.borrow() == 0 {
            crate::reactive::runtime::flush_batch_notifications();
        }
    });
    result
}

pub fn is_batching() -> bool {
    BATCH_DEPTH.with(|d| *d.borrow() > 0)
}
