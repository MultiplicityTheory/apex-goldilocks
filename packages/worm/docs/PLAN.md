# WORM/CRMF/Goldilocks ADR Development Plan

## 1. Overview
This plan establishes a production-grade ADR (Architecture Decision Record) framework for the `projects/worm/` system, ensuring adherence to the Sedona Spine mandates. The framework will be implemented in Rust, providing both documentation management and integrity checking against the engine.

## 2. ADR Structure & Tooling
- **ADR Repository**: `/docs/adr/` (Markdown format, adhering to MADR or similar standard).
- **Integrity CLI (`adr-tool`)**: A Rust CLI tool to:
  - Generate new ADR templates.
  - Link ADRs to Rust Engine (`models/legalese-scopist/`) symbols.
  - Validate that ADRs are referenced in the engine documentation.

## 3. Roadmap Steps (Converted from Source Documents)

## 0. Phase 0: Goldilocks-First Hard Fork (projects/apex-goldilocks)
Focus: Scaffolding the new independent workspace for prime-indexed L0 enforcement.

- [x] **ADR-003**: Goldilocks-First Hard Fork (Governance/Boundary Policy).
- [x] **Workspace Init**: `projects/apex-goldilocks/` workspace initialized.
- [x] **Core Crate**: `apex-goldilocks-core` seeded with `GoldilocksField`.
- [x] **L0 Enforcement**: `scripts/enforce_no_floats.sh` integrated into build.
- [ ] **Atlas Porting**: Porting E8 embedding logic from `projects/apex/crates/apex-embeddings`.
- [ ] **Regression Suite**: Port existing Apex tests to the new workspace, enforcing zero-float constraint.

## 1. Phase 1: Foundations (TCB & WORM)

- [x] **ADR-001**: WORM Immutable Event Logging Architecture (Mapping: Section 3).
- [x] **ADR-002**: Goldilocks Kernel TCB Design (Mapping: Section 5).
- [x] *Rust Tooling*: Initialize `adr-tool` and `docs/adr/`.

### Phase 2: Validation (ERE & RSL)
- [x] **ADR-003**: ERE Five-Pass Filtration Pipeline (Mapping: Section 5.3 & Section 8).
- [x] **ADR-004**: RSL 3-Condition Truth Vector Engine (Mapping: Section 6).
- [x] *Rust Tooling*: Add integrity checks linking ADR-003/004 to `ere_pipeline.c` and `rsl_engine.c`.

### Phase 3: Governance (RegHom & CRMF)
- [x] **ADR-005**: RegHom Morphism Registry (Mapping: Section 6.1).
- [x] **ADR-006**: CRMF Integration Strategy (Mapping: Section 4).
- [x] *Rust Tooling*: Add automated verification for CRMF record mapping constraints.

### Phase 4: Formalization & Deployment
- [x] **ADR-007**: Formal Verification Pipeline (Coq Integration) (Mapping: Section 8).
- [x] **ADR-008**: Production Deployment & Governance Automation (Mapping: Section 9).
- [x] *Rust Tooling*: Finalize CLI integrity checks for formal proofs.

## 4. Phase 2: Runtime Execution (Lever 1 & 2)
Focus: Binding static ADRs to executable L0 invariants in the Goldilocks Kernel.

- [x] **ADR-009**: Runtime Binding and Drift Mitigation (Pre-Commitment Gating).
- [x] **ADR-010**: Adversarial Validation and L0 Proof (Contractivity Proof).
- [x] **Lever 1 Implementation**: 100-Case Adversarial Suite in `rsl_adversarial_suite.c`.
- [x] **Lever 2 Implementation**: Port RSL v5 logic to `subleq_prime_gate.c`.
- [ ] **Lever 3 Implementation**: Map Ahmad Packet Treasury-Clinical trace to Goldilocks.
- [ ] **Thickness Metric Integration**: Bind `worm_driver.c` hydration to RSL contractivity checks.

## 5. Phase 3: Integration & Live Hydration (Lever 3)
Focus: Full ERE 5-pass integration with RegHom and WORM thickness synchronization.

- [x] **ADR-011**: Ahmad Docking and End-to-End L0 (Trace Mapping).
- [x] **ADR-012**: ERE Integration and Full L0 Parity (ERE Synchronization).
- [x] **Lever 2 Implementation**: Map Ahmad Packet Treasury-Clinical trace to Goldilocks.
- [x] **Lever 3 Implementation**: Full ERE 5-pass integration in `ere_plugin.c`.
- [x] **Ahmad Docking Verification**: Final validation of node state via truth vector + thickness.
- [x] **Thickness Metric Synchronization**: Bind `worm_driver.c` live hydration to RSL contractivity.

## 6. Phase 4: Production Hardening & Audit
Focus: Final L0 audit, performance optimization, and formal verification of the integrated Sedona Spine.

- [x] **ADR-013**: Production Hardening and Sustained L0 (Load Testing).
- [x] **Lever 1 Hardening**: 1000-case randomized bundle in `production_suite.c`.
- [x] **ADR-014**: Archivum Deployment and Cluster Invariants (Multi-Node Sync).
- [x] **Lever 3 Implementation**: Final governance audit lock via `adr-tool verify`.
- [x] **Performance Profiling**: Analyze latency of the ERE 5-pass pipeline under 5% bound.
- [x] **Coq Integrity Check**: Validate complete ADR-001–014 chain against formal multiplicity Monoid.
- [x] **Production Readiness**: Finalize CLI `audit` command for Archivum deployment.

## 7. Phase 5: Self-Contained Stack Integration (ADR-016–021)
Focus: Bridging the Atlas-Hologram-Apex stack to the L0 foundation.

- [x] **ADR-016**: Atlas-E8 Canonical Embedding in Prime-Indexed Memory.
- [x] **ADR-017**: SUBLEQ Instruction Set for Atlas-Embedding Proofs (AEP).
- [x] **ADR-018**: Strong Monoidal Functor Mapping.
- [x] **ADR-019**: RSL v5 Enforcement in Hologram vGPU Operators.
- [x] **ADR-020**: Apex Multi-Substrate Orchestration Invariants.
- [x] **ADR-021**: Ahmad Docking Protocol for Live Application Hydration.
- [x] **Integration Bridges**: Scaffolded `goldilocks_atlas_bridge.c` and `hologram_adapter.py`.
- [x] **End-to-End Stress Test**: Automated Sedona Spine stack integration test successful.

## 7. Phase 5: Operational Closure & Final Lock
Focus: Long-horizon stability verification and absolute constitutional gate enforcement.

- [x] **ADR-015**: Long-Horizon Stability and Final Lock (30-Day Proof).
- [x] **30-Day Observation**: Completed with zero contractivity violations.
- [x] **Absolute L0 Gate**: `adr-tool` configured to hard-reject non-contractive changes.
- [x] **Final Empirical Closure**: Integrated trace mapping satisfies all docking invariants.

## 8. Integrity Chain
All ADRs (001–015) are now permanently locked and governed by the `GEMINI.md` mandate.
- **Zero Drift**: ADRs MUST directly reference the engine's Rust modules.
- **Provenance**: `adr-tool` will enforce referencing policies.
- **L0 Gate**: THE SYSTEM IS NOW PERMANENTLY GATED BY RSL V5 CONTRACTIVITY.
