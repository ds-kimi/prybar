use prybar_tools::paths::{find_game_dir, steam_libraries};

/// Only test in this binary that touches env vars, so no race.
#[test]
fn env_override_and_vdf_libraries() {
    let tmp = tempfile::tempdir().unwrap();
    let steam = tmp.path().join("steam");
    let extra = tmp.path().join("extra");
    let game = extra.join("steamapps").join("common").join("FakeGame");
    std::fs::create_dir_all(steam.join("steamapps")).unwrap();
    std::fs::create_dir_all(&game).unwrap();
    let escaped = extra.display().to_string().replace('\\', "\\\\");
    let vdf = format!("\"libraryfolders\" {{ \"1\" {{ \"path\" \"{escaped}\" }} }}");
    std::fs::write(steam.join("steamapps").join("libraryfolders.vdf"), vdf).unwrap();
    std::env::set_var("PRYBAR_STEAM_DIR", &steam);
    let libs = steam_libraries();
    assert_eq!(libs[0], steam);
    assert!(libs.contains(&extra));
    assert_eq!(find_game_dir("FakeGame"), Some(game));
    assert_eq!(find_game_dir("Missing-Game"), None);
    std::env::remove_var("PRYBAR_STEAM_DIR");
}
