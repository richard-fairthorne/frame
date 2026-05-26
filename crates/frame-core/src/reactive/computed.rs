use std::sync::{Arc, RwLock};

struct ComputedInner<T> {
    compute: Box<dyn Fn() -> T + Send + Sync>,
    cached: RwLock<Option<T>>,
}

pub struct Computed<T> {
    inner: Arc<ComputedInner<T>>,
}

impl<T: Clone + 'static> Computed<T> {
    pub fn new<F>(compute: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            inner: Arc::new(ComputedInner {
                compute: Box::new(compute),
                cached: RwLock::new(None),
            }),
        }
    }

    pub fn get(&self) -> T {
        let v = (self.inner.compute)();
        let mut cached = self.inner.cached.write().unwrap();
        *cached = Some(v.clone());
        v
    }

    pub fn invalidate(&self) {
        {
            let mut cached = self.inner.cached.write().unwrap();
            *cached = None;
        }
        crate::reactive::runtime::request_render();
    }
}

impl<T> Clone for Computed<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
