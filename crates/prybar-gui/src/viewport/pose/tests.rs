use glam::{Mat4, Vec3};

use super::fixtures::rig;
use super::*;
use crate::viewport::mesh::Scene;

#[test]
fn clip_blends_frames_and_skins_vertices() {
    let clip = Clip::new(&rig(0), 0).unwrap();
    assert!(clip.looping);
    assert!((clip.duration() - 0.1).abs() < 1e-6);
    let world = clip.at(0.05);
    let child = world[1].w_axis.truncate();
    assert!(child.abs_diff_eq(Vec3::new(0.0, 0.0, 2.0), 1e-4), "{child}");
    let bind = Scene {
        vertices: vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        weights: vec![[(1, 1.0), (0, 0.0), (0, 0.0)]],
        ..Scene::default()
    };
    let mut out = Vec::new();
    skin(&bind, &clip.skinning(&world), &mut out);
    assert!((out[2] - 2.0).abs() < 1e-4, "{out:?}");
    assert_eq!(joints(&world, &clip.parents).len(), 1);
}

/// A parent pointing at itself or later is treated as a root.
#[test]
fn hostile_parents_and_weights_never_panic() {
    let clip = Clip::new(&rig(5), 0).unwrap();
    assert_eq!(clip.parents, vec![None, None]);
    let bind = Scene {
        vertices: vec![1.0; 8],
        weights: vec![[(200, 1.0), (0, f32::NAN), (1, -1.0)]],
        ..Scene::default()
    };
    let mut out = Vec::new();
    skin(&bind, &[Mat4::IDENTITY], &mut out);
    assert_eq!(out, vec![1.0; 8]);
    assert!(joints(&[], &clip.parents).is_empty());
}

#[test]
fn sequences_without_frames_do_not_play() {
    let mut ir = rig(0);
    ir.animations[0].frames.clear();
    assert!(Clip::new(&ir, 0).is_none());
    assert!(Clip::new(&ir, 9).is_none());
    ir.sequences[0].anims = vec![-1];
    assert!(Clip::new(&ir, 0).is_none());
}

#[test]
fn playback_loops_or_stops_and_toggles() {
    let mut clip = Clip::new(&rig(0), 0).unwrap();
    let mut play = Play {
        clip: Some(std::sync::Arc::new(clip.clone())),
        ..Play::default()
    };
    assert!(play.advance(0.15));
    assert!((play.time - 0.05).abs() < 1e-4, "looped: {}", play.time);
    clip.looping = false;
    play.clip = Some(std::sync::Arc::new(clip));
    play.speed = 1; // 2x
    assert!(play.advance(0.1));
    assert!(play.paused && (play.progress() - 1.0).abs() < 1e-6);
    assert!(!play.advance(0.1));
    play.toggle();
    assert!(!play.paused && play.time == 0.0, "replays from the start");
}
