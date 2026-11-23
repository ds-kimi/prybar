//! Stand-ins for studiomdl, Steam and the games config, so flows run
//! in tests without tools, network or the user's real config.

use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use prybar_service::tools::Game;
use prybar_tools::compile::CompileReport;
use prybar_workshop::spec::Visibility;
use prybar_workshop::{ListedItem, Progress, PublishReport, SteamStatus};

use crate::state::tools::Backend;

pub(crate) fn backend() -> Backend {
    Backend {
        compile,
        publish: |spec, dry, progress| {
            progress(&Progress {
                stage: "uploading".into(),
                done: 1,
                total: 2,
            });
            Ok(PublishReport {
                dry_run: dry,
                item_id: spec.item_id.or((!dry).then_some(42)),
                created: spec.item_id.is_none(),
                needs_legal_agreement: false,
            })
        },
        games: |detect| Ok(games(if detect { 3 } else { 2 })),
        save: |g| Ok(vec![g]),
        status: |_| SteamStatus::Ready {
            user: "tester".into(),
        },
        items: |_| Ok(vec![item()]),
        addon: |dir| prybar_workshop::addon::check_addon(dir).map_err(|e| e.to_string()),
        avatar: |_| prybar_workshop::Avatar::new(2, 2, vec![200; 16]),
        thumb: |url| match url {
            "https://bad" => Err("no".into()),
            _ => Ok(PathBuf::from(url)),
        },
    }
}

pub(crate) fn item() -> ListedItem {
    ListedItem {
        item_id: 7,
        title: "Crate pack".into(),
        tags: vec!["Addon".into()],
        time_updated: 1_700_000_000,
        time_created: 1_600_000_000,
        visibility: Visibility::Public,
        subscriptions: 12,
        favorites: 3,
        votes_up: 9,
        votes_down: 1,
        preview_url: None,
        ..Default::default()
    }
}

/// Prints three lines; a game named "Broken" fails the compile.
fn compile(
    game: &Game,
    _: &Path,
    on_line: &mut dyn FnMut(&str),
    _: &AtomicBool,
) -> Result<CompileReport, String> {
    let log = [
        "Processing",
        "WARNING: no weights",
        "writing /g/models/x.mdl:",
    ];
    log.iter().for_each(|l| on_line(l));
    Ok(CompileReport {
        success: game.name != "Broken",
        exit_code: Some(0),
        errors: vec![],
        warnings: vec![],
        log: log.join("\n"),
    })
}

pub(crate) fn games(n: usize) -> Vec<Game> {
    ["Garry's Mod", "Team Fortress 2", "Broken"][..n]
        .iter()
        .map(|name| Game {
            name: (*name).into(),
            game_dir: PathBuf::from("/games").join(name),
            studiomdl: Some("/games/bin/studiomdl.exe".into()),
            hlmv: None,
            packer: None,
        })
        .collect()
}
