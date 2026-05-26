use crate::animation::{Animation, AnimationState};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnimationId(u64);

pub struct Timeline {
    animations: HashMap<AnimationId, Animation>,
    next_id: u64,
}

impl Timeline {
    pub fn new() -> Self {
        Self { animations: HashMap::new(), next_id: 1 }
    }

    pub fn add(&mut self, animation: Animation) -> AnimationId {
        let id = AnimationId(self.next_id);
        self.next_id += 1;
        self.animations.insert(id, animation);
        id
    }

    pub fn remove(&mut self, id: AnimationId) {
        self.animations.remove(&id);
    }

    pub fn start(&mut self, id: AnimationId) {
        if let Some(anim) = self.animations.get_mut(&id) { anim.start(); }
    }

    pub fn start_all(&mut self) {
        for anim in self.animations.values_mut() { anim.start(); }
    }

    pub fn pause(&mut self, id: AnimationId) {
        if let Some(anim) = self.animations.get_mut(&id) { anim.pause(); }
    }

    pub fn tick(&mut self, dt_ms: u64) {
        for anim in self.animations.values_mut() { anim.tick(dt_ms); }
    }

    pub fn tick_ms(&mut self, dt_ms: u64) { self.tick(dt_ms); }

    pub fn is_any_running(&self) -> bool {
        self.animations.values().any(|a| a.state() == AnimationState::Running)
    }

    pub fn animation(&self, id: AnimationId) -> Option<&Animation> {
        self.animations.get(&id)
    }

    pub fn animation_mut(&mut self, id: AnimationId) -> Option<&mut Animation> {
        self.animations.get_mut(&id)
    }

    pub fn running_count(&self) -> usize {
        self.animations.values().filter(|a| a.state() == AnimationState::Running).count()
    }

    pub fn clear_completed(&mut self) {
        self.animations.retain(|_, a| a.state() != AnimationState::Completed);
    }
}

impl Default for Timeline {
    fn default() -> Self { Self::new() }
}

pub struct AnimationHandle {
    id: AnimationId,
}

impl AnimationHandle {
    pub fn id(&self) -> AnimationId { self.id }
}
