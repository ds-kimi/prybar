use prybar_mdl::ir::{Mat3x4, Vec3};

/// Inverse of a rigid `poseToBone` matrix: bone space to model space.
pub fn bone_to_model(m: &Mat3x4, p: Vec3) -> Vec3 {
    let d = [p[0] - m[0][3], p[1] - m[1][3], p[2] - m[2][3]];
    [0, 1, 2].map(|c| m[0][c] * d[0] + m[1][c] * d[1] + m[2][c] * d[2])
}

/// Unit face normal; degenerate triangles get +Z so the line stays valid.
pub fn face_normal(t: &[Vec3; 3]) -> Vec3 {
    let a = [0, 1, 2].map(|c| t[1][c] - t[0][c]);
    let b = [0, 1, 2].map(|c| t[2][c] - t[0][c]);
    let n = [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ];
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > 1e-12 {
        n.map(|c| c / len)
    } else {
        [0.0, 0.0, 1.0]
    }
}
