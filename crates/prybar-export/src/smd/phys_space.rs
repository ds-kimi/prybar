use super::phys_math::bone_to_model;
use prybar_mdl::ir::{KvBlock, Mat3x4, Vec3};

/// How a solid's vertices reach model space.
pub enum Space {
    Pose(Mat3x4),
    Bind(Mat3x4),
    Model,
}

impl Space {
    pub fn pick(pose: Option<Mat3x4>, bind: Option<Mat3x4>) -> Self {
        match (pose, bind) {
            (Some(p), _) => Space::Pose(p),
            (None, Some(b)) => Space::Bind(b),
            (None, None) => Space::Model,
        }
    }

    pub fn apply(&self, v: Vec3) -> Vec3 {
        match self {
            Space::Pose(m) => pose_to_model(m, v),
            Space::Bind(m) => bone_to_model(m, v),
            Space::Model => v,
        }
    }
}

/// Reference pose of a solid from its `origin` and `angles` text keys
/// (VTMB writes them when the collision was authored in another pose than
/// the MDL bind). Both keys must parse to finite values: a partial pose
/// cannot be trusted, so the caller falls back to the bind.
pub fn solid_pose(block: &KvBlock) -> Option<Mat3x4> {
    let vec = |k: &str| -> Option<[f64; 3]> {
        let f: Vec<f64> = block
            .get(k)?
            .split_whitespace()
            .map(|t| t.parse().ok())
            .collect::<Option<_>>()?;
        let v: [f64; 3] = f.try_into().ok()?;
        v.iter().all(|c| c.is_finite()).then_some(v)
    };
    let (o, a) = (vec("origin")?, vec("angles")?);
    let [p, y, r] = a.map(f64::to_radians);
    let (sp, cp, sy, cy, sr, cr) = (p.sin(), p.cos(), y.sin(), y.cos(), r.sin(), r.cos());
    let m = [
        [
            cp * cy,
            sr * sp * cy - cr * sy,
            cr * sp * cy + sr * sy,
            o[0],
        ],
        [
            cp * sy,
            sr * sp * sy + cr * cy,
            cr * sp * sy - sr * cy,
            o[1],
        ],
        [-sp, sr * cp, cr * cp, o[2]],
    ];
    Some(m.map(|row| row.map(|c| c as f32)))
}

/// Solid space to model space through an explicit reference pose.
fn pose_to_model(m: &Mat3x4, p: Vec3) -> Vec3 {
    [0, 1, 2].map(|r| m[r][0] * p[0] + m[r][1] * p[1] + m[r][2] * p[2] + m[r][3])
}
