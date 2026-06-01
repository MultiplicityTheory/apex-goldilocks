# ADR-038: Wiring Math-Lab Dashboard with Apex-Goldilocks Core via WASM

## Status
Proposed

## Context
The UOR Foundation's `math-lab` repository contains an interactive React/Vite dashboard for visualizing mathematical structures. However, it currently relies on javascript-based mock logic or external network APIs (`uor-foundation-math-js`), creating a risk of float drift and diverging behavior from the Rust-based L0 core invariants (the Sedona Spine).

To achieve "Zero Drift" and "No Floats" in the visualization layer, we must establish a direct, high-integrity execution path from the Rust `apex-goldilocks` engine to the browser-based dashboard.

## Decision
We will build a WebAssembly (WASM) compilation stratum for the `apex-goldilocks` workspace and configure `math-lab` to consume these bindings. 

### 1. WASM Bridge Crate
We will create a new crate `crates/apex-goldilocks-wasm` in the `apex-goldilocks` workspace:
- Uses `wasm-bindgen` to expose core verification algorithms.
- Exposes `audit_lattice_wasm() -> JsValue`: executes `LatticeCertificate::verify()` and returns a JSON-serialized structural map of the 12,288 elements, the 6 disjoint orbits, and $U_{\text{ref}}$ action.
- Exposes `run_pilot_wasm(domain: u64, budget: u64) -> JsValue`: executes an EchoBraid pilot, runs the veto-enforced `HarnessAdapter`, and returns the state history and final `AtlasEmbeddingProof` (AEP) validation status.

### 2. Vite Integration in Math-Lab
In the `math-lab` dashboard:
- Install the compiled WASM package via a local file reference or package link:
  ```json
  "dependencies": {
    "apex-goldilocks-wasm": "file:../crates/apex-goldilocks-wasm/pkg"
  }
  ```
- Use `vite-plugin-wasm` and `vite-plugin-top-level-await` in `math-lab/vite.config.ts` to allow synchronous-like imports of WASM modules.
- Refactor the dashboard's Redux slices (specifically the lattice and pilot states) to delegate all computations to the imported WASM functions.

---

## Acceptance Criterion

The wiring is considered successful when:
1.  `npm run build` in `projects/apex-goldilocks/math-lab` compiles without errors and bundles the WASM binary.
2.  The lattice page in the dashboard displays the exact canonical tiling of 12,288 elements, 6 disjoint orbits of size 2,048, and free action.
3.  The EchoBraid pilot interface successfully triggers the `HarnessAdapter` in Rust-WASM and shows appropriate veto dialogs on budget depletion or active prime gate violations.
4.  No float operations (`f32`/`f64`) are used to compute coordinates or orbits in the React layer.

---

## Owners & Metrics

-   **Owner**: Lead Integrator
-   **Metric**: $0\%$ float drift between visualization states and core certifiers.
-   **Horizon**: 15 days to initial pilot integration; 30 days to full deprecation of legacy js-math package in `math-lab`.

---

## Dependencies

1.  **Crates**:
    *   `apex-goldilocks-core` (tiling/R96 definitions)
    *   `multiplicity-runtime` (veto validation)
2.  **Tooling**:
    *   `wasm-pack` (rust-to-wasm compilation tool)
    *   `vite-plugin-wasm` (Vite integration plugin)
