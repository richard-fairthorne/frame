use std::sync::Arc;

use crate::reactive::tracker::SubscriberId;

pub struct Effect {
    _id: SubscriberId,
    _run: Arc<dyn Fn() + Send + Sync>,
}

impl Effect {
    pub fn new<F>(run: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        let id = crate::reactive::runtime::next_effect_id();
        let run: Arc<dyn Fn() + Send + Sync> = Arc::new(run);

        crate::reactive::runtime::register_effect(id, run.clone());
        crate::reactive::runtime::run_effect(id);

        Self {
            _id: id,
            _run: run,
        }
    }
}
