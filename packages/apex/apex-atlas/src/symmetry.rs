//! (Z/2)^11 action via commuting bit-flips

use crate::group::{split_p, join_p, P_MOD, B_MOD};

/// Gray code on 11 bits
pub fn gray11(i: usize) -> usize {
    (i & 0x7FF) ^ ((i & 0x7FF) >> 1)
}

/// Reverse Gray code on 11 bits
pub fn ungray11(mut g: usize) -> usize {
    g &= 0x7FF;
    let mut x = g;
    let mut s = 1;
    while (g >> s) > 0 {
        x ^= g >> s;
        s += 1;
    }
    x & 0x7FF
}

pub struct Involutions;

impl Involutions {
    /// Apply a single bit-flip involution
    /// bit_index 0..7: flips bits of byte b
    /// bit_index 8..10: flips low 3 bits of p2
    pub fn apply(p: usize, b: usize, bit_index: usize) -> (usize, usize) {
        assert!(bit_index < 11, "bit_index must be in [0,11)");
        
        let mut p_new = p % P_MOD;
        let mut b_new = b % B_MOD;

        if bit_index < 8 {
            b_new ^= 1 << bit_index;
        } else {
            let (mut p2, p3) = split_p(p_new);
            let k = bit_index - 8;
            p2 ^= 1 << k;
            p_new = join_p(p2, p3);
        }

        (p_new, b_new)
    }

    /// Enumerate the 2^11 orbit starting from anchor (p, b)
    pub fn orbit_from_anchor(p: usize, b: usize) -> Vec<(usize, usize)> {
        let mut orbit = Vec::with_capacity(2048);
        for t in 0..(1 << 11) {
            let g = gray11(t);
            // Gray code: 0..7 map to b bits, 8..10 map to p2 bits
            let b_flip = g & 0xFF;
            let p2_flip = (g >> 8) & 0x7;

            let b_new = b ^ b_flip;
            let (mut p2, p3) = split_p(p);
            p2 ^= p2_flip;
            let p_new = join_p(p2, p3);

            orbit.push((p_new, b_new));
        }
        orbit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_gray_code_roundtrip() {
        for i in 0..(1 << 11) {
            assert_eq!(ungray11(gray11(i)), i);
        }
    }

    #[test]
    fn test_orbit_uniqueness() {
        let orbit = Involutions::orbit_from_anchor(17, 42);
        let unique: HashSet<_> = orbit.iter().collect();
        assert_eq!(unique.len(), 2048);
    }
}
