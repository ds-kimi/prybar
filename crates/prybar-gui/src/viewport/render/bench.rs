use super::tests::request;
use super::*;
use crate::viewport::mesh;

/// Frame timing on a real model: `PRYBAR_BENCH_MDL=<model> cargo test
/// --release bench -- --ignored --nocapture`. Orbits 120 frames at a
/// full-window Retina size.
#[test]
#[ignore]
fn bench() {
    let path = std::env::var("PRYBAR_BENCH_MDL").expect("PRYBAR_BENCH_MDL");
    let ir = prybar_service::model::open(&path).unwrap();
    let scene = mesh::build(&ir, &[], 0).unwrap();
    let mut r = Renderer::new().unwrap();
    r.set_scene(&scene);
    let mut req = request(&scene, 2400, 1500);
    let mut total = 0.0;
    for i in 0..120 {
        req.camera.yaw = i as f32 * 0.05;
        total += r.render(&req).unwrap().ms;
    }
    eprintln!(
        "{} tris, {:.2} ms/frame avg",
        scene.triangles(),
        total / 120.0
    );
}
