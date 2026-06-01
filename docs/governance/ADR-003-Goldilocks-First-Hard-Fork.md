# ADR-003: Goldilocks-First Hard Fork

## Status
Accepted

## Context
The existing `projects/apex` stack suffers from "float leakage" and drift from L0 invariants. Rather than attempting a complex refactor of the entire existing codebase, we are initiating a hard fork into `projects/apex-goldilocks`.

## Decision
Create a new workspace `projects/apex-goldilocks` where the goldilocks arithmetic kernel is the mandatory foundation for all crates.

## Design Principles
1. **Field-Centric**: All calculations must be performed within `GoldilocksField` (p = 2^64 - 2^32 + 1).
2. **No Floats**: `f32` and `f64` are strictly prohibited in the core logic.
3. **Primitive Alignment**: Use `PrimeMask` and `ResonanceWord` from the goldilocks crate to ensure prime-indexed contractivity and 96-class resonance parity.
