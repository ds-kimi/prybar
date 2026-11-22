use crate::qc::sanitize::ident;
use prybar_mdl::ir::SubModel;

/// studiomdl only uses eyeball names to bind eyelids and does not store
/// them, so names are made up; on Valve faces the right eye is first and
/// turned by a positive angle.
pub fn eye_name(model: &SubModel, i: usize) -> String {
    let Some(eye) = model.eyeballs.get(i) else {
        return format!("eye_{i}");
    };
    if !eye.name.is_empty() {
        return ident(&eye.name);
    }
    match (i, eye.z_offset) {
        (0, z) if z > 0.0 => "eye_right".to_owned(),
        (1, z) if z < 0.0 => "eye_left".to_owned(),
        _ => format!("eye_{i}"),
    }
}
