use goldilocks::{GoldilocksField, PrimeMask, ResonanceWord, GOLDILOCKS_PRIME};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoldilocksElement(pub u64);

impl GoldilocksElement {
    /// Safe initialization enforcing the canonical field modulus limit
    pub fn new(val: u64) -> Self {
        if val >= GOLDILOCKS_PRIME {
            GoldilocksElement(val % GOLDILOCKS_PRIME)
        } else {
            GoldilocksElement(val)
        }
    }

    pub fn to_field(&self) -> GoldilocksField {
        GoldilocksField::new(self.0)
    }
}

/// Combines your 96 structural equivalence classes with field-pure prime indices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldConfinementState {
    pub resonance_class: u8,             // Constrained index from 0 to 95
    pub payload_field: GoldilocksElement, // Packed 57-bit metadata trace element
    pub prime_gate_mask: u64,            // P_64 bitmask for active prime-channel selection
}

impl FieldConfinementState {
    pub fn to_resonance_word(&self) -> Result<ResonanceWord, &'static str> {
        if self.resonance_class >= 96 {
            return Err("RESONANCE_VIOLATION: Class index out of bounds for the 96x12,288 lattice.");
        }
        if self.payload_field.0 >= (1 << 57) {
            return Err("PAYLOAD_OVERFLOW: Payload must fit in 57 bits.");
        }
        Ok(ResonanceWord::pack(self.resonance_class, self.payload_field.0))
    }

    pub fn to_prime_mask(&self) -> PrimeMask {
        PrimeMask(self.prime_gate_mask)
    }
}

/// Implements a rigid algebraic contraction checker to protect the L0 space boundary
pub struct GoldilocksValidator;

impl GoldilocksValidator {
    /// Asserts that a state change strictly conforms to field invariants under Plane B
    pub fn verify_state_confinement(state: &FieldConfinementState) -> Result<(), &'static str> {
        // Enforce Plane C: Spatial addressing must map under the 96 boundary matrix
        if state.resonance_class >= 96 {
            return Err("RESONANCE_VIOLATION: Class index out of bounds for the 96x12,288 lattice.");
        }

        // Enforce Plane A: Canonical integrity check against field-overflow exploits
        if state.payload_field.0 >= GOLDILOCKS_PRIME {
            return Err("ALGEBRAIC_DRIFT: Payload field element slips outside canonical field limits.");
        }

        Ok(())
    }
}
