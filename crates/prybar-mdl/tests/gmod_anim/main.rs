mod checks;

use checks::{check_bind_euler, find, frame0_vs_bind};
use prybar_mdl::ModelIr;
use prybar_testkit::gmod_dir_vpk;
use prybar_vpk::Vpk;

/// Parses every model of the VPK with its `.ani` when one exists.
fn models(vpk: &Vpk) -> Vec<(String, ModelIr)> {
    let read = |path: &str| {
        let e = vpk.entries.iter().find(|e| e.path == path)?;
        Some(vpk.read(e).unwrap())
    };
    let stems = vpk
        .entries
        .iter()
        .filter_map(|e| e.path.strip_suffix(".mdl"));
    stems
        .map(|stem| {
            let mdl = read(&format!("{stem}.mdl")).unwrap();
            let ani = read(&format!("{stem}.ani"));
            let ir = prybar_mdl::parse_with_ani(&mdl, ani.as_deref())
                .unwrap_or_else(|e| panic!("{stem}: {e}"));
            (stem.to_owned(), ir)
        })
        .collect()
}

/// Static props are compiled from their reference mesh, so frame 0 of
/// their idle anim is usually the bind pose. Some are not (`$origin`
/// shifts, posed doors, physics anims), hence ratios. Group03 male
/// ragdoll pose is exactly the bind pose below the root, which checks the
/// compressed streams of a full skeleton.
#[test]
fn decodes_real_gmod_animations() {
    let Some(vpk) = gmod_dir_vpk().map(|p| Vpk::open(&p).unwrap()) else {
        return eprintln!("skipped: garrysmod_dir.vpk not found");
    };
    let (mut idles, mut rot, mut pos, mut frames, mut ragdolls) = (0, 0, 0, 0, 0);
    for (stem, ir) in models(&vpk) {
        check_bind_euler(&stem, &ir);
        for a in ir.animations.iter().filter(|a| !a.frames.is_empty()) {
            assert_eq!(a.frames.len(), a.frame_count, "{stem} {}", a.name);
            frames += a.frame_count;
        }
        if let Some(a) = find(&ir, "@idle").filter(|_| stem.starts_with("models/props")) {
            let (r, p) = frame0_vs_bind(&ir, a, true);
            (idles, rot, pos) = (idles + 1, rot + usize::from(r), pos + usize::from(p));
        }
        if let Some(a) =
            find(&ir, "@ragdoll").filter(|_| stem.contains("/group03") && stem.contains("/male_0"))
        {
            assert_eq!(frame0_vs_bind(&ir, a, false), (true, true), "{stem}");
            ragdolls += 1;
        }
    }
    eprintln!("idle frame 0 vs bind on {idles} props: rot {rot}, pos {pos}; {ragdolls} ragdolls; {frames} frames");
    assert!(idles > 100 && rot * 100 >= idles * 95, "rot {rot}/{idles}");
    assert!(
        pos * 100 >= idles * 85 && ragdolls > 0 && frames > 0,
        "pos {pos}/{idles}"
    );
}
