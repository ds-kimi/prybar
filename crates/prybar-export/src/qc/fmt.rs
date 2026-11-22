use prybar_mdl::ir::Vec3;

/// Six decimals at most and no trailing zeros keep the QC diffable, and
/// non-finite values from hostile files print as 0 so studiomdl parses.
pub fn num(v: f32) -> String {
    if !v.is_finite() {
        return "0".to_owned();
    }
    let s = format!("{v:.6}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    match s {
        "-0" | "" => "0".to_owned(),
        s => s.to_owned(),
    }
}

pub fn vec3(v: Vec3) -> String {
    format!("{} {} {}", num(v[0]), num(v[1]), num(v[2]))
}

/// studiomdl's tokenizer has no escapes, so quotes and line breaks in a
/// name would end the token early; they are dropped.
pub fn quote(s: &str) -> String {
    let clean: String = s.chars().filter(|c| *c != '"' && !c.is_control()).collect();
    format!("\"{clean}\"")
}

/// Text-section values are untrusted; reparsing them stops a crafted
/// `.phy` from smuggling QC commands through a number field.
pub fn text_num(v: &str) -> String {
    num(v.trim().parse().unwrap_or(0.0))
}
