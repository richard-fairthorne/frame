use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex, OnceLock};

use crate::reactive::tracker::SubscriberId;

thread_local! {
    static GLOBAL_TRACKER: RefCell<crate::reactive::tracker::Tracker> =
        RefCell::new(crate::reactive::tracker::Tracker::new());
    static PENDING_SIGNALS: RefCell<HashSet<usize>> = RefCell::new(HashSet::new());
}

static RENDER_REQUEST_CALLBACK: OnceLock<Box<dyn Fn() + Send + Sync>> = OnceLock::new();

static EFFECTS: LazyLock<Mutex<HashMap<usize, Arc<dyn Fn() + Send + Sync>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static EFFECT_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn set_render_request_fn(f: Box<dyn Fn() + Send + Sync>) {
    let _ = RENDER_REQUEST_CALLBACK.set(f);
}

pub fn request_render() {
    if let Some(callback) = RENDER_REQUEST_CALLBACK.get() {
        callback();
    }
}

pub fn notify_signal_changed(signal_id: usize) {
    if crate::reactive::batch::is_batching() {
        PENDING_SIGNALS.with(|ps| {
            ps.borrow_mut().insert(signal_id);
        });
        return;
    }
    request_render();
}

pub fn flush_batch_notifications() {
    let has_pending = PENDING_SIGNALS.with(|ps| {
        let pending = !ps.borrow().is_empty();
        if pending {
            ps.borrow_mut().clear();
        }
        pending
    });
    if has_pending {
        request_render();
    }
}

pub fn track_read(signal_id: usize) {
    GLOBAL_TRACKER.with(|t| {
        t.borrow_mut().record_dependency(signal_id);
    });
}

pub fn start_effect_tracking(sub: SubscriberId) {
    GLOBAL_TRACKER.with(|t| {
        t.borrow_mut().start_tracking(sub);
    });
}

pub fn stop_effect_tracking() {
    GLOBAL_TRACKER.with(|t| {
        t.borrow().stop_tracking();
    });
}

pub fn next_effect_id() -> SubscriberId {
    SubscriberId(EFFECT_COUNTER.fetch_add(1, Ordering::Relaxed))
}

pub fn register_effect(id: SubscriberId, run: Arc<dyn Fn() + Send + Sync>) {
    EFFECTS.lock().unwrap().insert(id.0, run);
}

pub fn run_effect(id: SubscriberId) {
    let effect_fn = EFFECTS.lock().unwrap().get(&id.0).cloned();
    if let Some(f) = effect_fn {
        start_effect_tracking(id);
        f();
        stop_effect_tracking();
    }
}
