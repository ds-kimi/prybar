mod support;

use prybar_vtf::{Error, Vtf};

#[test]
fn rejects_bad_headers() {
    assert_eq!(Vtf::parse(b"nope".to_vec()).unwrap_err(), Error::BadMagic);
    assert_eq!(Vtf::parse(b"VTF\0".to_vec()).unwrap_err(), Error::Truncated);
    assert_eq!(
        Vtf::parse(support::vtf(9, 0, (1, 1), 1, &[0; 4])).unwrap_err(),
        Error::BadVersion(9)
    );
    assert_eq!(
        Vtf::parse(support::vtf(5, 7, (1, 1), 1, &[0])).unwrap_err(),
        Error::Format(7)
    );
    assert!(matches!(
        Vtf::parse(support::vtf(5, 0, (0, 4), 1, &[])),
        Err(Error::Size(0, 4))
    ));
}

#[test]
fn truncated_data_is_an_error() {
    let vtf = Vtf::parse(support::vtf(5, 13, (8, 8), 1, &[0; 8])).unwrap();
    assert_eq!(vtf.decode(0).unwrap_err(), Error::Truncated);
}

#[test]
fn huge_mip_count_does_not_overflow() {
    let vtf = Vtf::parse(support::vtf(2, 13, (8192, 8192), 255, &[0; 64])).unwrap();
    assert!(vtf.decode(0).is_err());
    // Smallest mips are stored first, so the 1x1 tail is readable.
    assert_eq!(vtf.decode(254).unwrap().width, 1);
}

#[test]
fn every_prefix_parses_or_errors() {
    let full = support::vtf(3, 15, (4, 4), 1, &[7; 16]);
    for n in 0..full.len() {
        if let Ok(v) = Vtf::parse(full[..n].to_vec()) {
            let _ = v.decode(0);
        }
    }
}
