use super::STRIDE;

/// Axis-aligned box over every finite vertex position.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            min: [-1.0; 3],
            max: [1.0; 3],
        }
    }
}

impl Bounds {
    /// Ignores NaN and infinite positions so one broken vertex cannot
    /// send the camera to infinity; falls back to a unit box.
    pub fn of(vertices: &[f32]) -> Self {
        let mut min = [f32::MAX; 3];
        let mut max = [f32::MIN; 3];
        for v in vertices.as_chunks::<STRIDE>().0 {
            if v[..3].iter().all(|c| c.is_finite() && c.abs() < 1e7) {
                for a in 0..3 {
                    min[a] = min[a].min(v[a]);
                    max[a] = max[a].max(v[a]);
                }
            }
        }
        if min[0] > max[0] {
            return Self::default();
        }
        Self { min, max }
    }

    pub fn center(&self) -> [f32; 3] {
        [0, 1, 2].map(|a| (self.min[a] + self.max[a]) * 0.5)
    }

    /// Half the diagonal, never zero so a flat or single-point model
    /// still gets a sane camera distance.
    pub fn radius(&self) -> f32 {
        let d: f32 = (0..3).map(|a| (self.max[a] - self.min[a]).powi(2)).sum();
        (d.sqrt() * 0.5).max(0.01)
    }
}
