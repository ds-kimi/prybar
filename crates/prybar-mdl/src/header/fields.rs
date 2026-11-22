//! Byte offsets of `studiohdr_t` fields (Source SDK 2013 `studio.h`).
//! Pairs are listed by their count field; the index field follows it.

pub const FLAGS: usize = 152;
pub const BONES: usize = 156;
pub const BONE_CONTROLLERS: usize = 164;
pub const HITBOX_SETS: usize = 172;
pub const LOCAL_ANIMS: usize = 180;
pub const LOCAL_SEQS: usize = 188;
pub const TEXTURES: usize = 204;
pub const CD_TEXTURES: usize = 212;
pub const SKIN_REF_COUNT: usize = 220;
pub const SKIN_FAMILY_COUNT: usize = 224;
pub const SKIN_INDEX: usize = 228;
pub const BODY_PARTS: usize = 232;
pub const ATTACHMENTS: usize = 240;
pub const FLEX_DESCS: usize = 260;
pub const FLEX_CONTROLLERS: usize = 268;
pub const FLEX_RULES: usize = 276;
pub const IK_CHAINS: usize = 284;
pub const POSE_PARAMS: usize = 300;
pub const SURFACE_PROP: usize = 308;
pub const KEY_VALUE_INDEX: usize = 312;
pub const KEY_VALUE_SIZE: usize = 316;
pub const IK_AUTOPLAY_LOCKS: usize = 320;
pub const MASS: usize = 328;
pub const INCLUDE_MODELS: usize = 336;
pub const ANIM_BLOCKS: usize = 352;
/// Only meaningful with [`VERT_ANIM_FIXED_POINT`] set in the flags.
pub const VERT_ANIM_SCALE: usize = 392;
pub const STUDIOHDR2_INDEX: usize = 400;

/// `STUDIOHDR_FLAGS_VERT_ANIM_FIXED_POINT_SCALE`.
pub const VERT_ANIM_FIXED_POINT: i32 = 1 << 21;
