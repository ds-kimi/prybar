use prybar_tools::config::{load, save, Config, Game, Packer};

fn sample() -> Config {
    Config {
        games: vec![Game {
            name: "garrysmod".into(),
            game_dir: "games/GarrysMod/garrysmod".into(),
            studiomdl: Some("games/GarrysMod/bin/studiomdl.exe".into()),
            hlmv: None,
            packer: Some(Packer::Gma),
        }],
    }
}

#[test]
fn save_then_load_round_trips() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("sub").join("config.toml");
    save(&path, &sample()).unwrap();
    assert_eq!(load(&path).unwrap(), sample());
}

#[test]
fn missing_file_is_empty_and_garbage_is_an_error() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("config.toml");
    assert_eq!(load(&path).unwrap(), Config::default());
    std::fs::write(&path, "games = 3\n[[[").unwrap();
    assert!(load(&path).is_err());
}

#[test]
fn merge_keeps_hand_edited_entries_and_lookup_ignores_case() {
    let mut cfg = sample();
    let mut other = sample().games;
    other[0].game_dir = "elsewhere".into();
    other.push(Game {
        name: "tf2".into(),
        ..other[0].clone()
    });
    cfg.merge(other);
    assert_eq!(cfg.games.len(), 2);
    let gmod = cfg.game("GarrysMod").unwrap();
    assert_eq!(gmod.game_dir, sample().games[0].game_dir);
}
