#[derive(Debug, Clone, Copy)]
pub struct SpringConfig {
    pub damping: f32,
    pub stiffness: f32,
    pub mass: f32,
    pub precision: f32,
}

impl SpringConfig {
    pub fn default_spring() -> Self {
        Self { damping: 10.0, stiffness: 100.0, mass: 1.0, precision: 0.01 }
    }

    pub fn gentle() -> Self {
        Self { damping: 8.0, stiffness: 50.0, mass: 1.0, precision: 0.01 }
    }

    pub fn bouncy() -> Self {
        Self { damping: 6.0, stiffness: 150.0, mass: 1.0, precision: 0.01 }
    }

    pub fn stiff() -> Self {
        Self { damping: 40.0, stiffness: 300.0, mass: 1.0, precision: 0.01 }
    }

    pub fn slow() -> Self {
        Self { damping: 15.0, stiffness: 40.0, mass: 1.0, precision: 0.01 }
    }

    pub fn is_overdamped(&self) -> bool {
        let critical = 2.0 * (self.stiffness * self.mass).sqrt();
        self.damping >= critical
    }

    pub fn is_underdamped(&self) -> bool {
        !self.is_overdamped()
    }
}

impl Default for SpringConfig {
    fn default() -> Self { Self::default_spring() }
}

#[derive(Debug, Clone, Copy)]
pub struct SpringState {
    pub current: f32,
    pub velocity: f32,
    pub target: f32,
}

impl SpringState {
    pub fn new(from: f32, target: f32) -> Self {
        Self { current: from, velocity: 0.0, target }
    }

    pub fn at_rest(&self, config: &SpringConfig) -> bool {
        (self.current - self.target).abs() < config.precision
            && self.velocity.abs() < config.precision
    }
}

pub struct Spring {
    pub config: SpringConfig,
    pub state: SpringState,
}

impl Spring {
    pub fn new(from: f32, to: f32) -> Self {
        Self { config: SpringConfig::default(), state: SpringState::new(from, to) }
    }

    pub fn with_config(mut self, config: SpringConfig) -> Self {
        self.config = config;
        self
    }

    pub fn set_target(&mut self, target: f32) {
        self.state.target = target;
    }

    pub fn step(&mut self, dt: f32) -> f32 {
        let displacement = self.state.current - self.state.target;
        let spring_force = -self.config.stiffness * displacement;
        let damping_force = -self.config.damping * self.state.velocity;
        let acceleration = (spring_force + damping_force) / self.config.mass;

        self.state.velocity += acceleration * dt;
        self.state.current += self.state.velocity * dt;

        self.state.current
    }

    pub fn advance_by_ms(&mut self, ms: f32) -> f32 {
        let steps = 8;
        let dt = (ms / 1000.0) / steps as f32;
        for _ in 0..steps {
            self.step(dt);
        }
        self.state.current
    }

    pub fn is_at_rest(&self) -> bool {
        self.state.at_rest(&self.config)
    }
}
