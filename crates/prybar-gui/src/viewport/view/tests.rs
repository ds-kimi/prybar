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

/// The packed path form finds the VVD and VTX inside the package too.
#[gpui::test]
async fn packed_model_loads_with_its_companions(cx: &mut TestAppContext) {
    let dir = tempfile::tempdir().unwrap();
    let sample = crate::sample::write(dir.path()).unwrap();
    let packed = format!("{}:models/sample.mdl", sample.package.display());
    let v = cx.new(|_| Viewport::default());
    v.update(cx, |v, cx| v.show(Some(packed.into()), cx));
    cx.run_until_parked();
    assert!(v.read_with(cx, |v, _| v.triangles) > 0);
}

/// Picking a sequence and another variant keeps the user's camera.
#[gpui::test]
async fn play_and_repick_keep_the_camera(cx: &mut TestAppContext) {
    let dir = tempfile::tempdir().unwrap();
    let sample = crate::sample::write(dir.path()).unwrap();
    let v = cx.new(|_| Viewport::default());
    v.update(cx, |v, cx| v.show(Some(sample.model.clone()), cx));
    cx.run_until_parked();
    let moved = v.update(cx, |v, cx| {
        let cam = v.camera.as_mut().unwrap();
        cam.distance *= 2.0;
        let moved = *cam;
        v.play(Some(0), cx);
        let lod = super::super::mesh::Pick {
            lod: 3,
            ..v.pick.clone()
        };
        v.repick(lod, cx);
        moved
    });
    cx.run_until_parked();
    v.read_with(cx, |v, _| {
        assert_eq!(v.camera, Some(moved));
        assert_eq!(v.play.seq, Some(0));
        assert_eq!(v.pick.lod, 3);
        assert!(v.triangles > 0, "missing LOD falls back to LOD 0");
    });
    v.update(cx, |v, cx| v.play(None, cx));
    assert!(v.read_with(cx, |v, _| v.play.clip.is_none()));
}
