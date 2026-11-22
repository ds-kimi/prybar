use crate::ir::{Mat3x4, Quat, Vec3};
use crate::Result;
use prybar_io::Cursor;

pub fn vec3(c: &mut Cursor) -> Result<Vec3> {
    Ok([c.read()?, c.read()?, c.read()?])
}

pub fn quat(c: &mut Cursor) -> Result<Quat> {
    Ok([c.read()?, c.read()?, c.read()?, c.read()?])
}

pub fn mat3x4(c: &mut Cursor) -> Result<Mat3x4> {
    let mut m = [[0.0; 4]; 3];
    for row in &mut m {
        for v in row.iter_mut() {
            *v = c.read()?;
        }
    }
    Ok(m)
}
