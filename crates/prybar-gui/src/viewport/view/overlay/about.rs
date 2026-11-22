//! What each viewport control does, for its tooltip.

pub fn about(id: &str) -> &'static str {
    match id {
        "pb-vp-wire" => "Draw the triangle edges over the shaded model (W)",
        "pb-vp-bones" => "Show the skeleton: one line from each bone to its parent (B)",
        "pb-vp-normals" => "Show which way each vertex faces, to spot flipped normals",
        "pb-vp-backdrop" => "Swap the background for a neutral grey, to judge colors",
        "pb-vp-frame" => "Fit the whole model in view again (F)",
        "pb-vp-full" => "Show the 3D view alone in the window (Shift-F)",
        "pb-vp-play" => "Play or pause the sequence",
        "pb-vp-speed" => "Change the playback speed",
        "pb-vp-skin" => "Show the next skin family of the model",
        "pb-vp-lod" => "Show the next level of detail (lower poly version)",
        _ if id.starts_with("pb-vp-body") => "Show the next choice of this body group",
        _ => "Change what the 3D view shows",
    }
}
