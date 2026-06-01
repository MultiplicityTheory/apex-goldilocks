use std::collections::HashMap;
use num_rational::Rational64;
use num_traits::Pow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sig {
    // Maps Prime Identity (u64) to Multiplicity Exponent (i32)
    pub multiplicity_map: HashMap<u64, i32>,
}

impl Sig {
    pub fn new() -> Self {
        Sig {
            multiplicity_map: HashMap::new(),
        }
    }

    // Multiplicity Functor M: Sig -> Q (Exact Rational)
    // Computes Product(p_i^m_i)
    pub fn multiplicity_functor(&self) -> Rational64 {
        self.multiplicity_map.iter()
            .map(|(&p, &m)| {
                let base = Rational64::from_integer(p as i64);
                // Rational64 supports pow(i32)
                base.pow(m)
            })
            .product()
    }

    // Compose signatures via Tensor Product (Multiplicity Addition)
    pub fn tensor_product(&self, other: &Sig) -> Sig {
        let mut new_map = self.multiplicity_map.clone();
        for (prime, exponent) in &other.multiplicity_map {
            *new_map.entry(*prime).or_insert(0) += exponent;
        }
        Sig { multiplicity_map: new_map }
    }
}
