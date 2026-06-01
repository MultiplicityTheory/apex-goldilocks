use apex_core::arithmetic::Rational;

#[derive(Debug, Clone)]
pub struct Z96Quantizer {
    pub scale: Rational,
    pub offset: i64,
}

impl Default for Z96Quantizer {
    fn default() -> Self {
        Self {
            scale: Rational::from_integer(1),
            offset: 0,
        }
    }
}

impl Z96Quantizer {
    pub fn new(scale: Rational, offset: i64) -> Self {
        Self { scale, offset }
    }

    pub fn quantize(&self, x: Rational) -> i32 {
        let y = (self.scale * x).to_integer() + self.offset;
        let q = y.rem_euclid(96) as i32;
        q
    }

    pub fn quantize_all(&self, xs: &[Rational]) -> Vec<i32> {
        xs.iter().map(|&x| self.quantize(x)).collect()
    }

    pub fn decode(&self, z: i32) -> Rational {
        Rational::from_integer(z as i64 - self.offset) / self.scale
    }
}
