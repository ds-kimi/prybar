//! Version-neutral model representation. Parsers fill it, exporters read it.

mod anim;
mod body;
mod flex;
mod geometry;
mod material;
mod physics;
mod procbone;
mod sequence;
mod skeleton;

pub use anim::{Animation, IkRule, Movement, Pose};
pub use body::{BodyPart, Mesh, SubModel};
pub use flex::{Eyeball, Flex, FlexController, FlexOp, FlexRule, VertexDelta};
pub use geometry::{Lod, LodMesh, Vertex};
pub use material::{IncludeModel, Material, Texture};
pub use physics::{ConvexPiece, KvBlock, PhysSolid, Physics};
pub use procbone::{AimAt, Jiggle, ProcBone, QuatTrigger};
pub use sequence::{AutoLayer, Event, IkLock, PoseParam, Sequence};
pub use skeleton::{Attachment, Bone, Hitbox, HitboxSet, IkChain};

use serde::Serialize;

pub type Vec3 = [f32; 3];
pub type Quat = [f32; 4];
/// Row-major 3x4 matrix, as Source stores bone and attachment transforms.
pub type Mat3x4 = [[f32; 4]; 3];

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct ModelIr {
    pub version: i32,
    pub name: String,
    pub checksum: i32,
    pub flags: i32,
    pub eye_position: Vec3,
    pub illum_position: Vec3,
    pub hull_min: Vec3,
    pub hull_max: Vec3,
    pub view_min: Vec3,
    pub view_max: Vec3,
    pub mass: f32,
    pub surface_prop: String,
    pub key_values: String,
    pub bones: Vec<Bone>,
    pub attachments: Vec<Attachment>,
    pub hitbox_sets: Vec<HitboxSet>,
    pub ik_chains: Vec<IkChain>,
    /// `$ikautoplaylock` entries, applied to every autoplay sequence.
    pub ik_auto_locks: Vec<IkLock>,
    pub body_parts: Vec<BodyPart>,
    pub materials: Vec<Material>,
    pub material_dirs: Vec<String>,
    /// One row per skin family, each mapping skin slot to material index.
    pub skins: Vec<Vec<u16>>,
    pub include_models: Vec<IncludeModel>,
    pub pose_params: Vec<PoseParam>,
    pub animations: Vec<Animation>,
    pub sequences: Vec<Sequence>,
    /// Flex desc names (FACS names on Valve faces).
    pub flex_descs: Vec<String>,
    pub flex_controllers: Vec<FlexController>,
    pub flex_rules: Vec<FlexRule>,
    /// Speech crossfade durations `(min, max)` that MDL 2531 stores and
    /// its QC sets with `$phonemefilter`; zero elsewhere.
    #[serde(skip_serializing_if = "is_zero_pair")]
    pub phoneme_filter: [f32; 2],
    /// GoldSrc embedded textures, from the model or its `T.mdl`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub textures: Vec<Texture>,
    /// Filled from the `.phy` file; `None` until loaded.
    pub physics: Option<Physics>,
    /// Sections the file holds but the parser could not recover; empty
    /// for every fully supported version.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

fn is_zero_pair(pair: &[f32; 2]) -> bool {
    *pair == [0.0; 2]
}
