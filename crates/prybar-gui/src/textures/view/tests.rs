use glam::Vec2;
use gpui::{AppContext as _, TestAppContext};

use super::*;

#[test]
fn zoom_keeps_the_point_under_the_cursor() {
    let (area, image) = (Vec2::new(400.0, 300.0), Vec2::new(100.0, 100.0));
    let mut pan = Pan::default();
    assert_eq!(
        pan.scale(area, image),
        3.0,
        "fitting enlarges by whole steps"
    );
    let at = Vec2::new(30.0, -20.0);
    let texel = (at - pan.offset) / pan.scale(area, image);
    pan.zoom_at(at, 2.0, area, image);
    assert_eq!((at - pan.offset) / pan.scale(area, image), texel);
    pan.zoom_at(at, 1e9, area, image);
    assert!(pan.scale(area, image) <= 64.0);
}

#[test]
fn glide_slows_down_and_stops() {
    let mut pan = Pan {
        slide: Vec2::new(900.0, 0.0),
        ..Pan::default()
    };
    let mut frames = 0;
    while pan.step(1.0 / 60.0) {
        frames += 1;
        assert!(frames < 600, "glide never stopped");
    }
    assert!(pan.offset.x > 0.0 && frames > 5);
}

#[test]
fn alpha_channel_shows_as_grey() {
    let img = prybar_vtf::Image {
        width: 1,
        height: 1,
        rgba: vec![10, 20, 30, 77],
    };
    let rgb = load::image(img.clone(), Channel::Rgb).unwrap();
    let alpha = load::image(img, Channel::Alpha).unwrap();
    assert_eq!(rgb.as_bytes(0).unwrap(), &[30, 20, 10, 255]);
    assert_eq!(alpha.as_bytes(0).unwrap(), &[77, 77, 77, 255]);
}

/// The sample model's material is found next to it and decodes; the
/// mip control clamps to the levels the file has.
#[gpui::test]
async fn sample_texture_is_found_and_shown(cx: &mut TestAppContext) {
    let dir = tempfile::tempdir().unwrap();
    let sample = crate::sample::write(dir.path()).unwrap();
    let v = cx.new(|_| TextureView::default());
    v.update(cx, |v, cx| v.show(Some(sample.model.clone()), cx));
    cx.run_until_parked();
    let (status, size) = v.read_with(cx, |v, _| {
        (
            v.status.clone(),
            v.shown.as_ref().map(|s| (s.width, s.height)),
        )
    });
    assert_eq!((status, size), (Status::Ready, Some((64, 64))));
    v.update(cx, |v, cx| v.set_mip(200, cx));
    cx.run_until_parked();
    let (mip, size) = v.read_with(cx, |v, _| (v.mip, v.shown.as_ref().map(|s| s.width)));
    assert_eq!((mip, size), (6, Some(1)));
}
