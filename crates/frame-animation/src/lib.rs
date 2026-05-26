pub mod curve;
pub mod spring;
pub mod animation;
pub mod timeline;

pub use curve::{AnimationCurve, EasingFunction};
pub use spring::{Spring, SpringConfig, SpringState};
pub use animation::{Animation, AnimationState, AnimationTarget};
pub use timeline::{Timeline, AnimationHandle};
