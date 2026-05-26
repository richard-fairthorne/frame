use frame_animation::animation::{Animation, AnimationState};
use std::sync::{Arc, Mutex};

#[test]
fn tween_basic() {
    let mut anim = Animation::tween(0.0, 100.0, 1000);
    anim.start();
    assert_eq!(anim.state(), AnimationState::Running);

    anim.tick(500);
    let v = anim.current_value();
    assert!(v > 0.0 && v < 100.0);

    anim.tick(500);
    assert_eq!(anim.state(), AnimationState::Completed);
    assert!((anim.current_value() - 100.0).abs() < 0.01);
}

#[test]
fn tween_with_delay() {
    let mut anim = Animation::tween(0.0, 100.0, 1000).delay(500);
    anim.start();
    anim.tick(200);
    assert_eq!(anim.current_value(), 0.0);
    anim.tick(301);
    assert!(anim.current_value() > 0.0);
}

#[test]
fn tween_with_target() {
    let value = Arc::new(Mutex::new(0.0f32));
    let target = Box::new(frame_animation::animation::SignalTarget::new(Arc::clone(&value)));

    let mut anim = Animation::tween(0.0, 50.0, 1000).target(target);
    anim.start();
    anim.tick(1000);

    assert!((*value.lock().unwrap() - 50.0).abs() < 0.01);
}

#[test]
fn tween_pause_resume() {
    let mut anim = Animation::tween(0.0, 100.0, 1000);
    anim.start();
    anim.tick(500);
    anim.pause();
    assert_eq!(anim.state(), AnimationState::Paused);
    let v = anim.current_value();
    anim.tick(100);
    assert_eq!(anim.current_value(), v);
    anim.resume();
    anim.tick(500);
    assert_eq!(anim.state(), AnimationState::Completed);
}

#[test]
fn tween_repeat() {
    let mut anim = Animation::tween(0.0, 100.0, 100).repeat();
    anim.start();
    anim.tick(100);
    assert_eq!(anim.state(), AnimationState::Running);
    anim.tick(100);
    assert_eq!(anim.state(), AnimationState::Running);
}

#[test]
fn tween_auto_reverse() {
    let mut anim = Animation::tween(0.0, 100.0, 100).repeat().auto_reverse();
    anim.start();
    anim.tick(100);
    anim.tick(50);
    assert!(anim.current_value() > 0.0 && anim.current_value() < 100.0);
}

#[test]
fn animation_progress() {
    let mut anim = Animation::tween(0.0, 100.0, 1000);
    anim.start();
    assert_eq!(anim.progress(), 0.0);
    anim.tick(500);
    assert!((anim.progress() - 0.5).abs() < 0.01);
}
