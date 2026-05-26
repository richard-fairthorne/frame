use frame_animation::curve::AnimationCurve;
use frame_animation::curve::CubicBezier;

#[test]
fn linear_curve() {
    let curve = AnimationCurve::Linear;
    assert_eq!(curve.evaluate(0.0), 0.0);
    assert_eq!(curve.evaluate(0.5), 0.5);
    assert_eq!(curve.evaluate(1.0), 1.0);
}

#[test]
fn ease_in() {
    let curve = AnimationCurve::EaseIn;
    assert_eq!(curve.evaluate(0.0), 0.0);
    assert!(curve.evaluate(0.5) < 0.5);
    assert_eq!(curve.evaluate(1.0), 1.0);
}

#[test]
fn ease_out() {
    let curve = AnimationCurve::EaseOut;
    assert_eq!(curve.evaluate(0.0), 0.0);
    assert!(curve.evaluate(0.5) > 0.5);
    assert_eq!(curve.evaluate(1.0), 1.0);
}

#[test]
fn ease_in_out_symmetric() {
    let curve = AnimationCurve::EaseInOut;
    assert_eq!(curve.evaluate(0.0), 0.0);
    assert_eq!(curve.evaluate(1.0), 1.0);
    assert!((curve.evaluate(0.5) - 0.5).abs() < 0.01);
}

#[test]
fn cubic_bezier_basics() {
    let bezier = CubicBezier::new(0.42, 0.0, 0.58, 1.0);
    assert_eq!(bezier.evaluate(0.0), 0.0);
    assert_eq!(bezier.evaluate(1.0), 1.0);
}

#[test]
fn all_curves_clamped() {
    let curves = [
        AnimationCurve::Linear, AnimationCurve::EaseIn, AnimationCurve::EaseOut,
        AnimationCurve::EaseInOut, AnimationCurve::EaseInCubic, AnimationCurve::EaseOutCubic,
        AnimationCurve::EaseInExpo, AnimationCurve::EaseOutExpo,
    ];
    for curve in curves {
        assert_eq!(curve.evaluate(-1.0), 0.0);
        assert_eq!(curve.evaluate(2.0), 1.0);
    }
}
