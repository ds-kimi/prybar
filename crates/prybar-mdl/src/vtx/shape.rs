//! Field offsets of the VTX headers that differ between v7 (v44+ models)
//! and v107 (Vampire: Bloodlines, MDL 2531). The tree walk is shared.

use crate::read::Source;
use crate::Result;

#[derive(Debug, Clone, Copy)]
pub struct Shape {
    /// v107 stores mesh, strip group and strip counts as i16.
    pub narrow: bool,
    pub mesh_stride: usize,
    /// Vertex count, vertex offset, index count, index offset, strip
    /// count, strip offset, flags.
    pub group: [usize; 7],
    pub group_stride: usize,
    pub vert_stride: usize,
    /// Where the mesh-relative vertex id sits inside a vertex record.
    pub vert_id: usize,
    /// Index count, first index, flags.
    pub strip: [usize; 3],
    pub strip_stride: usize,
    /// Group flag for static props, whose vertices are bare u16 ids.
    pub static_flag: u8,
}

pub const V7: Shape = Shape {
    narrow: false,
    mesh_stride: 9,
    group: [0, 4, 8, 12, 16, 20, 24],
    group_stride: 25,
    vert_stride: 9,
    vert_id: 4,
    strip: [0, 4, 18],
    strip_stride: 27,
    static_flag: 0,
};

/// Three bone ids per vertex and unpacked, aligned headers.
pub const V107: Shape = Shape {
    narrow: true,
    mesh_stride: 8,
    group: [0, 8, 2, 12, 4, 16, 6],
    group_stride: 20,
    vert_stride: 12,
    vert_id: 10,
    strip: [0, 2, 9],
    strip_stride: 16,
    static_flag: 0x10,
};

/// Reads a count or index field in the width the version uses.
pub fn int(r: &Source, at: usize, narrow: bool) -> Result<i32> {
    Ok(if narrow {
        r.read::<i16>(at)?.into()
    } else {
        r.read(at)?
    })
}
