//! Convolution operations (Conv)
//!
//! Standard ONNX convolution with full generic type support.

use super::{OnnxHRMNode, Result};
use crate::error::CompilerError;
use crate::hrm::numeric::Numeric;
use hologram_hrm::Atlas;

/// Conv operation (Convolution)
///
/// Performs 2D convolution on input tensors.
/// 
/// Note: This is a simplified implementation. Full ONNX Conv supports
/// multi-channel tensors, padding, dilation, and stride.
#[derive(Debug, Clone)]
pub struct ConvOp {
    /// Input height
    pub h_in: usize,
    /// Input width
    pub w_in: usize,
    /// Kernel height
    pub k_h: usize,
    /// Kernel width
    pub k_w: usize,
}

impl ConvOp {
    /// Create a new Conv operation
    pub fn new(h_in: usize, w_in: usize, k_h: usize, k_w: usize) -> Self {
        Self { h_in, w_in, k_h, k_w }
    }

    /// Construct from ONNX node shapes
    pub fn from_shapes(input_shapes: &[Vec<i64>]) -> Result<Self> {
        crate::validate_input_count!(input_shapes, 2, "Conv");

        let (h_in, w_in) = crate::extract_2d_dims!(input_shapes[0], "Conv input X");
        let (k_h, k_w) = crate::extract_2d_dims!(input_shapes[1], "Conv weight W");

        Ok(Self::new(h_in, w_in, k_h, k_w))
    }
}

impl<T: Numeric> OnnxHRMNode<T> for ConvOp {
    fn op_type(&self) -> &'static str {
        "Conv"
    }

    fn execute(&self, _atlas: &Atlas, inputs: &[&[T]]) -> Result<Vec<T>> {
        self.validate_inputs(inputs)?;

        let x = inputs[0];
        let w = inputs[1];

        let h_out = self.h_in - self.k_h + 1;
        let w_out = self.w_in - self.k_w + 1;
        let mut y = vec![T::zero(); h_out * w_out];

        for i in 0..h_out {
            for j in 0..w_out {
                let mut sum = T::zero();
                for ki in 0..self.k_h {
                    for kj in 0..self.k_w {
                        let in_i = i + ki;
                        let in_j = j + kj;
                        let x_val = x[in_i * self.w_in + in_j];
                        let w_val = w[ki * self.k_w + kj];
                        sum = sum.add(x_val.mul(w_val));
                    }
                }
                y[i * w_out + j] = sum;
            }
        }

        Ok(y)
    }

    fn validate_inputs(&self, inputs: &[&[T]]) -> Result<()> {
        crate::validate_input_count!(inputs, 2, "Conv");
        crate::validate_input_size!(inputs[0], self.h_in * self.w_in, "Conv input X");
        crate::validate_input_size!(inputs[1], self.k_h * self.k_w, "Conv weight W");
        
        if self.h_in < self.k_h || self.w_in < self.k_w {
             return Err(CompilerError::InvalidModel(format!(
                "Conv input size [{}, {}] smaller than kernel size [{}, {}]",
                self.h_in, self.w_in, self.k_h, self.k_w
            )));
        }
        
        Ok(())
    }
}
