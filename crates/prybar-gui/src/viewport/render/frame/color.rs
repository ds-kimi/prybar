use crate::viewport::render::Request;

/// Theme colors arrive as sRGB; the sRGB target expects linear values.
pub fn linear(c: [f32; 4]) -> [f32; 4] {
    let ch = |v: f32| {
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    [ch(c[0]), ch(c[1]), ch(c[2]), c[3]]
}

/// The `Globals` uniform, laid out as both shaders declare it.
pub fn globals(req: &Request) -> Vec<u8> {
    let aspect = req.width as f32 / req.height.max(1) as f32;
    let m = req.camera.view_proj(aspect).to_cols_array();
    let eye = req.camera.eye();
    let normals = if req.flags.normals { 1.0 } else { 0.0 };
    let wire = linear(req.wire);
    let all = m
        .into_iter()
        .chain([eye.x, eye.y, eye.z, 1.0, normals, 0.0, 0.0, 0.0])
        .chain(wire);
    all.flat_map(f32::to_le_bytes).collect()
}

/// Line vertices: position then RGBA, little-endian floats.
pub fn lines(points: &[[f32; 3]], color: [f32; 4]) -> Vec<u8> {
    let flat: Vec<f32> = points
        .iter()
        .flat_map(|p| p.iter().chain(&color).copied())
        .collect();
    floats(&flat)
}

pub fn floats(v: &[f32]) -> Vec<u8> {
    v.iter().flat_map(|f| f.to_le_bytes()).collect()
}
