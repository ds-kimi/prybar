use super::*;

fn unit() -> Bounds {
    Bounds {
        min: [-1.0, -2.0, 0.0],
        max: [1.0, 2.0, 4.0],
    }
}

#[test]
fn fit_centers_and_contains_every_corner() {
    let cam = Orbit::fit(&unit());
    assert_eq!(cam.target, Vec3::new(0.0, 0.0, 2.0));
    for aspect in [0.4, 1.0, 2.5] {
        for i in 0..8 {
            let b = unit();
            let p = [0, 1, 2].map(|a| if i >> a & 1 == 1 { b.max[a] } else { b.min[a] });
            let v = cam.view_proj(aspect) * Vec3::from(p).extend(1.0);
            let c = v.truncate() / v.w;
            assert!(c.x.abs() <= 1.0 && c.y.abs() <= 1.0, "{aspect} {p:?} {c}");
            assert!((0.0..=1.0).contains(&c.z), "depth {c}");
        }
    }
}

#[test]
fn eye_keeps_distance_and_clamps_pitch() {
    let mut cam = Orbit::fit(&unit());
    cam.pitch = 10.0;
    let d = (cam.eye() - cam.target).length();
    assert!((d - cam.distance).abs() < 1e-3);
    assert!(cam.view_proj(1.0).is_finite());
}

#[test]
fn bad_aspect_does_not_poison_the_matrix() {
    let cam = Orbit::fit(&Bounds::default());
    for aspect in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        assert!(cam.view_proj(aspect).is_finite());
    }
}
