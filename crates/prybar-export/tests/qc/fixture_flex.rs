use prybar_mdl::ir::*;

pub fn controllers() -> Vec<FlexController> {
    let c = |kind: &str, name: &str, min| FlexController {
        kind: kind.into(),
        name: name.into(),
        min,
        max: 1.0,
    };
    vec![
        c("phoneme", "smile", 0.0),
        c("eyes", "blink", 0.0),
        c("default", "lr", -1.0),
    ]
}

/// One rule per op family, plus a hostile one (underflow, unknown op,
/// huge combo count) that must degrade instead of panicking.
pub fn rules() -> Vec<FlexRule> {
    use FlexOp::*;
    let rule = |flex, ops| FlexRule { flex, ops };
    vec![
        rule(0, vec![Fetch(0), Const(2.0), Fetch(2), Sub, Mul]),
        rule(1, vec![Fetch(0), Fetch(1), Max, Neg]),
        rule(
            2,
            vec![TwoWay0(2), TwoWay1(2), Combo(2), Fetch(1), Dominate(1)],
        ),
        rule(2, vec![Add, Unknown { op: 99, value: 0 }, Combo(1000)]),
    ]
}

fn kv(name: &str, pairs: &[(&str, &str)]) -> KvBlock {
    let pairs = pairs.iter().map(|(k, v)| (k.to_string(), v.to_string()));
    KvBlock {
        name: name.into(),
        pairs: pairs.collect(),
    }
}

/// Ragdoll with one injection attempt in a number field.
pub fn physics() -> Physics {
    let piece = || ConvexPiece {
        bone: 0,
        triangles: vec![[[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]],
    };
    let solid = |n| PhysSolid {
        pieces: vec![piece(); n],
    };
    let root = [
        ("index", "0"),
        ("name", "root"),
        ("damping", "0.1"),
        (
            "rotdamping",
            "1}
$evil",
        ),
    ];
    let head = [("index", "1"), ("name", "head"), ("inertia", "2")];
    let joint = [
        ("parent", "0"),
        ("child", "1"),
        ("xmin", "-30"),
        ("xmax", "30"),
        ("xfriction", "0.5"),
    ];
    let blocks = vec![
        kv("solid", &root),
        kv("solid", &head),
        kv("ragdollconstraint", &joint),
        kv("editparams", &[("rootname", "root"), ("totalmass", "90")]),
        kv("collisionrules", &[("selfcollisions", "0")]),
    ];
    Physics {
        solids: vec![solid(1), solid(2)],
        blocks,
        ..Physics::default()
    }
}
