use super::*;
use crate::viewport::mesh;

pub(super) fn sample_scene() -> Scene {
    let dir = tempfile::tempdir().unwrap();
    let sample = crate::sample::write(dir.path()).unwrap();
    let ir = prybar_service::model::open(&sample.model.to_string_lossy()).unwrap();
    mesh::build(&ir, &[], 0).unwrap()
}

pub(super) fn request(scene: &Scene, width: u32, height: u32) -> Request {
    Request {
        width,
        height,
        camera: Orbit::fit(&scene.bounds),
        flags: Flags {
            wire: true,
            normals: false,
            bones: true,
        },
        clear: [0.0, 0.0, 0.0, 1.0],
        grid: [0.5, 0.5, 0.5, 0.5],
        wire: [1.0, 1.0, 1.0, 0.3],
    }
}

/// CI machines may have no GPU at all; the renderer must say so, and
/// the pixel checks only run where one exists.
#[test]
fn renders_the_sample_at_any_size() {
    let mut r = match Renderer::new() {
        Ok(r) => r,
        Err(e) => return eprintln!("skipped: {e}"),
    };
    let scene = sample_scene();
    assert!(r.render(&request(&scene, 8, 8)).is_err(), "no scene yet");
    r.set_scene(&scene);
    for (w, h) in [(96, 64), (33, 77), (0, 0)] {
        let f = r.render(&request(&scene, w, h)).unwrap();
        assert_eq!(f.bgra.len() as u32, f.width * f.height * 4);
        assert!(f.width >= 1 && f.height >= 1);
    }
    let f = r.render(&request(&scene, 96, 64)).unwrap();
    let center = ((32 * 96 + 48) * 4) as usize;
    assert_ne!(
        &f.bgra[center..center + 3],
        &[0, 0, 0],
        "model not drawn at the center"
    );
}

#[test]
fn worker_answers_with_a_frame_or_an_error() {
    let w = Worker::spawn();
    let scene = sample_scene();
    let req = request(&scene, 40, 30);
    w.scene(scene);
    w.frame(req);
    loop {
        match w.rx.recv_blocking() {
            Ok(Output::Ready { .. }) => continue,
            Ok(Output::Frame(f)) => assert_eq!((f.width, f.height), (40, 30)),
            Ok(Output::Error(e)) => eprintln!("skipped: {e}"),
            Err(_) => panic!("render thread ended without answering"),
        }
        break;
    }
}
