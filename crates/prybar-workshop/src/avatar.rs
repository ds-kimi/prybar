//! A decoded account picture as Steam hands it over.

/// Straight RGBA, row by row, `width * height * 4` bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Avatar {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

impl Avatar {
    /// Refuses a buffer whose size does not match, so the UI never reads
    /// past the end of what Steam returned.
    pub fn new(width: u32, height: u32, rgba: Vec<u8>) -> Option<Avatar> {
        let want = u64::from(width) * u64::from(height) * 4;
        (width > 0 && height > 0 && rgba.len() as u64 == want).then_some(Avatar {
            width,
            height,
            rgba,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Avatar;

    #[test]
    fn refuses_a_short_buffer() {
        assert!(Avatar::new(2, 2, vec![0; 15]).is_none());
        assert!(Avatar::new(0, 0, Vec::new()).is_none());
        assert_eq!(
            Avatar::new(1, 1, vec![1, 2, 3, 4]).map(|a| a.rgba.len()),
            Some(4)
        );
    }
}
