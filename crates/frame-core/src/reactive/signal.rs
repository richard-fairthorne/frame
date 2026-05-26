use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

static SIGNAL_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct Signal<T> {
    id: usize,
    value: Arc<RwLock<T>>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            value: Arc::clone(&self.value),
        }
    }
}

impl<T> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            id: SIGNAL_COUNTER.fetch_add(1, Ordering::Relaxed),
            value: Arc::new(RwLock::new(value)),
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        crate::reactive::runtime::track_read(self.id);
        *self.value.read().unwrap()
    }

    pub fn get_cloned(&self) -> T
    where
        T: Clone,
    {
        crate::reactive::runtime::track_read(self.id);
        self.value.read().unwrap().clone()
    }

    pub fn set(&self, value: T) {
        {
            let mut guard = self.value.write().unwrap();
            *guard = value;
        }
        crate::reactive::runtime::notify_signal_changed(self.id);
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T) -> T,
    {
        {
            let mut guard = self.value.write().unwrap();
            *guard = f(&mut *guard);
        }
        crate::reactive::runtime::notify_signal_changed(self.id);
    }

    pub fn with<U, F>(&self, f: F) -> U
    where
        F: FnOnce(&T) -> U,
    {
        crate::reactive::runtime::track_read(self.id);
        let guard = self.value.read().unwrap();
        f(&guard)
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
