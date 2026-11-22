use crate::header::{fields, Header};
use crate::read::Source;
use crate::{detect, parse, Family, ModelIr, Result};
use serde::Serialize;

/// Short, serializable overview of a model, sized for an agent to read
/// without dumping the whole IR.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Summary {
    pub version: i32,
    pub family: Family,
    pub name: String,
    pub checksum: i32,
    pub bones: usize,
    pub attachments: usize,
    pub hitboxes: usize,
    pub body_parts: usize,
    pub models: usize,
    pub meshes: usize,
    pub materials: Vec<String>,
    pub material_dirs: Vec<String>,
    pub skin_families: usize,
    pub include_models: Vec<String>,
    pub sequences: i32,
    pub animations: i32,
    pub flex_controllers: i32,
    pub pose_parameters: i32,
    /// What the parser could not recover (legacy versions only).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

pub fn inspect(data: &[u8]) -> Result<Summary> {
    let version = detect(data)?;
    let ir = parse(data)?;
    if matches!(version.family, Family::SourceLegacy | Family::GoldSrc) {
        let mut s = summarize(&ir, version.family);
        s.sequences = ir.sequences.len() as i32;
        s.animations = ir.animations.len() as i32;
        return Ok(s);
    }
    let hdr = Header::parse(&Source::new(data))?;
    let mut s = summarize(&ir, version.family);
    s.sequences = hdr.int(fields::LOCAL_SEQS);
    s.animations = hdr.int(fields::LOCAL_ANIMS);
    s.flex_controllers = hdr.int(fields::FLEX_CONTROLLERS);
    s.pose_parameters = hdr.int(fields::POSE_PARAMS);
    Ok(s)
}

fn summarize(ir: &ModelIr, family: Family) -> Summary {
    let models = ir.body_parts.iter().flat_map(|p| &p.models);
    Summary {
        version: ir.version,
        family,
        name: ir.name.clone(),
        checksum: ir.checksum,
        bones: ir.bones.len(),
        attachments: ir.attachments.len(),
        hitboxes: ir.hitbox_sets.iter().map(|s| s.hitboxes.len()).sum(),
        body_parts: ir.body_parts.len(),
        models: models.clone().count(),
        meshes: models.map(|m| m.meshes.len()).sum(),
        materials: ir.materials.iter().map(|m| m.name.clone()).collect(),
        material_dirs: ir.material_dirs.clone(),
        skin_families: ir.skins.len(),
        include_models: ir.include_models.iter().map(|i| i.path.clone()).collect(),
        sequences: 0,
        animations: 0,
        flex_controllers: 0,
        pose_parameters: 0,
        notes: ir.notes.clone(),
    }
}
