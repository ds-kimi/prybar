use proptest::prelude::*;
use prybar_tools::config::{load, save, Config, Game, Packer};

fn game() -> impl Strategy<Value = Game> {
    let packer = prop::option::of(prop_oneof![Just(Packer::Gma), Just(Packer::Vpk)]);
    let path = "[a-zA-Z0-9 _./-]{1,40}";
    (
        "[a-z0-9_]{1,16}",
        path,
        prop::option::of(path),
        prop::option::of(path),
        packer,
    )
        .prop_map(|(name, dir, s, h, packer)| Game {
            name,
            game_dir: dir.into(),
            studiomdl: s.map(Into::into),
            hlmv: h.map(Into::into),
            packer,
        })
}

proptest! {
    #[test]
    fn save_load_round_trip(games in prop::collection::vec(game(), 0..6)) {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("config.toml");
        let cfg = Config { games };
        save(&path, &cfg).unwrap();
        prop_assert_eq!(load(&path).unwrap(), cfg);
    }
}
