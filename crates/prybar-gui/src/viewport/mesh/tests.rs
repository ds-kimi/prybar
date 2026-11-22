use super::fixtures::{ir, tri_model};
use super::*;
use prybar_mdl::ir::SubModel;

#[test]
fn sample_model_builds_with_valid_indices() {
    let dir = tempfile::tempdir().unwrap();
    let sample = crate::sample::write(dir.path()).unwrap();
    let ir = prybar_service::model::open(&sample.model.to_string_lossy()).unwrap();
    let scene = build(&ir, &[], 0).unwrap();
    let verts = (scene.vertices.len() / STRIDE) as u32;
    assert!(scene.triangles() > 0);
    assert!(scene.indices.iter().all(|&i| i < verts));
    assert!(scene.bounds.radius().is_finite());
}

#[test]
fn out_of_range_triangles_are_dropped_not_fatal() {
    let scene = build(&ir(vec![tri_model(vec![[0, 1, 2], [0, 1, 9]], 0)]), &[], 0).unwrap();
    assert_eq!(scene.indices, vec![0, 1, 2]);
    assert_eq!(scene.bounds.max, [2.0, 0.0, 4.0]);
}

#[test]
fn skin_and_body_choice_apply() {
    let models = vec![tri_model(vec![[0, 1, 2]], 0), tri_model(vec![[2, 1, 0]], 1)];
    let scene = build(&ir(models.clone()), &[1], 1).unwrap();
    assert_eq!(scene.indices, vec![2, 1, 0]);
    assert_eq!(scene.parts[0].material, 0);
    // An out-of-range body choice falls back to the first model.
    let scene = build(&ir(models), &[7], 0).unwrap();
    assert_eq!(scene.indices, vec![0, 1, 2]);
}

#[test]
fn empty_model_is_an_error() {
    assert!(build(&ModelIr::default(), &[], 0).is_err());
    assert!(build(&ir(vec![SubModel::default()]), &[], 0).is_err());
}
