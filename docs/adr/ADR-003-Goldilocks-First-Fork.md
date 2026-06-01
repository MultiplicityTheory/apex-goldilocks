# ADR-003: Goldilocks-First Hard Fork (Scoped to Multiplicity)

## Status
Accepted

## Context
The existing `projects/apex` stack suffers from "float leakage" and drift from L0 invariants. To preserve mathematical fidelity while maintaining engineering velocity, we are initiating a scoped hard fork into `projects/apex-goldilocks`, focusing initially on the Multiplicity backend.

## Decision
Create a new workspace `projects/apex-goldilocks` where the `goldilocks` arithmetic kernel is the mandatory foundation. 

Initially, only the following components will be ported/implemented:
1.  **apex-goldilocks-core**: Basic field arithmetic abstractions.
2.  **multiplicity-runtime**: The core ACE/PETC runtime and CRMF/CSC certifiers.

## Design Principles
1.  **Field-Centric**: All calculations must be performed within `GoldilocksField`.
2.  **No Floats**: `f32` and `f64` are strictly prohibited in `projects/apex-goldilocks`.
3.  **Transitional Oracle**: The legacy `projects/apex` stack will be maintained for 30 days as a validation oracle to ensure parity and correct for drift during the migration.

## 12,288 Boundary Lattice Derivation
The 12,288-element boundary lattice (G = P × B, |G|=12,288) is established as the hard classical computational horizon. 

### Mandate
- **First-Principles Derivation**: The lattice must be derived bottom-up in `GoldilocksField` (P = Z/48Z, B = Z/256Z).
- **Subgroup Certificate**: Must prove the `U_ref` subgroup (order 2048) acting on 6 anchors produces a valid tiling and free action.
- **Resonance Canonicalization**: R96 classes must be derived from the Gray-orbit reduction on this lattice.

## Implementation Path
- **Phase 0**: Workspace scaffolding and core field primitives.
- **Phase 1**: Multiplicity certification flow (ACE → CSC).
- **Phase 2**: Expansion to `atlas-embeddings-v2` and `hologram-goldilocks` as capacity permits.

## Consequences
- **Integrity**: Guaranteed L0 invariant preservation for the core runtime.
- **Parallel Maintenance**: Requirement to maintain two workspaces during the transition.
- **Validation**: Higher confidence in the new implementation by comparing with legacy outputs.
