//! Quantizer primitives

use goldilocks::GoldilocksField;

/// Ring quantizer for real to field mapping
pub struct Quantizer {
    pub scale: f64,
    pub bias: f64,
}

impl Quantizer {
    pub fn new(scale: f64, bias: f64) -> Self {
        Self { scale, bias }
    }

    pub fn quantize(&self, value: f64) -> GoldilocksField {
        let quantized = (value * self.scale + self.bias) as u64;
        // Assuming conversion from u64 to GoldilocksField
        GoldilocksField::from_canonical(quantized)
    }
}
