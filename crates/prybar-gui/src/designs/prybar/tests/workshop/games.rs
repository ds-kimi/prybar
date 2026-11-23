//! The game chips: the page follows the picked game, and with no
//! supported game installed it says what to do instead of listing.

use gpui::TestAppContext;

use super::super::details::click;
use super::super::window;
use crate::state::Screen;

#[gpui::test]
fn picking_a_game_switches_the_page(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    state.update(&mut cx, |s, cx| s.navigate(Screen::Publish, cx));
    cx.run_until_parked();
    assert!(cx.debug_bounds("pb-game-0").is_some(), "chips drawn");
    state.read_with(&cx, |s, _| assert_eq!(s.tools.steam.game, Some(4000)));
    click(&mut cx, "pb-game-1");
    state.read_with(&cx, |s, _| {
        assert_eq!(s.tools.steam.game, Some(440));
        assert_eq!(s.tools.steam.target().map(|g| g.id), Some("tf2"));
    });
    assert!(cx.debug_bounds("pb-item-0").is_some(), "tf2 items listed");
}

#[gpui::test]
fn no_installed_game_shows_how_to_add_one(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    state.update(&mut cx, |s, cx| {
        s.tools.backend.targets = || prybar_service::tools::WorkshopGames {
            detected: Vec::new(),
            supported: prybar_workshop::games::WORKSHOP_GAMES.to_vec(),
            unsupported: Vec::new(),
        };
        // The badge asked Steam at start; ask again with no game.
        s.refresh_steam(cx);
        s.navigate(Screen::Publish, cx)
    });
    cx.run_until_parked();
    assert!(cx.debug_bounds("pb-games-empty").is_some(), "empty state");
    assert!(cx.debug_bounds("pb-item-0").is_none(), "no grid");
    click(&mut cx, "pb-games-settings");
    state.read_with(&cx, |s, _| assert_eq!(s.screen, Screen::Settings));
}
