use frame_animation::spring::{Spring, SpringConfig};

#[test]
fn spring_converges() {
    let mut spring = Spring::new(0.0, 100.0);
    for _ in 0..200 {
        spring.advance_by_ms(16.0);
    }
    assert!((spring.state.current - 100.0).abs() < 1.0);
}

#[test]
fn spring_gentle() {
    let mut spring = Spring::new(0.0, 1.0).with_config(SpringConfig::gentle());
    for _ in 0..300 {
        spring.advance_by_ms(16.0);
    }
    assert!(spring.is_at_rest());
}

#[test]
fn spring_bouncy_oscillates() {
    let mut spring = Spring::new(0.0, 100.0).with_config(SpringConfig::bouncy());
    let mut crossed_target = false;
    for _ in 0..100 {
        spring.advance_by_ms(16.0);
        if spring.state.current > 100.0 { crossed_target = true; }
    }
    assert!(crossed_target);
}

#[test]
fn spring_config_overdamped() {
    let config = SpringConfig::stiff();
    assert!(config.is_overdamped());
}

#[test]
fn spring_config_underdamped() {
    let config = SpringConfig::bouncy();
    assert!(config.is_underdamped());
}

#[test]
fn spring_set_target() {
    let mut spring = Spring::new(0.0, 100.0);
    spring.set_target(200.0);
    for _ in 0..200 {
        spring.advance_by_ms(16.0);
    }
    assert!((spring.state.current - 200.0).abs() < 2.0);
}
