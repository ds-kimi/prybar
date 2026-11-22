//! What each texture control does, for its tooltip.

pub fn texture(id: &str) -> &'static str {
    match id {
        "pb-tex-mip-prev" => "Show a larger mip level",
        "pb-tex-mip-next" => "Show a smaller mip level, as the game uses at a distance",
        "pb-tex-fit" => "Fit the whole texture in view again",
        _ if id.contains("alpha") => "Show only the alpha channel, where masks and cutouts live",
        _ if id.contains("rgb") => "Show the colors without transparency",
        _ => "Change how the texture is shown",
    }
}
