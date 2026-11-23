//! File sizes as people read them, instead of raw byte counts.

/// `912 B`, `16.1 KB`, `3.4 MB`, `1.2 GB` (powers of 1024, one decimal).
pub fn human(n: u64) -> String {
    const UNITS: [&str; 4] = ["KB", "MB", "GB", "TB"];
    if n < 1024 {
        return format!("{n} B");
    }
    let mut v = n as f64 / 1024.0;
    let mut unit = 0;
    while v >= 1024.0 && unit + 1 < UNITS.len() {
        v /= 1024.0;
        unit += 1;
    }
    format!("{v:.1} {}", UNITS[unit])
}

#[cfg(test)]
mod tests {
    #[test]
    fn picks_the_unit() {
        assert_eq!(super::human(53), "53 B");
        assert_eq!(super::human(16_463), "16.1 KB");
        assert_eq!(super::human(5 << 20), "5.0 MB");
        assert_eq!(super::human(u64::MAX), "16777216.0 TB");
    }
}
