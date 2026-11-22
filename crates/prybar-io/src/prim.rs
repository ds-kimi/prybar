/// Fixed-size little-endian scalar.
pub trait Primitive: Sized + Copy {
    const SIZE: usize;
    /// Callers guarantee `bytes` is exactly `SIZE` long.
    fn from_le(bytes: &[u8]) -> Self;
    fn to_le(self, out: &mut Vec<u8>);
}

macro_rules! primitive {
    ($($t:ty),*) => {$(
        impl Primitive for $t {
            const SIZE: usize = std::mem::size_of::<$t>();
            fn from_le(bytes: &[u8]) -> Self {
                let mut raw = [0u8; std::mem::size_of::<$t>()];
                raw.copy_from_slice(bytes);
                <$t>::from_le_bytes(raw)
            }
            fn to_le(self, out: &mut Vec<u8>) {
                out.extend_from_slice(&self.to_le_bytes());
            }
        }
    )*};
}

primitive!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);
