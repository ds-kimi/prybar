use proptest::prelude::*;
use prybar_update::{parse_sums, verify_signature, Release};

proptest! {
    #[test]
    fn release_json_never_panics(bytes in proptest::collection::vec(any::<u8>(), 0..512)) {
        let _ = Release::parse(&bytes);
    }

    #[test]
    fn sums_never_panic(text in ".{0,400}") {
        let _ = parse_sums(&text);
        let _ = verify_signature(text.as_bytes(), &text, &text);
    }
}

#[test]
fn sums_reject_paths_and_bad_hex() {
    let h = "a".repeat(64);
    assert_eq!(
        parse_sums(&format!("{h}  prybar.zip\n{h} *b.tar.gz\n"))
            .expect("ok")
            .len(),
        2
    );
    assert!(parse_sums(&format!("{h}  ../evil")).is_err());
    assert!(parse_sums(&format!("{}  x", "g".repeat(64))).is_err());
    assert!(parse_sums("abc  x").is_err());
}

#[test]
fn release_rejects_drafts_and_odd_tags() {
    let draft = br#"{"tag_name":"v1.0.0","draft":true}"#;
    assert!(Release::parse(draft).is_err());
    assert!(Release::parse(br#"{"tag_name":"latest"}"#).is_err());
    assert!(Release::parse(
        br#"{"tag_name":"v1.2.3","assets":[{"name":"a","browser_download_url":"u"}]}"#
    )
    .is_ok());
}
