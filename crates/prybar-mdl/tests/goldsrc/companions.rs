use super::builder;
use prybar_mdl::{parse, parse_goldsrc, Companions};

#[test]
fn v10_external_textures_and_groups() {
    let mut buf = builder::build();
    let tex = buf.clone();
    builder::put(&mut buf, 45 * 4, 0);
    builder::put(&mut buf, 49 * 4, 0);
    let bare = parse(&buf).unwrap();
    assert!(bare.textures.is_empty());
    let comp = Companions {
        textures: Some(&tex),
        groups: &[],
    };
    let full = parse_goldsrc(&buf, comp).unwrap();
    assert_eq!(full.textures.len(), 1);
    assert_eq!(full.skins, [vec![0u16]]);
}

#[test]
fn v10_external_sequence_groups() {
    let mut buf = builder::build();
    let tex = buf.clone();
    builder::put(&mut buf, 32 + 1748 + 31 * 4, 1);
    builder::put(&mut buf, 43 * 4, 2);
    let grouped = parse(&buf).unwrap();
    assert!(grouped.animations[0].frames.is_empty());
    assert!(grouped
        .notes
        .iter()
        .any(|n| n.starts_with("sequence groups")));
    let comp = Companions {
        textures: None,
        groups: &[&tex],
    };
    assert_eq!(
        parse_goldsrc(&buf, comp).unwrap().animations[0]
            .frames
            .len(),
        2
    );
}

#[test]
fn v14_keeps_skeleton_and_notes_meshes() {
    let mut buf = builder::build();
    builder::put(&mut buf, 4, 14);
    let ir = parse(&buf).unwrap();
    assert_eq!((ir.version, ir.bones.len(), ir.sequences.len()), (14, 2, 1));
    assert!(ir.body_parts.is_empty() && ir.textures.is_empty());
    assert!(ir
        .notes
        .iter()
        .any(|n| n.starts_with("meshes, textures: v14")));
}
