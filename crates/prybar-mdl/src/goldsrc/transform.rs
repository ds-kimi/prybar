use crate::ir::{Bone, Mat3x4, Quat, Vec3};

/// Model-space matrix of every bone. GoldSrc vertices are stored in bone
/// space while SMD wants model space. A parent that is not an earlier
/// bone (forged files) is treated as the root, so there are no cycles.
pub fn bone_matrices(bones: &[Bone]) -> Vec<Mat3x4> {
    let mut out: Vec<Mat3x4> = Vec::with_capacity(bones.len());
    for (i, b) in bones.iter().enumerate() {
        let local = local(b.quat, b.position);
        let parent = usize::try_from(b.parent).ok().filter(|&p| p < i);
        out.push(parent.map_or(local, |p| mul(&out[p], &local)));
    }
    out
}

fn local(q: Quat, p: Vec3) -> Mat3x4 {
    let [x, y, z, w] = q;
    [
        [
            1.0 - 2.0 * (y * y + z * z),
            2.0 * (x * y - w * z),
            2.0 * (x * z + w * y),
            p[0],
        ],
        [
            2.0 * (x * y + w * z),
            1.0 - 2.0 * (x * x + z * z),
            2.0 * (y * z - w * x),
            p[1],
        ],
        [
            2.0 * (x * z - w * y),
            2.0 * (y * z + w * x),
            1.0 - 2.0 * (x * x + y * y),
            p[2],
        ],
    ]
}

fn mul(a: &Mat3x4, b: &Mat3x4) -> Mat3x4 {
    let mut m = [[0.0; 4]; 3];
    for (i, row) in m.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            let t = if j == 3 { a[i][3] } else { 0.0 };
            *v = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j] + t;
        }
    }
    m
}

/// `w` is 1 for points and 0 for directions.
pub fn apply(m: &Mat3x4, v: Vec3, w: f32) -> Vec3 {
    let row = |r: &[f32; 4]| r[0] * v[0] + r[1] * v[1] + r[2] * v[2] + r[3] * w;
    [row(&m[0]), row(&m[1]), row(&m[2])]
}
