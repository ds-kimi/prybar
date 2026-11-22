//! Per-version layout of the pre-v44 Source formats. The table walk is
//! shared; only header slots and struct strides move between versions.

/// Which `mstudiobone_t` shape the version stores.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoneShape {
    /// v32-v37: euler rotation, qAlignment, then the quat after the tail.
    V37,
    /// v27-v31: no qAlignment, quat or contents.
    V31,
    /// 2531 (Vampire: Bloodlines): quat in place of the euler angles.
    V2531,
}

/// Header slots are indices into the int run that follows the vectors.
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub name_len: usize,
    /// 2531 carries an unknown vector between illum and hull min.
    pub extra_vector: bool,
    pub flags: usize,
    pub bones: usize,
    pub hitbox_sets: usize,
    pub anims: usize,
    pub seqs: usize,
    pub textures: usize,
    /// Skin ref count; family count and skin index follow it.
    pub skins: usize,
    pub body_parts: usize,
    pub attachments: Option<usize>,
    pub flex_ctrls: usize,
    pub pose_params: usize,
    pub surface_prop: usize,
    pub mass: Option<usize>,
    pub bone: BoneShape,
    pub texture_stride: usize,
    pub model_stride: usize,
    pub model_name_len: usize,
    /// False when hitboxes are 32-byte records with no name offset.
    pub hitbox_named: bool,
}

/// v32-v37 share the header; v27-v31 slot an old hitbox pair in after the
/// bone controllers, which pushes every later slot two ints down.
pub fn for_version(version: i32) -> Layout {
    match version {
        2531 => V2531,
        27..=31 => shifted(V37, 2),
        _ => V37,
    }
}

fn shifted(base: Layout, by: usize) -> Layout {
    let s = |v: usize| v + by;
    Layout {
        hitbox_sets: s(base.hitbox_sets),
        anims: s(base.anims),
        seqs: s(base.seqs),
        textures: s(base.textures),
        skins: s(base.skins),
        body_parts: s(base.body_parts),
        attachments: None,
        flex_ctrls: s(base.flex_ctrls),
        pose_params: s(base.pose_params),
        surface_prop: s(base.surface_prop),
        mass: base.mass.map(s),
        bone: BoneShape::V31,
        model_stride: 120,
        hitbox_named: false,
        ..base
    }
}

const V37: Layout = Layout {
    name_len: 64,
    extra_vector: false,
    flags: 0,
    bones: 1,
    hitbox_sets: 5,
    anims: 7,
    seqs: 13,
    textures: 18,
    skins: 22,
    body_parts: 25,
    attachments: Some(27),
    flex_ctrls: 33,
    pose_params: 41,
    surface_prop: 43,
    mass: Some(48),
    bone: BoneShape::V37,
    texture_stride: 32,
    model_stride: 140,
    model_name_len: 64,
    hitbox_named: true,
};

const V2531: Layout = Layout {
    name_len: 128,
    extra_vector: true,
    flags: 0,
    bones: 3,
    hitbox_sets: 7,
    anims: 9,
    seqs: 11,
    textures: 16,
    skins: 20,
    body_parts: 23,
    attachments: Some(25),
    flex_ctrls: 31,
    pose_params: 39,
    surface_prop: 41,
    mass: None,
    bone: BoneShape::V2531,
    texture_stride: 20,
    model_stride: 224,
    model_name_len: 128,
    hitbox_named: false,
};
