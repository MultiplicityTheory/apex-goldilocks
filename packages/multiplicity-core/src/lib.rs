//! Multiplicity Core - Rust Implementation
//! 
//! This crate implements the core architectural primitives:
//! - Lane layout
//! - Toggle streams
//! - Quantizer
//! - ACE safety
//! - PETC ledger
//! - Audit ledger
//! - Proofs
//! - Watchdog
//! - Boundary lattice
//! - Lattice guard
//! - Phase0 index
//! - Sigmatics bridge
//! - Scheduler

pub mod lanes;
pub mod toggles;
pub mod quantizer;
pub mod ace;
pub mod petc;
pub mod ledger;
pub mod proofs;
pub mod watchdog;
pub mod boundary_lattice;
pub mod lattice_guard;
pub mod phase0_index;
pub mod sigmatics_bridge;
pub mod scheduler;
pub mod runtime;

// Public API surface
pub use lanes::Lane;
pub use toggles::ToggleStream;
pub use quantizer::Quantizer;
pub use ace::AceProjection;
pub use petc::Ledger as PetcLedger;
pub use ledger::Ledger;
