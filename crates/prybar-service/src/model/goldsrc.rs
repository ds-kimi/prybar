use crate::source::Origin;
use prybar_mdl::{Companions, ModelIr};

/// GoldSrc sequence groups are numbered `01`..`99`; the first gap ends
/// the set, as in the engine.
const MAX_GROUPS: usize = 99;

/// GoldSrc keeps textures in `<stem>T.mdl` and sequence groups in
/// `<stem>NN.mdl` when the model was compiled with them split out.
pub fn load(origin: Origin, stem: &str, mdl: &[u8]) -> prybar_mdl::Result<ModelIr> {
    let get = |suffix: &str| origin.get(stem, suffix).ok().flatten();
    let textures = get("T.mdl").or_else(|| get("t.mdl"));
    let groups: Vec<Vec<u8>> = (1..=MAX_GROUPS)
        .map_while(|i| get(&format!("{i:02}.mdl")))
        .collect();
    let refs: Vec<&[u8]> = groups.iter().map(Vec::as_slice).collect();
    let comp = Companions {
        textures: textures.as_deref(),
        groups: &refs,
    };
    prybar_mdl::parse_goldsrc(mdl, comp)
}
