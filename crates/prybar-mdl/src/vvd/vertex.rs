use crate::ir::Vertex;
use crate::read::{vec3, Source};
use crate::Result;
use prybar_io::Cursor;

pub fn read(r: &Source, at: usize) -> Result<Vertex> {
    let mut c = Cursor::new(r.reader, at);
    let w: [f32; 3] = [c.read()?, c.read()?, c.read()?];
    let bones = c.bytes(3)?;
    let count = usize::from(c.read::<u8>()?).min(3);
    Ok(Vertex {
        weights: (0..count).map(|i| (bones[i], w[i])).collect(),
        position: vec3(&mut c)?,
        normal: vec3(&mut c)?,
        uv: [c.read()?, c.read()?],
    })
}
