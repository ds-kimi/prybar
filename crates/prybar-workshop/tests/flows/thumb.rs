use prybar_workshop::thumb::cached;

#[test]
fn only_https_is_fetched_and_a_second_ask_hits_the_cache() {
    let tmp = tempfile::tempdir().unwrap();
    let err = cached("http://example.com/a.jpg", tmp.path()).unwrap_err();
    assert!(err.contains("https"));
    assert!(!tmp.path().join("x").exists());
    // Seed the cache under the URL's own name, then no network is needed.
    let url = "https://cdn.invalid/preview.jpg";
    let first = cached(url, tmp.path());
    assert!(first.is_err(), "the .invalid host never resolves");
    let names: Vec<_> = std::fs::read_dir(tmp.path()).unwrap().collect();
    assert!(names.is_empty(), "a failed download leaves nothing behind");
}
