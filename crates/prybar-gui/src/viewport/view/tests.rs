use gpui::{AppContext as _, TestAppContext};

use super::*;

/// A broken file ends as a message in the viewport, never a panic.
#[gpui::test]
async fn bad_model_shows_an_error(cx: &mut TestAppContext) {
    let dir = tempfile::tempdir().unwrap();
    let bad = dir.path().join("bad.mdl");
    std::fs::write(&bad, b"IDST not a model").unwrap();
    let v = cx.new(|_| Viewport::default());
    v.update(cx, |v, cx| v.show(Some(bad.clone()), cx));
    assert_eq!(v.read_with(cx, |v, _| v.status.clone()), Status::Loading);
    cx.run_until_parked();
    let status = v.read_with(cx, |v, _| v.status.clone());
    assert!(matches!(status, Status::Failed(_)), "{status:?}");
    v.update(cx, |v, cx| v.show(None, cx));
    assert_eq!(v.read_with(cx, |v, _| v.status.clone()), Status::Idle);
}

#[gpui::test]
async fn sample_model_gets_a_fitted_camera(cx: &mut TestAppContext) {
    let dir = tempfile::tempdir().unwrap();
    let sample = crate::sample::write(dir.path()).unwrap();
    let v = cx.new(|_| Viewport::default());
    v.update(cx, |v, cx| v.show(Some(sample.model.clone()), cx));
    cx.run_until_parked();
    let (camera, tris) = v.read_with(cx, |v, _| (v.camera, v.triangles));
    assert!(camera.is_some_and(|c| c.distance.is_finite()));
    assert!(tris > 0);
}
