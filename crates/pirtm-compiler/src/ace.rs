// Define the SCALE_BASE as per Sedona Spine mandates.
pub const SCALE_BASE: i64 = 1_000_000;

#[derive(Debug, Clone, Copy)]
pub struct FixedPoint(pub i64);

impl FixedPoint {
    pub fn from_rational(numerator: i64, denominator: i64) -> Self {
        FixedPoint((numerator * SCALE_BASE) / denominator)
    }

    pub fn to_i64(self) -> i64 {
        self.0
    }
}

// Refactored structure using FixedPoint for "No Floats" compliance.
pub struct DynamicOperator {
    pub signature: crate::types::Sig,
    pub norm: FixedPoint, // Norm represented in fixed-point
}

// Refactored stability check sourceing c from MLIR attributes.
pub fn verify_stability(
    ops: &[DynamicOperator], 
    governance_c: FixedPoint
) -> Result<(), String> {
    let total_norm: i64 = ops.iter().map(|op| op.norm.0).sum();
    
    // Check: Sum(F_i) + c < 1 (where 1 is represented by SCALE_BASE)
    if total_norm + governance_c.0 < SCALE_BASE {
        Ok(())
    } else {
        Err(format!(
            "Stability constraint violated: Sum(F_i)={:?} + c={:?} >= SCALE_BASE={:?}",
            total_norm, governance_c.0, SCALE_BASE
        ))
    }
}
