//! Decompositions and bit helpers for Atlas coordinates

pub const P_MOD: usize = 48;
pub const B_MOD: usize = 256;

/// Decompose p ∈ Z/48 as p = p2 + 16 * p3 with p2 ∈ Z/16, p3 ∈ Z/3
pub fn split_p(p: usize) -> (usize, usize) {
    let p_mod = p % P_MOD;
    let p3 = p_mod / 16;
    let p2 = p_mod % 16;
    (p2, p3)
}

/// Recompose p from p2 and p3
pub fn join_p(p2: usize, p3: usize) -> usize {
    (p2 % 16) + 16 * (p3 % 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_join_p() {
        for p in 0..P_MOD {
            let (p2, p3) = split_p(p);
            assert_eq!(join_p(p2, p3), p);
        }
    }
}
