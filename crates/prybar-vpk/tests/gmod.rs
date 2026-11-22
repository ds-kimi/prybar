use prybar_vpk::Vpk;

use prybar_testkit::gmod_dir_vpk;

#[test]
fn lists_and_reads_real_gmod_vpk() {
    let Some(path) = gmod_dir_vpk() else {
        eprintln!("skipped: garrysmod_dir.vpk not found");
        return;
    };
    let vpk = Vpk::open(&path).unwrap();
    assert!(vpk.entries.len() > 1000);
    let small = vpk.entries.iter().filter(|e| e.size() < 4096).take(50);
    for e in small {
        vpk.read(e).unwrap();
    }
}
