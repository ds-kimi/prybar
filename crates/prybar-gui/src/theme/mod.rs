//! Colors. The chooser stays neutral so no design wins by its frame;
//! each design gets its own palette, refined in its own lot.

use gpui::{rgb, Rgba};

use crate::state::DesignKind;

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
    let base = chooser();
    match kind {
        DesignKind::Workbench => Palette {
            bg: rgb(0x14171c),
            accent: rgb(0x4c9ff0),
            ..base
        },
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
            bg: rgb(0x0c0d0f),
            surface: rgb(0x0c0d0f),
            accent: rgb(0x7fd99a),
            ..base
        },
    }
}
