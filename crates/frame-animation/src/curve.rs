pub trait EasingFunction: Send + Sync {
    fn evaluate(&self, t: f32) -> f32;
    fn clone_box(&self) -> Box<dyn EasingFunction>;
}

impl Clone for Box<dyn EasingFunction> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AnimationCurve {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInExpo,
    EaseOutExpo,
}

impl AnimationCurve {
    pub fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            AnimationCurve::Linear => t,
            AnimationCurve::EaseIn => t * t,
            AnimationCurve::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            AnimationCurve::EaseInOut => {
                if t < 0.5 { 2.0 * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(2) / 2.0 }
            }
            AnimationCurve::EaseInQuad => t * t,
            AnimationCurve::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            AnimationCurve::EaseInOutQuad => {
                if t < 0.5 { 2.0 * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(2) / 2.0 }
            }
            AnimationCurve::EaseInCubic => t * t * t,
            AnimationCurve::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            AnimationCurve::EaseInOutCubic => {
                if t < 0.5 { 4.0 * t * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(3) / 2.0 }
            }
            AnimationCurve::EaseInExpo => {
                if t == 0.0 { 0.0 } else { 2.0_f32.powf(10.0 * t - 10.0) }
            }
            AnimationCurve::EaseOutExpo => {
                if t == 1.0 { 1.0 } else { 1.0 - 2.0_f32.powf(-10.0 * t) }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CubicBezier {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl CubicBezier {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    pub fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        let mut guess = t;
        for _ in 0..8 {
            let x = self.sample_x(guess) - t;
            let dx = self.sample_dx(guess);
            if dx.abs() < 1e-6 { break; }
            guess -= x / dx;
            guess = guess.clamp(0.0, 1.0);
        }
        self.sample_y(guess)
    }

    fn sample_x(&self, t: f32) -> f32 {
        3.0 * (1.0 - t) * (1.0 - t) * t * self.x1 + 3.0 * (1.0 - t) * t * t * self.x2 + t * t * t
    }

    fn sample_y(&self, t: f32) -> f32 {
        3.0 * (1.0 - t) * (1.0 - t) * t * self.y1 + 3.0 * (1.0 - t) * t * t * self.y2 + t * t * t
    }

    fn sample_dx(&self, t: f32) -> f32 {
        3.0 * (1.0 - t) * (1.0 - t) * self.x1 + 6.0 * (1.0 - t) * t * (self.x2 - self.x1) + 3.0 * t * t * (1.0 - self.x2)
    }
}

#[derive(Debug, Clone)]
pub struct Tween<T: Clone + Copy + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + std::ops::Mul<f32, Output = T>> {
    pub from: T,
    pub to: T,
    pub duration_ms: u64,
    pub curve: AnimationCurve,
    pub delay_ms: u64,
}

impl<T> Tween<T>
where
    T: Clone + Copy + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + std::ops::Mul<f32, Output = T>,
{
    pub fn new(from: T, to: T, duration_ms: u64) -> Self {
        Self { from, to, duration_ms, curve: AnimationCurve::EaseInOut, delay_ms: 0 }
    }

    pub fn curve(mut self, curve: AnimationCurve) -> Self {
        self.curve = curve;
        self
    }

    pub fn delay(mut self, ms: u64) -> Self {
        self.delay_ms = ms;
        self
    }

    pub fn value_at(&self, elapsed_ms: u64) -> T {
        let adjusted = elapsed_ms as i64 - self.delay_ms as i64;
        if adjusted <= 0 { return self.from; }
        let t = (adjusted as f32 / self.duration_ms as f32).min(1.0);
        let eased_t = self.curve.evaluate(t);
        self.from + (self.to - self.from) * eased_t
    }

    pub fn is_complete(&self, elapsed_ms: u64) -> bool {
        let adjusted = elapsed_ms as i64 - self.delay_ms as i64;
        adjusted as f32 >= self.duration_ms as f32
    }
}
