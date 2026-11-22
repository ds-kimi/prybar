use prybar_mdl::ir::ModelIr;

/// Bone segments in bind pose. `pose_to_bone` maps model space into bone
/// space, so a bone sits at `-R^T t`; this avoids walking the hierarchy
/// and cannot loop on a hostile parent chain.
pub fn bones(ir: &ModelIr) -> Vec<[[f32; 3]; 2]> {
    let at: Vec<[f32; 3]> = ir.bones.iter().map(|b| origin(&b.pose_to_bone)).collect();
    ir.bones
        .iter()
        .enumerate()
        .filter_map(|(i, b)| {
            let parent = usize::try_from(b.parent).ok()?;
            let seg = [*at.get(parent)?, at[i]];
            seg.iter().flatten().all(|c| c.is_finite()).then_some(seg)
        })
        .collect()
}

fn origin(m: &[[f32; 4]; 3]) -> [f32; 3] {
    [0, 1, 2].map(|c| -(0..3).map(|r| m[r][c] * m[r][3]).sum::<f32>())
}
