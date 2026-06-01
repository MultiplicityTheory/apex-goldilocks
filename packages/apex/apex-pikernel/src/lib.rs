//! # apex-pikernel
//!
//! Exact-arithmetic Rust implementation of π-kernel for ACE and PETC.
//! Complies with the Sedona Spine mandate of zero floating-point reliance.

pub mod projectors;
pub mod l1proj;
pub mod kernel;
pub mod ledger;
pub mod certificates;

// Re-exports
pub use projectors::*;
pub use l1proj::{project_weighted_l1_ball}; // Export specific items
pub use kernel::*;
pub use ledger::*;
pub use certificates::{slope_upper_bound, gap_lower_bound}; // Export specific items
pub use l1proj::Rational; // Export Rational from one place only
