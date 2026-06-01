use crate::arithmetic::Rational;
use num_traits::{Zero, Signed};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DynamicRationalMatrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Rational>,
}

impl DynamicRationalMatrix {
    pub fn new(rows: usize, cols: usize, data: Vec<Rational>) -> Self {
        assert_eq!(data.len(), rows * cols);
        Self { rows, cols, data }
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![Rational::zero(); rows * cols],
        }
    }

    pub fn get(&self, r: usize, c: usize) -> Rational {
        self.data[r * self.cols + c]
    }

    pub fn set(&mut self, r: usize, c: usize, val: Rational) {
        self.data[r * self.cols + c] = val;
    }

    pub fn matvec(&self, v: &[Rational]) -> Vec<Rational> {
        assert_eq!(self.cols, v.len());
        let mut out = vec![Rational::zero(); self.rows];
        for r in 0..self.rows {
            let mut sum = Rational::zero();
            for c in 0..self.cols {
                sum += self.get(r, c) * v[c];
            }
            out[r] = sum;
        }
        out
    }

    pub fn transpose_matvec(&self, v: &[Rational]) -> Vec<Rational> {
        assert_eq!(self.rows, v.len());
        let mut out = vec![Rational::zero(); self.cols];
        for c in 0..self.cols {
            let mut sum = Rational::zero();
            for r in 0..self.rows {
                sum += self.get(r, c) * v[r];
            }
            out[c] = sum;
        }
        out
    }

    pub fn row_sum_norm(&self) -> Rational {
        let mut max_sum = Rational::zero();
        for r in 0..self.rows {
            let mut sum = Rational::zero();
            for c in 0..self.cols {
                sum += self.get(r, c).abs();
            }
            if sum > max_sum {
                max_sum = sum;
            }
        }
        max_sum
    }
}
