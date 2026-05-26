use crate::curve::AnimationCurve;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    Idle,
    Running,
    Paused,
    Completed,
}

pub trait AnimationTarget: Send + Sync {
    fn set_value(&mut self, value: f32);
    fn clone_box(&self) -> Box<dyn AnimationTarget>;
}

impl Clone for Box<dyn AnimationTarget> {
    fn clone(&self) -> Self { self.clone_box() }
}

pub struct SignalTarget {
    signal: Arc<Mutex<f32>>,
}

impl SignalTarget {
    pub fn new(signal: Arc<Mutex<f32>>) -> Self {
        Self { signal }
    }
}

impl AnimationTarget for SignalTarget {
    fn set_value(&mut self, value: f32) {
        *self.signal.lock().unwrap() = value;
    }
    fn clone_box(&self) -> Box<dyn AnimationTarget> {
        Box::new(SignalTarget { signal: Arc::clone(&self.signal) })
    }
}

pub struct Animation {
    from: f32,
    to: f32,
    duration_ms: u64,
    delay_ms: u64,
    curve: AnimationCurve,
    state: AnimationState,
    elapsed_ms: u64,
    repeat: bool,
    auto_reverse: bool,
    forward: bool,
    target: Option<Box<dyn AnimationTarget>>,
    on_complete: Option<Box<dyn FnMut() + Send + Sync>>,
}

impl Animation {
    pub fn tween(from: f32, to: f32, duration_ms: u64) -> Self {
        Self {
            from, to, duration_ms, delay_ms: 0, curve: AnimationCurve::EaseInOut,
            state: AnimationState::Idle, elapsed_ms: 0, repeat: false,
            auto_reverse: false, forward: true, target: None, on_complete: None,
        }
    }

    pub fn curve(mut self, curve: AnimationCurve) -> Self { self.curve = curve; self }
    pub fn delay(mut self, ms: u64) -> Self { self.delay_ms = ms; self }
    pub fn repeat(mut self) -> Self { self.repeat = true; self }
    pub fn auto_reverse(mut self) -> Self { self.auto_reverse = true; self }

    pub fn target(mut self, target: Box<dyn AnimationTarget>) -> Self {
        self.target = Some(target); self
    }

    pub fn on_complete(mut self, f: impl FnMut() + Send + Sync + 'static) -> Self {
        self.on_complete = Some(Box::new(f)); self
    }

    pub fn start(&mut self) {
        self.state = AnimationState::Running;
        self.elapsed_ms = 0;
        self.forward = true;
    }

    pub fn pause(&mut self) {
        if self.state == AnimationState::Running { self.state = AnimationState::Paused; }
    }

    pub fn resume(&mut self) {
        if self.state == AnimationState::Paused { self.state = AnimationState::Running; }
    }

    pub fn stop(&mut self) {
        self.state = AnimationState::Idle;
        self.elapsed_ms = 0;
    }

    pub fn tick(&mut self, dt_ms: u64) {
        if self.state != AnimationState::Running { return; }
        self.elapsed_ms += dt_ms;

        let value = self.current_value();

        if let Some(ref mut target) = self.target {
            target.set_value(value);
        }

        let adjusted = self.elapsed_ms as i64 - self.delay_ms as i64;
        if adjusted >= 0 && (adjusted as u64) >= self.duration_ms {
            if self.repeat {
                self.elapsed_ms = 0;
                if self.auto_reverse { self.forward = !self.forward; }
            } else {
                self.state = AnimationState::Completed;
                if let Some(ref mut cb) = self.on_complete { cb(); }
            }
        }
    }

    pub fn current_value(&self) -> f32 {
        let adjusted = self.elapsed_ms as i64 - self.delay_ms as i64;
        if adjusted <= 0 { return if self.forward { self.from } else { self.to }; }

        let t = (adjusted as f32 / self.duration_ms as f32).min(1.0);
        let eased = self.curve.evaluate(t);

        let (from, to) = if self.forward { (self.from, self.to) } else { (self.to, self.from) };
        from + (to - from) * eased
    }

    pub fn state(&self) -> AnimationState { self.state }
    pub fn progress(&self) -> f32 {
        let adjusted = self.elapsed_ms as f32 - self.delay_ms as f32;
        (adjusted / self.duration_ms as f32).clamp(0.0, 1.0)
    }
}
