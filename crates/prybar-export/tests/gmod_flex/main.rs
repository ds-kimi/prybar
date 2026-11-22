mod checks;
mod load;

use checks::{flex_count, inside, mesh_box, vta_frames};
use load::{model, stems};
use prybar_testkit::gmod_dir_vpk;
use prybar_vpk::Vpk;

/// Citizen faces (player group01-03) are the flexed models GMod ships.
#[test]
fn decompiles_real_face_flexes() {
    let Some(vpk) = gmod_dir_vpk().map(|p| Vpk::open(&p).unwrap()) else {
        return eprintln!("skipped: garrysmod_dir.vpk not found");
    };
    let (mut faces, mut frames, mut vrds) = (0, 0, 0);
    for stem in stems(&vpk, ".mdl") {
        let Some(ir) = model(&vpk, stem).filter(|ir| !ir.flex_descs.is_empty()) else {
            continue;
        };
        let vtas = prybar_export::vta::vta_files(&ir);
        let flexed = ir.body_parts.iter().flat_map(|p| &p.models);
        let flexed: Vec<_> = flexed
            .filter(|m| m.meshes.iter().any(|x| !x.flexes.is_empty()))
            .collect();
        assert_eq!(vtas.len(), flexed.len(), "{stem}");
        for (vta, m) in vtas.iter().zip(&flexed) {
            assert_eq!(
                vta_frames(&vta.text),
                flex_count(&ir, m) + 1,
                "{stem} {}",
                vta.name
            );
            frames += flex_count(&ir, m);
        }
        faces += usize::from(stem.contains("/group0") && !vtas.is_empty());
        vrds += usize::from(prybar_export::vrd::vrd(&ir).is_some_and(|v| v.contains("<helper>")));
    }
    eprintln!("{faces} citizen faces, {frames} flex frames, {vrds} VRDs");
    assert!(faces > 0 && frames > 0 && vrds > 0);
}

/// Static props store their single solid in model space, ragdolls one
/// solid per bone in bone space; both must land on the model.
#[test]
fn parses_real_physics() {
    let Some(vpk) = gmod_dir_vpk().map(|p| Vpk::open(&p).unwrap()) else {
        return eprintln!("skipped: garrysmod_dir.vpk not found");
    };
    let (mut props, mut fit, mut ragdolls, mut rag_fit, mut pieces) = (0, 0, 0, 0, 0);
    for stem in stems(&vpk, ".phy") {
        let Some(ir) = model(&vpk, stem) else {
            continue;
        };
        let Some(phy) = &ir.physics else { continue };
        assert!(phy.solids.iter().all(|s| !s.pieces.is_empty()), "{stem}");
        pieces += phy.solids.iter().map(|s| s.pieces.len()).sum::<usize>();
        let smd = prybar_export::smd::physics_smd(&ir).unwrap();
        if phy.solids.len() == 1 {
            props += 1;
            fit += usize::from(inside((ir.hull_min, ir.hull_max), &smd) > 0.99);
        } else {
            ragdolls += 1;
            rag_fit += usize::from(inside(mesh_box(&ir), &smd) > 0.95);
        }
    }
    eprintln!("{props} props ({fit} in hull), {ragdolls} ragdolls ({rag_fit} in mesh box), {pieces} pieces");
    assert!(props > 100 && fit * 100 >= props * 95);
    assert!(ragdolls > 0 && rag_fit * 100 >= ragdolls * 90);
}
