//! Z96 classification and heavy/light distribution

pub const R96: usize = 96;

/// Classify a byte mod 96
pub fn classify_byte_mod96(b: usize) -> usize {
    b % R96
}

/// Calculate distribution of bytes across 96 classes
pub fn z96_distribution() -> Vec<usize> {
    let mut counts = vec![0; R96];
    for b in 0..256 {
        counts[classify_byte_mod96(b)] += 1;
    }
    counts
}

/// Return classes that occur 3 times (heavy) vs 2 times (light)
pub fn heavy_vs_light_classes() -> (Vec<usize>, Vec<usize>) {
    let dist = z96_distribution();
    let mut heavy = Vec::new();
    let mut light = Vec::new();
    for (i, &count) in dist.iter().enumerate() {
        if count == 3 {
            heavy.push(i);
        } else if count == 2 {
            light.push(i);
        }
    }
    (heavy, light)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribution_counts() {
        let (heavy, light) = heavy_vs_light_classes();
        assert_eq!(heavy.len(), 64);
        assert_eq!(light.len(), 32);
        assert_eq!(heavy.len() * 3 + light.len() * 2, 256);
    }
}
