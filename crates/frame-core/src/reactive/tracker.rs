use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

thread_local! {
    static CURRENT_TRACKING: RefCell<Option<SubscriberId>> = const { RefCell::new(None) };
    static TRACKED_SIGNALS: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubscriberId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignalId(pub usize);

pub struct Tracker {
    signal_to_subs: HashMap<SignalId, HashSet<SubscriberId>>,
    sub_to_signals: HashMap<SubscriberId, HashSet<SignalId>>,
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}

impl Tracker {
    pub fn new() -> Self {
        Self {
            signal_to_subs: HashMap::new(),
            sub_to_signals: HashMap::new(),
        }
    }

    pub fn start_tracking(&mut self, sub: SubscriberId) {
        self.clear_subscriber(sub);
        TRACKED_SIGNALS.with(|t| {
            t.borrow_mut().clear();
        });
        CURRENT_TRACKING.with(|t| {
            *t.borrow_mut() = Some(sub);
        });
    }

    pub fn record_dependency(&mut self, signal: usize) {
        CURRENT_TRACKING.with(|t| {
            if let Some(sub) = *t.borrow() {
                let sig_id = SignalId(signal);
                self.signal_to_subs
                    .entry(sig_id)
                    .or_default()
                    .insert(sub);
                self.sub_to_signals
                    .entry(sub)
                    .or_default()
                    .insert(sig_id);
                TRACKED_SIGNALS.with(|ts| {
                    ts.borrow_mut().push(signal);
                });
            }
        });
    }

    pub fn stop_tracking(&self) -> Vec<usize> {
        CURRENT_TRACKING.with(|t| {
            *t.borrow_mut() = None;
        });
        TRACKED_SIGNALS.with(|t| t.borrow().clone())
    }

    pub fn signal_changed(&mut self, signal: usize) -> HashSet<SubscriberId> {
        let sig_id = SignalId(signal);
        self.signal_to_subs
            .get(&sig_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn clear_subscriber(&mut self, sub: SubscriberId) {
        if let Some(signals) = self.sub_to_signals.remove(&sub) {
            for sig in signals {
                if let Some(subs) = self.signal_to_subs.get_mut(&sig) {
                    subs.remove(&sub);
                }
            }
        }
    }
}
