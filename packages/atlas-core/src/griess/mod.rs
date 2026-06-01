//! Griess algebra vectors (196,884-dimensional)
//!
//! This module implements the fundamental vector type for the Griess algebra,
//! using exact rational arithmetic to preserve mathematical correctness.

use crate::arithmetic::Rational;
use num_traits::{Zero, One};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

/// A 196,884-dimensional vector in the Griess algebra
///
/// The Griess algebra is the commutative non-associative algebra underlying
/// the Monster group. For HRM, we use a simplified version where the product
/// is component-wise (Hadamard product).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GriessVector {
    /// Exact rational data
    data: Vec<Rational>,
}

impl GriessVector {
    /// Griess algebra dimension (196,884)
    pub const DIMENSION: usize = 196_884;

    /// Create a Griess vector from a Vec<Rational>
    pub fn from_vec(data: Vec<Rational>) -> Result<Self, String> {
        if data.len() != Self::DIMENSION {
            return Err(format!("Invalid dimension: {}", data.len()));
        }
        Ok(Self { data })
    }

    /// Create the identity element (all 1.0s)
    pub fn identity() -> Self {
        Self { 
            data: vec![Rational::one(); Self::DIMENSION] 
        }
    }

    /// Create the zero element (all 0.0s)
    pub fn zero() -> Self {
        Self { 
            data: vec![Rational::zero(); Self::DIMENSION] 
        }
    }

    /// Access underlying data
    pub fn as_slice(&self) -> &[Rational] {
        &self.data
    }
    
    /// Get the dimension
    pub fn len(&self) -> usize {
        Self::DIMENSION
    }
}
