use prybar_vpk::Vpk;

use prybar_testkit::gmod_dir_vpk;

fn decompile(vpk: &Vpk, stem: &str) -> Option<Vec<prybar_export::smd::SmdFile>> {
    let read = |ext: &str| {
        let path = format!("{stem}{ext}");
        let e = vpk.entries.iter().find(|e| e.path == path)?;
        Some(vpk.read(e).unwrap())
    };
    let (mdl, vvd, vtx) = (read(".mdl")?, read(".vvd")?, read(".dx90.vtx")?);
    let mut ir = prybar_mdl::parse(&mdl).unwrap();
    let vvd = prybar_mdl::parse_vvd(&vvd).unwrap();
    let vtx = prybar_mdl::parse_vtx(&vtx).unwrap();
    prybar_mdl::load_geometry(&mut ir, &vvd, &vtx).unwrap();
    Some(prybar_export::smd::mesh_files(&ir))
}

#[test]
fn decompiles_real_gmod_meshes() {
    let Some(path) = gmod_dir_vpk() else {
        eprintln!("skipped: garrysmod_dir.vpk not found");
        return;
    };
    let vpk = Vpk::open(&path).unwrap();
    let stems = vpk
        .entries
        .iter()
        .filter_map(|e| e.path.strip_suffix(".mdl"));
    let done = stems.filter_map(|s| decompile(&vpk, s)).take(20).count();
    assert_eq!(done, 20);
}
