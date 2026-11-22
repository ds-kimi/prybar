use super::builder::{float, put, text};

/// One part, one model with three points on bone 1, one strip of three.
pub fn body(b: &mut [u8], (part, model, mesh, tris): (usize, usize, usize, usize)) {
    text(b, part, "body");
    for (i, v) in [1, 1, model as i32].into_iter().enumerate() {
        put(b, part + 64 + i * 4, v);
    }
    model_records(b, model, mesh);
    put(b, mesh, 1);
    put(b, mesh + 4, tris as i32);
    let corners: [i16; 13] = [3, 0, 0, 0, 0, 1, 0, 2, 0, 2, 0, 0, 2];
    for (i, v) in corners.into_iter().enumerate() {
        b[tris + i * 2..tris + i * 2 + 2].copy_from_slice(&v.to_le_bytes());
    }
}

/// Points (1,0,0), (0,1,0), (0,0,1) and one +Z normal, all on bone 1.
fn model_records(b: &mut [u8], model: usize, mesh: usize) {
    text(b, model, "box_ref");
    let (verts, info, norms, ninfo) = (model + 112, model + 148, model + 152, model + 164);
    for (i, v) in [
        0,
        0,
        1,
        mesh as i32,
        3,
        info as i32,
        verts as i32,
        1,
        ninfo as i32,
        norms as i32,
    ]
    .into_iter()
    .enumerate()
    {
        put(b, model + 64 + i * 4, v);
    }
    for (i, x) in [1.0, 0.0, 0.0].into_iter().enumerate() {
        float(b, verts + i * 12, x);
    }
    float(b, verts + 16, 1.0);
    float(b, verts + 32, 1.0);
    b[info..info + 3].fill(1);
    b[ninfo] = 1;
    float(b, norms + 8, 1.0);
}
