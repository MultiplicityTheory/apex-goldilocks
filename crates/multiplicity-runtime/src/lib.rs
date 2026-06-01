pub mod harness;

use apex_goldilocks_core::boundary_lattice::{LatticeElement, R96};
use apex_goldilocks_core::{GoldVector, Resonance};
use goldilocks::{GoldilocksField, PrimeMask};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Compatibility check failed")]
    Incompatible,
    #[error("ACE Budget exceeded")]
    BudgetExceeded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CRMFConfig {
    pub domain_tag: u64,
    pub prime_index: usize,
    pub prime_mask: PrimeMask,
    pub signature: Option<pirtm_compiler::types::Sig>,
}

impl CRMFConfig {
    pub fn is_compatible(&self, other: &Self) -> bool {
        self.domain_tag == other.domain_tag && self.prime_index == other.prime_index
    }
}

pub struct MultiplicityRuntime {
    pub config: CRMFConfig,
    pub ace_budget: u64,
    pub tau: u64,
}

impl MultiplicityRuntime {
    pub fn new(config: CRMFConfig, ace_budget: u64) -> Self {
        Self {
            config,
            ace_budget,
            tau: 1000,
        }
    }

    pub fn project_ace(&mut self, state: &mut GoldVector) -> Result<(), RuntimeError> {
        if self.ace_budget == 0 {
            return Err(RuntimeError::BudgetExceeded);
        }

        // Enforce prime gate activation via PrimeMask
        let prime_bit = self.config.prime_index % 64;
        if !self.config.prime_mask.is_prime_set(prime_bit) {
            return Err(RuntimeError::Incompatible);
        }
        
        // Exact integer-based weighted l1-ball projection (float-free)
        let mut coords: Vec<u64> = state.elements.iter().map(|e| e.to_canonical()).collect();
        let sum: u64 = coords.iter().sum();
        
        if sum > self.tau {
            let mut lam_lo = 0;
            let mut lam_hi = *coords.iter().max().unwrap_or(&0);
            
            for _ in 0..50 {
                if lam_lo >= lam_hi { break; }
                let lam = (lam_lo + lam_hi) / 2;
                let projected_sum: u64 = coords.iter().map(|&c| if c > lam { c - lam } else { 0 }).sum();
                if projected_sum <= self.tau {
                    lam_hi = lam;
                } else {
                    lam_lo = lam + 1;
                }
            }
            
            let final_lam = lam_hi;
            for c in coords.iter_mut() {
                *c = if *c > final_lam { *c - final_lam } else { 0 };
            }
        }
        
        for (i, val) in state.elements.iter_mut().enumerate() {
            *val = GoldilocksField::new(coords[i]);
        }
        
        self.ace_budget -= 1;
        Ok(())
    }

    pub fn certify(&self, resonance: Resonance) -> bool {
        // Simplified CSC certification
        let (class, _payload) = resonance.unpack();
        class < 96
    }

    pub fn certify_lattice(&self, element: LatticeElement) -> bool {
        let r96 = R96::from_lattice(element);
        // Ensure the resonance class is within the canonical R96 range
        r96.0 < 96
    }
}
