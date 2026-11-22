use std::path::Path;

use prybar_tools::compile::compile;
use prybar_tools::config::{game_from_install, KNOWN_GAMES};
use prybar_tools::paths::gmod_dir;
use prybar_tools::runner::Runner;

/// Our own one-triangle model, so no game asset is involved.
const SMD: &str = "version 1\nnodes\n0 \"root\" -1\nend\nskeleton\ntime 0\n\
0 0 0 0 0 0 0\nend\ntriangles\nprybar_tri\n\
0 0 0 0 0 0 1 0 0\n0 10 0 0 0 0 1 1 0\n0 0 10 0 0 0 1 0 1\nend\n";
const QC: &str = "$modelname \"prybar_test/tri.mdl\"\n$cdmaterials \"models/prybar_test\"\n\
$body \"body\" \"tri.smd\"\n$sequence \"idle\" \"tri.smd\"\n";

fn write_sources(dir: &Path) {
    std::fs::write(dir.join("tri.smd"), SMD).unwrap();
    std::fs::write(dir.join("tri.qc"), QC).unwrap();
    // A scratch game dir keeps the compiled model out of the real install.
    std::fs::create_dir_all(dir.join("game")).unwrap();
    let info = "\"GameInfo\"\n{\n game \"prybar\"\n FileSystem\n {\n  SearchPaths\n  {\n   Game |gameinfo_path|.\n  }\n }\n}\n";
    std::fs::write(dir.join("game").join("gameinfo.txt"), info).unwrap();
}

/// Done-criterion: a synthetic QC compiles with GMod studiomdl.
#[test]
fn compiles_one_triangle_with_gmod_studiomdl() {
    let tool = gmod_dir().and_then(|r| game_from_install(&KNOWN_GAMES[0], &r)?.studiomdl);
    let Some(studiomdl) = tool.filter(|_| cfg!(windows)) else {
        eprintln!("skip: no GMod studiomdl.exe (or not Windows; Wine run not tested here)");
        return;
    };
    let tmp = tempfile::tempdir().unwrap();
    write_sources(tmp.path());
    let game = tmp.path().join("game");
    let qc = tmp.path().join("tri.qc");
    let report = compile(&Runner::Native, &studiomdl, &game, &qc).unwrap();
    assert!(report.success, "{}", report.log);
    let mdl = game.join("models").join("prybar_test").join("tri.mdl");
    assert!(mdl.is_file(), "no output at {}", mdl.display());
    eprintln!("compiled, {} warnings", report.warnings.len());
}
