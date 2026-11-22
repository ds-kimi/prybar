use super::Vec3;
use serde::Serialize;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct FlexController {
    /// Group name such as `eyes` or `phoneme`.
    pub kind: String,
    pub name: String,
    pub min: f32,
    pub max: f32,
}

/// Weight of flex `flex` as a postfix expression over controllers.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct FlexRule {
    pub flex: i32,
    pub ops: Vec<FlexOp>,
}

/// `STUDIO_*` flex rule ops; the payload is a controller or flex index
/// except for `Const`, so the QC writer can print rules back as text.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum FlexOp {
    Const(f32),
    Fetch(i32),
    FetchFlex(i32),
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Exp,
    Open,
    Close,
    Comma,
    Max,
    Min,
    TwoWay0(i32),
    TwoWay1(i32),
    NWay(i32),
    Combo(i32),
    Dominate(i32),
    DmeLowerEyelid(i32),
    DmeUpperEyelid(i32),
    Unknown { op: i32, value: i32 },
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Eyeball {
    pub name: String,
    pub bone: i32,
    pub origin: Vec3,
    pub z_offset: f32,
    pub radius: f32,
    pub up: Vec3,
    pub forward: Vec3,
    pub texture: i32,
    pub iris_scale: f32,
    pub upper_flex_descs: [i32; 3],
    pub lower_flex_descs: [i32; 3],
    pub upper_targets: [f32; 3],
    pub lower_targets: [f32; 3],
    pub upper_lid_flex_desc: i32,
    pub lower_lid_flex_desc: i32,
    pub non_facs: bool,
}

/// One shape key of one mesh.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Flex {
    pub desc: i32,
    /// Right-hand flex desc of a stereo pair, 0 when mono.
    pub partner: i32,
    pub targets: [f32; 4],
    pub wrinkle: bool,
    #[serde(skip)]
    pub deltas: Vec<VertexDelta>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct VertexDelta {
    /// Vertex index relative to the mesh.
    pub index: u16,
    pub speed: u8,
    /// Left/right blend of a stereo flex, 0 = left and 255 = right.
    pub side: u8,
    pub position: Vec3,
    pub normal: Vec3,
    pub wrinkle: f32,
}
