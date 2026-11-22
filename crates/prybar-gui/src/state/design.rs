//! The three design directions under comparison.

/// Which design renders the shared state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignKind {
    Workbench,
    Canvas,
    Console,
    Prybar,
}

impl DesignKind {
    pub const ALL: [DesignKind; 4] = [Self::Workbench, Self::Canvas, Self::Console, Self::Prybar];

    /// Reads the `--design` value; `None` for anything unknown.
    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|d| d.name().eq_ignore_ascii_case(name))
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Workbench => "Workbench",
            Self::Canvas => "Canvas",
            Self::Console => "Console",
            Self::Prybar => "Prybar",
        }
    }

    pub fn pitch(self) -> &'static str {
        match self {
            Self::Workbench => "Dense pro tool: panels, trees and tables at a glance.",
            Self::Canvas => "Friendly visual flow: big cards, one step at a time.",
            Self::Console => "Keyboard first and minimal: type, run, read.",
            Self::Prybar => "Workbench calm with a top bar, dark or light.",
        }
    }
}
