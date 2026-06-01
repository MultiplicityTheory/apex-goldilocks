# Apex: Lean 4 Proof Bridging

This document defines the formal mapping between the **Lean 4** mathematical formalization and the **Rust** implementation (The Sedona Spine). This mapping ensures that the high-integrity execution path remains logically consistent with the formally verified proofs.

## 1. Foundational Types

| Lean 4 Type | Rust Type (`apex-core`) | Rationale |
|-------------|-------------------------|-----------|
| `Rat`       | `arithmetic::Rational`  | Exact rational arithmetic with arbitrary precision numerators and denominators (mapped to `i64` in implementation for performance, with overflow checks). |
| `ResVertex` | `atlas::Label`          | The 6-tuple `(e1, e2, e3, d45, e6, e7)` representing resonance classes. |
| `ResGraph`  | `atlas::Atlas`          | The 96-vertex adjacency structure derived from Hamming-1 coordinate flips. |

## 2. Symmetry Identities

### Mirror Symmetry (τ)
- **Lean**: `def tau (v : ResVertex) : ResVertex := ...`
- **Rust**: `Label::mirror(&self)` and `Atlas::mirror_pair(v)`
- **Identity**: `τ ∘ τ = id`. This is verified by `test_mirror_symmetry` in Rust and formally proved in Lean.

### E₈ Root System Embedding
- **Lean**: `def embed_e8 (v : ResVertex) : E8Root := ...`
- **Rust**: `arithmetic::RationalVector<8>` (Derived from `Label` extension logic).
- **Hardening**: Rust implementation must enforce that every `Label` maps to a unique vector with norm squared = 2 (in appropriate units).

## 3. AEP Invariants

The Atlas-Embedding Proofs (AEP) are Rust executions of Lean-verified predicates.

| Lean Predicate | Rust Implementation (`apex-aep`) |
|----------------|----------------------------------|
| `is_unity_neutral` | `aep::decision_rules::UnityNeutral` |
| `is_mirror_safe`   | `aep::decision_rules::MirrorSafe` |
| `is_contractive`   | `aep::ace_runtime::verify_contraction` |

## 4. Verification Path

The "Path of Integrity" (GEMINI.md) is now formalized:
1. **Lean 4**: Proves existence and uniqueness of the Atlas and the emergence of Lie groups.
2. **Rust Core**: Implements the Atlas construction using exact arithmetic.
3. **Rust AEP**: Generates and verifies transition proofs (ACE/SPASC).
4. **ZK Sketch**: Succinctly proves the consistency of multiple AEP steps to external auditors.

## 5. Drift Prevention

Any change to the core arithmetic or adjacency logic in Rust **must** be preceded by a corresponding update in the Lean 4 formalization. The Rust tests serve as a local check, but the Lean 4 proof remains the ultimate authority.
