/// Symmetry detection in metric patterns.
/// If room metrics have symmetry groups, the room is predictable and healthy.
/// Symmetry = stability.

/// A detected symmetry group in time series data.
#[derive(Debug, Clone, PartialEq)]
pub enum SymmetryGroup {
    /// No symmetry detected
    None,
    /// Mirror symmetry: pattern reflects around a center point
    Mirror { center_index: usize, strength: f64 },
    /// Translational symmetry: pattern repeats at intervals
    Translational { period: usize, strength: f64 },
    /// Rotational symmetry: values rotate with a shift
    Rotational { shift: usize, strength: f64 },
    /// Combined symmetries
    Combined { groups: Vec<Box<SymmetryGroup>>, overall_strength: f64 },
}

impl SymmetryGroup {
    /// How strong is the symmetry (0.0 = none, 1.0 = perfect).
    pub fn strength(&self) -> f64 {
        match self {
            SymmetryGroup::None => 0.0,
            SymmetryGroup::Mirror { strength, .. } => *strength,
            SymmetryGroup::Translational { strength, .. } => *strength,
            SymmetryGroup::Rotational { strength, .. } => *strength,
            SymmetryGroup::Combined { overall_strength, .. } => *overall_strength,
        }
    }

    /// Is this a meaningful symmetry (above noise threshold)?
    pub fn is_significant(&self, threshold: f64) -> bool {
        self.strength() >= threshold
    }
}

/// Symmetry detector for metric time series.
#[derive(Debug, Clone)]
pub struct SymmetryDetection {
    /// Threshold below which symmetry is considered noise
    noise_threshold: f64,
}

impl SymmetryDetection {
    pub fn new(noise_threshold: f64) -> Self {
        Self { noise_threshold }
    }

    /// Detect symmetry in a time series.
    pub fn detect_metric_symmetry(&self, time_series: &[f64]) -> SymmetryGroup {
        if time_series.len() < 4 {
            return SymmetryGroup::None;
        }

        // Try all symmetry types and pick the strongest
        let mirror = self.detect_mirror(time_series);
        let translational = self.detect_translational(time_series);
        let rotational = self.detect_rotational(time_series);

        let mirror_str = mirror.strength();
        let trans_str = translational.strength();
        let rot_str = rotational.strength();

        let max_str = mirror_str.max(trans_str).max(rot_str);

        if max_str < self.noise_threshold {
            return SymmetryGroup::None;
        }

        // If multiple strong symmetries, return combined
        let strong: Vec<Box<SymmetryGroup>> = [&mirror, &translational, &rotational]
            .iter()
            .filter(|s| s.strength() >= self.noise_threshold)
            .map(|s| Box::new((*s).clone()))
            .collect();

        if strong.len() > 1 {
            SymmetryGroup::Combined {
                overall_strength: max_str,
                groups: strong,
            }
        } else if mirror_str >= trans_str && mirror_str >= rot_str {
            mirror
        } else if trans_str >= rot_str {
            translational
        } else {
            rotational
        }
    }

    /// Detect mirror symmetry around center.
    fn detect_mirror(&self, data: &[f64]) -> SymmetryGroup {
        let center = data.len() / 2;
        let mut total_error = 0.0;
        let mut count = 0;

        for i in 0..center {
            let mirror_idx = data.len() - 1 - i;
            if mirror_idx > center && mirror_idx < data.len() {
                let range = (data[i] + data[mirror_idx]).abs().max(1e-10);
                total_error += ((data[i] - data[mirror_idx]).abs() / range).min(1.0);
                count += 1;
            }
        }

        let strength = if count > 0 {
            1.0 - (total_error / count as f64)
        } else {
            0.0
        };

        SymmetryGroup::Mirror {
            center_index: center,
            strength: strength.max(0.0),
        }
    }

    /// Detect translational symmetry (periodicity).
    fn detect_translational(&self, data: &[f64]) -> SymmetryGroup {
        let len = data.len();
        let _best_period = 1;
        let mut best_strength = 0.0;

        // Test periods from 2 to half the series length
        for period in 2..=(len / 2) {
            let mut total_error = 0.0;
            let mut count = 0;

            for i in 0..len {
                if i + period < len {
                    let range = (data[i] + data[i + period]).abs().max(1e-10);
                    total_error += ((data[i] - data[i + period]).abs() / range).min(1.0);
                    count += 1;
                }
            }

            if count > 0 {
                let strength = 1.0 - (total_error / count as f64);
                if strength > best_strength {
                    best_strength = strength;
                    // We'd capture best_period but borrow issues; recalculate below
                }
            }
        }

        // Find the best period again
        let mut best_period = 2;
        best_strength = 0.0;
        for period in 2..=(len / 2) {
            let mut total_error = 0.0;
            let mut count = 0;
            for i in 0..len {
                if i + period < len {
                    let range = (data[i] + data[i + period]).abs().max(1e-10);
                    total_error += ((data[i] - data[i + period]).abs() / range).min(1.0);
                    count += 1;
                }
            }
            if count > 0 {
                let strength = 1.0 - (total_error / count as f64);
                if strength > best_strength {
                    best_strength = strength;
                    best_period = period;
                }
            }
        }

        SymmetryGroup::Translational {
            period: best_period,
            strength: best_strength.max(0.0),
        }
    }

    /// Detect rotational symmetry (values shifted by a constant offset repeat).
    fn detect_rotational(&self, data: &[f64]) -> SymmetryGroup {
        let len = data.len();
        let mut best_shift = 1;
        let mut best_strength = 0.0;

        for shift in 1..=(len / 2) {
            let mut total_error = 0.0;
            let mut count = 0;

            for i in 0..len {
                let _j = (i + shift) % len;
                // Rotational symmetry: data[i] - data[j] should be roughly constant
                if i > 0 {
                    let diff_i = data[i] - data[(i + shift) % len];
                    let diff_prev = data[i - 1] - data[(i - 1 + shift) % len];
                    let range = (diff_i.abs() + diff_prev.abs()).max(1e-10);
                    total_error += ((diff_i - diff_prev).abs() / range).min(1.0);
                    count += 1;
                }
            }

            if count > 0 {
                let strength = 1.0 - (total_error / count as f64);
                if strength > best_strength {
                    best_strength = strength;
                    best_shift = shift;
                }
            }
        }

        SymmetryGroup::Rotational {
            shift: best_shift,
            strength: best_strength.max(0.0),
        }
    }

    /// Health score based on symmetry: more symmetry = healthier room.
    pub fn symmetry_health_score(&self, time_series: &[f64]) -> f64 {
        let symmetry = self.detect_metric_symmetry(time_series);
        symmetry.strength()
    }
}

impl Default for SymmetryDetection {
    fn default() -> Self {
        Self::new(0.3)
    }
}
