//! Colors. The chooser stays neutral so no design wins by its frame;
//! each design gets its own palette, refined in its own lot.

pub mod prybar;

use gpui::{rgb, Rgba};

use crate::state::{AppState, DesignKind};

#[derive(Clone, Copy)]
pub struct Palette {
    pub bg: Rgba,
    pub surface: Rgba,
    pub hover: Rgba,
    pub active: Rgba,
    pub border: Rgba,
    pub text: Rgba,
    pub muted: Rgba,
    pub accent: Rgba,
    pub danger: Rgba,
}

/// The console's monospace face, one that ships with each OS.
pub const MONO: &str = if cfg!(target_os = "macos") {
    "Menlo"
} else if cfg!(target_os = "windows") {
    "Consolas"
} else {
    "DejaVu Sans Mono"
};

/// Neutral greys for the chooser.
pub fn chooser() -> Palette {
    Palette {
        bg: rgb(0x16181d),
        surface: rgb(0x20232a),
        hover: rgb(0x2a2e37),
        active: rgb(0x343945),
        border: rgb(0x323743),
        text: rgb(0xe4e7ec),
        muted: rgb(0x8b93a1),
        accent: rgb(0xb8c0cc),
        danger: rgb(0xe5675f),
    }
}

pub fn for_design(kind: DesignKind) -> Palette {
    match kind {
        DesignKind::Workbench => workbench(false),
        DesignKind::Prybar => prybar::palette(false),
        DesignKind::Canvas => Palette {
            bg: rgb(0xf4f1ec),
            surface: rgb(0xffffff),
            hover: rgb(0xf0ebe3),
            active: rgb(0xe6dfd4),
            border: rgb(0xe2dcd2),
            text: rgb(0x2b2622),
            muted: rgb(0x8a8077),
            accent: rgb(0xe0733f),
            danger: rgb(0xc8453c),
        },
        DesignKind::Console => Palette {
            bg: rgb(0x0b0c0e),
            surface: rgb(0x121417),
            hover: rgb(0x1b1e22),
            active: rgb(0x252930),
            border: rgb(0x2a2e35),
            text: rgb(0xf2f4f5),
            muted: rgb(0x7f8791),
            accent: rgb(0x7fd99a),
            danger: rgb(0xff6b6b),
        },
    }
}

/// Graphite with the logo's orange as the only accent; the light variant
/// keeps the same roles so every view reads the same in both.
pub fn workbench(light: bool) -> Palette {
    if light {
        return Palette {
            bg: rgb(0xf7f7f8),
            surface: rgb(0xeeeff1),
            hover: rgb(0xe3e5e8),
            active: rgb(0xd8dbdf),
            border: rgb(0xd6d8dc),
            text: rgb(0x22252a),
            muted: rgb(0x6a707a),
            accent: rgb(0xd9692a),
            danger: rgb(0xc9423a),
        };
    }
    Palette {
        bg: rgb(0x1b1d21),
        surface: rgb(0x222429),
        hover: rgb(0x2b2e34),
        active: rgb(0x33373e),
        border: rgb(0x2f3238),
        text: rgb(0xd7dade),
        muted: rgb(0x7f8590),
        accent: rgb(0xf0883e),
        danger: rgb(0xe5675f),
    }
}

/// The palette the window frame uses for the state's design and variant.
pub fn current(state: &AppState) -> Palette {
    match state.design {
        None => chooser(),
        Some(DesignKind::Workbench) => workbench(state.light),
        Some(DesignKind::Prybar) => prybar::palette(state.light),
        Some(kind) => for_design(kind),
    }
}
