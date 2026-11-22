//! The three design directions under comparison.

/// Which design renders the shared state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignKind {
    Workbench,
    Canvas,
    Console,
}

impl DesignKind {
    pub const ALL: [DesignKind; 3] = [Self::Workbench, Self::Canvas, Self::Console];

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
        }
    }

    pub fn pitch(self) -> &'static str {
        match self {
            Self::Workbench => "Dense pro tool: panels, trees and tables at a glance.",
            Self::Canvas => "Friendly visual flow: big cards, one step at a time.",
            Self::Console => "Keyboard first and minimal: type, run, read.",
        }
    }
}
