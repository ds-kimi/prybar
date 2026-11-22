use prybar_tools::config::{detect_games, game_from_install, KNOWN_GAMES};
use prybar_tools::paths::gmod_dir;

#[test]
fn fake_install_needs_gameinfo_and_finds_tools() {
    let tmp = tempfile::tempdir().unwrap();
    let gmod = &KNOWN_GAMES[0];
    assert!(game_from_install(gmod, tmp.path()).is_none());
    let content = tmp.path().join(gmod.content);
    std::fs::create_dir_all(content.join("..").join("bin").join("win64")).unwrap();
    std::fs::create_dir_all(&content).unwrap();
    std::fs::write(content.join("gameinfo.txt"), "GameInfo {}").unwrap();
    std::fs::write(tmp.path().join("bin/win64/hlmv.exe"), "").unwrap();
    let game = game_from_install(gmod, tmp.path()).unwrap();
    assert_eq!(game.game_dir, content);
    assert!(game.studiomdl.is_none());
    assert_eq!(game.hlmv.unwrap(), tmp.path().join("bin/win64/hlmv.exe"));
}

/// Done-criterion: the local Garry's Mod is detected.
#[test]
fn detects_local_gmod_when_installed() {
    let Some(root) = gmod_dir() else {
        eprintln!("skip: no Garry's Mod install found (set PRYBAR_GMOD_DIR)");
        return;
    };
    let games = detect_games();
    let game = games.iter().find(|g| g.name == "garrysmod");
    let game = game.expect("GarrysMod installed but not detected");
    assert!(game.game_dir.starts_with(&root));
    eprintln!("detected: {game:?}");
}
