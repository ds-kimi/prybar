use prybar_workshop::changelog::parse;

const PAGE: &str = r#"<div class="detailBox workshopAnnouncement noFooter changeLogCtn">
<div class="changelog headline">Update: 12 Mar, 2023 @ 3:04pm</div>
<p id="1678633440" >Fixed &amp; improved<br>crates <b>bigger</b></p></div>
<div class="detailBox workshopAnnouncement noFooter changeLogCtn">
<div class="changelog headline">Update: 1 Jan, 2023 @ 1:00am</div>
<p id="1672534800" ></p></div>"#;

#[test]
fn reads_entries_dates_and_plain_notes() {
    let all = parse(PAGE);
    assert_eq!(all.len(), 2);
    assert_eq!(all[0].time, Some(1_678_633_440));
    assert_eq!(all[0].headline, "Update: 12 Mar, 2023 @ 3:04pm");
    assert_eq!(all[0].note, "Fixed & improved\ncrates bigger");
    assert!(all[1].note.is_empty());
}

#[test]
fn hostile_pages_never_panic() {
    let cut: Vec<String> = (0..PAGE.len())
        .filter(|&i| PAGE.is_char_boundary(i))
        .map(|i| PAGE[..i].to_owned())
        .collect();
    for page in cut {
        let _ = parse(&page);
    }
    let weird = [
        "changelog headline",
        "changelog headline<p id=\"99999999999999999999\">x",
        "changelog headline>é</div><p id=\"\">€<</p>",
        "<script>changelog headline</script>",
    ];
    for page in weird {
        let all = parse(page);
        assert!(all.iter().all(|c| c.time.is_none()));
    }
    let many = "changelog headline>x</div>".repeat(2_000);
    assert_eq!(parse(&many).len(), 500);
    let long = format!(
        "changelog headline>h</div><p id=\"1\">{}</p>",
        "a".repeat(50_000)
    );
    assert_eq!(parse(&long)[0].note.len(), 8_000);
}
