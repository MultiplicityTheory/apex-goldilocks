//! # hologram-kernels
//!
//! Native Rust implementations of legacy ONNX and stdlib kernels.

pub mod onnx;
pub mod stdlib {
    pub mod vector;
    pub mod matrix;
    pub mod activation;
}

// Re-exports
pub use onnx::*;
pub use stdlib::vector::*;
pub use stdlib::matrix::*;
pub use stdlib::activation::*;
