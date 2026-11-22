use glam::{Mat4, Quat, Vec3};
use prybar_mdl::anim::root_to_smd;
use prybar_mdl::ir::{Bone, Mat3x4, Pose};

/// Bone-to-model matrices for one frame. Only an earlier bone counts as
/// a parent, so a hostile parent chain cannot loop or index out of range.
pub fn world(bones: &[Bone], frame: &[Pose]) -> Vec<Mat4> {
    let mut out: Vec<Mat4> = Vec::with_capacity(bones.len());
    for (i, b) in bones.iter().enumerate() {
        let parent = parent(b, i);
        let pose = match frame.get(i) {
            // Root frames are stored in the engine's animation space;
            // the bind pose is in model space. Motion is left out so the
            // model plays in place under the camera.
            Some(p) if parent.is_none() => root_to_smd(*p, ([0.0; 3], 0.0)),
            Some(p) => *p,
            None => Pose {
                position: b.position,
                quat: b.quat,
            },
        };
        let local = local(pose);
        out.push(parent.map_or(local, |p| out[p] * local));
    }
    out
}

pub fn parent(b: &Bone, i: usize) -> Option<usize> {
    usize::try_from(b.parent).ok().filter(|&p| p < i)
}

fn local(p: Pose) -> Mat4 {
    let q = Quat::from_array(p.quat);
    let q = if q.is_finite() && q.length_squared() > 1e-8 {
        q.normalize()
    } else {
        Quat::IDENTITY
    };
    let t = Vec3::from_array(p.position);
    Mat4::from_rotation_translation(q, if t.is_finite() { t } else { Vec3::ZERO })
}

/// `pose_to_bone` is stored as three rows of a 3x4 matrix.
pub fn inverse_bind(m: &Mat3x4) -> Mat4 {
    let col = |c: usize| [m[0][c], m[1][c], m[2][c], if c == 3 { 1.0 } else { 0.0 }];
    Mat4::from_cols_array_2d(&[col(0), col(1), col(2), col(3)])
}
