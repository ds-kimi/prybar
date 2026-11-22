use crate::metrics::Metrics;

/// studiomdl welds duplicate vertices and splits seams differently from
/// the shipped compiler build, so counts drift a little on real models.
const VERTEX_TOLERANCE: f64 = 0.02;

/// Every way `rebuilt` differs from `original`; empty means a pass.
pub fn diff(original: &Metrics, rebuilt: &Metrics) -> Vec<String> {
    let seqs = |m: &Metrics| -> Vec<String> {
        m.sequences
            .iter()
            .map(|s| format!("{}:{}", s.0, s.1))
            .collect()
    };
    let mut out = Vec::new();
    list(&mut out, "bones", &original.bones, &rebuilt.bones);
    if !close(original.vertices, rebuilt.vertices) {
        out.push(format!(
            "vertices: {} -> {}",
            original.vertices, rebuilt.vertices
        ));
    }
    list(&mut out, "sequences", &seqs(original), &seqs(rebuilt));
    if original.flexes != rebuilt.flexes {
        out.push(format!("flexes: {} -> {}", original.flexes, rebuilt.flexes));
    }
    list(
        &mut out,
        "materials",
        &original.materials,
        &rebuilt.materials,
    );
    out
}

fn close(a: usize, b: usize) -> bool {
    let (a, b) = (a as f64, b as f64);
    (a - b).abs() <= a.max(b) * VERTEX_TOLERANCE
}

/// Names the first differing pair, so a report line points at the
/// culprit instead of dumping both lists.
fn list(out: &mut Vec<String>, what: &str, a: &[String], b: &[String]) {
    if a == b {
        return;
    }
    let at = a.iter().zip(b).position(|(x, y)| x != y);
    let gap = match at {
        Some(i) => format!("#{i} {:?} vs {:?}", a[i], b[i]),
        None => "one list is a prefix of the other".to_owned(),
    };
    out.push(format!("{what}: {} -> {} ({gap})", a.len(), b.len()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_vertex_drift_passes_but_renames_fail() {
        let m = |bones: &[&str], vertices| Metrics {
            bones: bones.iter().map(|b| b.to_string()).collect(),
            vertices,
            sequences: vec![("idle".into(), 1)],
            flexes: 0,
            materials: vec!["a".into()],
        };
        assert!(diff(&m(&["root"], 1000), &m(&["root"], 1015)).is_empty());
        assert_eq!(diff(&m(&["root"], 1000), &m(&["root"], 1100)).len(), 1);
        let renamed = diff(&m(&["root"], 10), &m(&["hip"], 10));
        assert!(renamed[0].contains("\"root\" vs \"hip\""), "{renamed:?}");
    }
}
