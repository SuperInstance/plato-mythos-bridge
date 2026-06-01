/// Quipu encoding for tile compression — encode tile values as hierarchical knot sequences.
/// Inspired by Incan quipu: data as knotted cords with checksum validation.

/// A single knot in a quipu cord.
#[derive(Debug, Clone, PartialEq)]
pub enum Knot {
    /// Single knot representing a digit 0-9
    Digit(u8),
    /// A branch point leading to sub-cords
    Branch(CordTree),
}

/// A cord tree: hierarchical representation of tile values.
#[derive(Debug, Clone, PartialEq)]
pub struct CordTree {
    /// The knots on this cord (in order)
    pub knots: Vec<Knot>,
    /// Checksum for corruption detection
    pub checksum: u64,
}

impl CordTree {
    pub fn new(knots: Vec<Knot>) -> Self {
        let mut cord = Self { knots, checksum: 0 };
        cord.checksum = cord.compute_checksum();
        cord
    }

    fn compute_checksum(&self) -> u64 {
        // Simple hash: sum of digit values with positional weighting
        let mut hash: u64 = 0;
        Self::hash_knots(&self.knots, &mut hash, 1);
        hash
    }

    fn hash_knots(knots: &[Knot], hash: &mut u64, depth: u64) {
        for (i, knot) in knots.iter().enumerate() {
            match knot {
                Knot::Digit(d) => {
                    *hash = hash.wrapping_add((*d as u64).wrapping_mul((i as u64 + 1) * depth * 31));
                }
                Knot::Branch(sub) => {
                    Self::hash_knots(&sub.knots, hash, depth + 1);
                }
            }
        }
    }

    /// Validate the cord tree against its checksum.
    pub fn validate(&self) -> bool {
        self.compute_checksum() == self.checksum
    }

    /// Count total knots including branches.
    pub fn knot_count(&self) -> usize {
        let mut count = 0;
        self.count_recursive(&mut count);
        count
    }

    fn count_recursive(&self, count: &mut usize) {
        for knot in &self.knots {
            *count += 1;
            if let Knot::Branch(sub) = knot {
                sub.count_recursive(count);
            }
        }
    }
}

/// Quipu tile encoder/decoder.
#[derive(Debug, Clone)]
pub struct QuipuTile {
    precision: u8, // decimal places to encode
}

impl QuipuTile {
    pub fn new(precision: u8) -> Self {
        Self { precision }
    }

    /// Encode tile values into a quipu cord tree.
    pub fn encode_tiles(&self, tiles: &[f64]) -> CordTree {
        let knots: Vec<Knot> = tiles
            .iter()
            .map(|&v| {
                let scaled = (v * 10f64.powi(self.precision as i32)).round() as i64;
                let digits = self.value_to_knots(scaled.abs(), if scaled < 0 { Some(true) } else { None });
                if digits.len() == 1 {
                    digits.into_iter().next().unwrap()
                } else {
                    Knot::Branch(CordTree::new(digits))
                }
            })
            .collect();
        CordTree::new(knots)
    }

    /// Decode a quipu cord tree back to tile values.
    pub fn decode_tiles(&self, cord: &CordTree) -> Result<Vec<f64>, String> {
        if !cord.validate() {
            return Err("Checksum mismatch: cord tree is corrupted".into());
        }
        let mut values = Vec::new();
        for knot in &cord.knots {
            let v = self.knot_to_value(knot)?;
            values.push(v);
        }
        Ok(values)
    }

    fn value_to_knots(&self, value: i64, _negative: Option<bool>) -> Vec<Knot> {
        if value == 0 {
            return vec![Knot::Digit(0)];
        }
        let mut digits = Vec::new();
        let mut v = value;
        while v > 0 {
            digits.push(Knot::Digit((v % 10) as u8));
            v /= 10;
        }
        digits.reverse();
        digits
    }

    fn knot_to_value(&self, knot: &Knot) -> Result<f64, String> {
        match knot {
            Knot::Digit(d) => Ok(*d as f64 / 10f64.powi(self.precision as i32)),
            Knot::Branch(sub) => {
                let mut value: f64 = 0.0;
                for knot in &sub.knots {
                    match knot {
                        Knot::Digit(d) => {
                            value = value * 10.0 + *d as f64;
                        }
                        Knot::Branch(_) => return Err("Nested branches not supported in decode".into()),
                    }
                }
                Ok(value / 10f64.powi(self.precision as i32))
            }
        }
    }
}

impl Default for QuipuTile {
    fn default() -> Self {
        Self::new(1)
    }
}
