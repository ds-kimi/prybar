use prybar_vpk::Vpk;

use prybar_testkit::gmod_dir_vpk;

#[test]
fn inspects_real_gmod_models() {
    let Some(path) = gmod_dir_vpk() else {
        eprintln!("skipped: garrysmod_dir.vpk not found");
        return;
    };
    let vpk = Vpk::open(&path).unwrap();
    let models = vpk.entries.iter().filter(|e| e.path.ends_with(".mdl"));
    for e in models.take(20) {
        let data = vpk.read(e).unwrap();
        let s = prybar_mdl::inspect(&data).unwrap_or_else(|err| panic!("{}: {err}", e.path));
        assert!(s.bones > 0, "{} has no bones", e.path);
    }
}
