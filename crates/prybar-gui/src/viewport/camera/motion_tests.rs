use glam::Vec2;

use super::tests::unit;
use super::*;

#[test]
fn inertia_decays_the_same_at_any_frame_rate() {
    let run = |fps: f32| {
        let mut cam = Orbit::fit(&unit());
        let mut i = Inertia {
            spin: Vec2::new(600.0, 0.0),
            ..Default::default()
        };
        let mut steps = 0;
        while i.step(&mut cam, 1.0 / fps, 600.0) && steps < 10_000 {
            steps += 1;
        }
        assert!(!i.moving(), "stops cleanly at {fps}");
        cam.yaw
    };
    assert!((run(60.0) - run(120.0)).abs() < 0.05);
}

#[test]
fn release_uses_only_recent_samples() {
    let s = [
        (0.0, Vec2::ZERO),
        (0.95, Vec2::ZERO),
        (1.0, Vec2::new(10.0, 0.0)),
    ];
    assert!((Inertia::release(&s, 1.0) - Vec2::new(200.0, 0.0)).length() < 0.01);
    assert_eq!(Inertia::release(&s, 5.0), Vec2::ZERO);
    assert_eq!(Inertia::release(&[], 0.0), Vec2::ZERO);
}

#[test]
fn zoom_and_pan_stay_within_limits() {
    let mut cam = Orbit::fit(&unit());
    let mut i = Inertia {
        zoom_to: Some(1e-6),
        ..Default::default()
    };
    let mut n = 0;
    while i.step(&mut cam, 1.0 / 60.0, 600.0) {
        n += 1;
        assert!(n < 1000, "unreachable zoom goal ends");
    }
    assert!(cam.distance >= cam.radius * 0.3);
    cam.zoom(1e9, None);
    assert!(cam.distance <= cam.radius * 12.5);
    for _ in 0..100 {
        cam.pan(Vec2::new(1e4, 1e4), 600.0);
    }
    assert!((cam.target - cam.home).length() <= cam.radius * 3.01);
}
