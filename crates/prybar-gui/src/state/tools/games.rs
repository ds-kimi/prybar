//! The games config as the compile and settings screens see it.

use gpui::Context;
use prybar_service::tools::Game;

use crate::state::AppState;

impl AppState {
    /// Reads the config once, the first time a screen lists games.
    pub fn ensure_games(&mut self, cx: &mut Context<Self>) {
        if self.tools.games_loaded || self.is_running("Read games") {
            return;
        }
        let games = self.tools.backend.games;
        self.run_job(
            "Read games".into(),
            move || games(false),
            |s, list| {
                let n = list.len();
                s.set_games(list);
                format!("{n} {} configured", if n == 1 { "game" } else { "games" })
            },
            cx,
        );
    }

    /// Finds installs through Steam and saves them with the hand edits.
    pub fn detect_games(&mut self, cx: &mut Context<Self>) {
        let games = self.tools.backend.games;
        self.run_job(
            "Detect games".into(),
            move || games(true),
            |s, list| {
                let n = list.len();
                s.set_games(list);
                format!("Saved {n} {}", if n == 1 { "game" } else { "games" })
            },
            cx,
        );
    }

    /// Saves one game's folders; empty tool paths keep the old ones.
    pub fn save_game(&mut self, game: Game, cx: &mut Context<Self>) {
        let save = self.tools.backend.save;
        let name = game.name.clone();
        self.run_job(
            "Save game".into(),
            move || save(game),
            move |s, list| {
                s.set_games(list);
                format!("Saved {name}")
            },
            cx,
        );
    }

    /// A shorter list must not leave the pickers past its end.
    fn set_games(&mut self, list: Vec<Game>) {
        let t = &mut self.tools;
        t.games = list;
        t.games_loaded = true;
        if t.game >= t.games.len() {
            t.game = 0;
        }
        t.editing = t.editing.filter(|&i| i < t.games.len());
    }
}
