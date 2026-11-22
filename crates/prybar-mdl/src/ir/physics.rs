use super::Vec3;
use serde::Serialize;

/// Contents of a `.phy` file.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Physics {
    pub checksum: i32,
    pub solids: Vec<PhysSolid>,
    /// Raw text section (solid params, constraints, collision rules).
    pub text: String,
    pub blocks: Vec<KvBlock>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct PhysSolid {
    pub pieces: Vec<ConvexPiece>,
}

/// One convex hull of a solid.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct ConvexPiece {
    /// Bone stored by studiomdl in the ledge client data, -1 when unset.
    pub bone: i32,
    /// Triangles in engine units, in the space of the solid's bone.
    #[serde(skip)]
    pub triangles: Vec<[Vec3; 3]>,
}

/// A `name { "key" "value" ... }` block of the text section.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct KvBlock {
    pub name: String,
    pub pairs: Vec<(String, String)>,
}

impl KvBlock {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.pairs
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}
