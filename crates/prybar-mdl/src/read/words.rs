use super::Source;
use crate::ir::Vec3;
use crate::Result;

/// A fixed struct read as raw 32-bit words, so wide Source structs can be
/// mapped field by field without one long sequential read per field.
pub struct Words(Vec<i32>);

impl Words {
    pub fn read(r: &Source, at: usize, count: usize) -> Result<Self> {
        Ok(Self(r.array(at, count, count)?))
    }

    pub fn i(&self, n: usize) -> i32 {
        self.0.get(n).copied().unwrap_or(0)
    }

    pub fn f(&self, n: usize) -> f32 {
        f32::from_bits(self.i(n) as u32)
    }

    pub fn v3(&self, n: usize) -> Vec3 {
        [self.f(n), self.f(n + 1), self.f(n + 2)]
    }
}
