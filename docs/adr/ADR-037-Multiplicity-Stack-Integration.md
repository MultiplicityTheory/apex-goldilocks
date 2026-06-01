# ADR-037: Multiplicity Stack Integration - Neural Harness Stratum

## Status
Accepted

## Context
Integrating adaptive learning (neuroplasticity/EchoBraid) with the sealed Phase Mirror stack. The challenge is allowing for interpretive flexibility without compromising the L0 invariants (CSC sealing, Goldilocks arithmetic, prime-gated witnesses).

## Decision
Establish a **Neural Harness** stratum. This is an interpretive-only layer positioned above the representation and certification rails.

## Design Rules (The Veto-Enforced Rail)
1.  **Interpretive Only**: The harness may propose adaptations but cannot unilaterally commit them. All commitments must route through the CSC/Pro certification gate.
2.  **Sealed Adaptation**: Every transition $\Theta(t) \to \Theta(t+1)$ must be validated by the Multiplicity Runtime for:
    - **CSC Compatibility**: DomainTag and P_N alignment.
    - **Multiplicity Preservation**: No expansion of surviving structures.
    - **Zero Float Invariant**: Absolute enforcement in the underlying implementation.
3.  **Tier 4 Recovery**: The harness utilizes Tier 4 spectral recovery (`spectral_healthy()`) as a guide for plasticity, but any proposed state must pass the `Compatible()` guard.
4.  **Veto Mechanism**: The Goldilocks runtime retains absolute veto power over any harness proposal that breaches CSL $\Delta S$ or seal invariants.

## Boundary Definition
- **Gold Zone (Certification Rail)**: `mirror-math/crmf`, `apex-goldilocks-core`.
- **Interpretive Zone (Harness)**: `EchoBraid`, `NeuroplasticHarness`.

## Implementation Path
- **Phase 1**: Implement the `HarnessAdapter` in `multiplicity-runtime` to bridge EchoBraid proposals to CSC Tier 4 witnesses.
- **Phase 2**: Wire the recursive $\Theta(t+1)=\Xi(\Theta(t))$ mapping to the AEP/carry-forward logic.

## Appendix: EchoBraid Adapter Implementation
The concrete implementation of the Neural Harness stratum focuses on the **EchoBraid** recursive primitive.

### EchoBraid State (Θ)
The state is represented by `EchoBraidState`, which encapsulates the `GoldVector` and the iteration count.

### Transition Function (Ξ)
The transition $\Theta(t+1) = \Xi(\Theta(t))$ is implemented as a prime-graded transformation within the `NeuralHarness::transition` method. This allows for maximal interpretive expressiveness within the bounded plasticity window.

### Veto Paths Verified
1.  **Multiplicity Veto**: Proposals exceeding the `prime_index` capacity are rejected.
2.  **Budget Veto**: Proposals requested when the `ace_budget` is exhausted are rejected.
3.  **Spectral Veto**: Proposals with invalid Tier 4 spectral witnesses are rejected.

### Decision on Precision
We prioritize **maximal interpretive expressiveness** during the braiding process. The robust, sealed veto mechanism in the `HarnessAdapter` ensures that even complex or contradictory proposals are safely contained, allowing the system to explore rich identity braiding without risking the integrity of the Goldilocks certification rail.
