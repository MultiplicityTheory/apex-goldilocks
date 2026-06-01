# Poly-Ontological — Unified Math‑Clean Specification v1.2

> Institute for Mathematical Discovery (IMD)
>
> Unified engineering contracts and mathematical foundations.

---

## 0. Global Conventions

**Sets.** \(\mathbb N,\mathbb Z,\mathbb Q,\mathbb R,\mathbb C\).\
**Vectors/Operators.** Hilbert spaces \(H\); bounded operators \(\mathcal B(H)\); adjoint \(T^{\dagger}\); spectrum \(\sigma(T)\); spectral radius \(\rho(T)\).\
**Norms.** \(\|\cdot\|\) operator norm; \(\|\cdot\|_2\) Euclidean; \(\|\cdot\|_1\) L1.\
**Probability.** Random variables uppercase; distributions \(\mathsf P,\mathsf Q\).\
**Categories.** Categories \(\mathcal C,\mathcal D\); functors \(F\colon\mathcal C\to\mathcal D\); natural transformations \(\eta:F\Rightarrow G\); adjunction \(F\dashv G\).\
**Metadata.** Every symbol below has a single meaning; units are indicated where relevant.

---

## 1. System Overview

IMD consists of:

1. **Engines** — safety‑critical services with formal contracts.
2. **Core Modules** — compact, composable components with fixed I/O.
3. **Auxiliary Engines** — support services (telemetry, sampling, persistence) with explicit bounds.
4. **Mathematical Foundations** — poly‑categorical semantics and spectral certification used by Engines.

---

## 2. Engines — Contracts

### 2.1 Interface Schema

For an Engine \(E\):

- **Inputs** \(x\in\mathcal X\) with unit map \(u_\mathcal X\colon\mathcal X\to\mathbb R^k\).
- **Outputs** \(y\in\mathcal Y\) with unit map \(u_\mathcal Y\colon\mathcal Y\to\mathbb R^m\).
- **Configuration** \(\theta\in\Theta\) (immutable during a run) and **state** \(s\in S\).
- **Contract**: predicates
  - Safety \(\mathsf{Safe}(x,\theta,s)\).
  - Liveness \(\mathsf{Live}(s)\).
  - Performance \(\mathsf{Perf}(x,y;\theta)\geq\tau\).

### 2.2 Runtime Invariants

1. **Boundedness**: \(\|y\|\leq b_0+b_1\|x\|\).
2. **Stability (spectral)**: if \(A(\theta)\) is the closed‑loop operator, require \(\rho(A(\theta))<1\).
3. **Monotone risk**: empirical risk \(\hat R_n\) is non‑increasing under accepted updates.

### 2.3 Certification Pipeline

1. **Distributional check (DKW/KS)**. For empirical CDF \(F_n\) and truth \(F\):

   $$
   \Pr\big(\sup_x |F_n(x)-F(x)|>\varepsilon\big)\le 2e^{-2n\varepsilon^2}.
   $$

   At level \(\alpha\), choose \(\varepsilon=\sqrt{\tfrac{1}{2n}\ln\tfrac{2}{\alpha}}\).  The classical 5% threshold is \(\varepsilon\approx 1.36/\sqrt n\).

2. **Sequential test (SPRT)** between \(\mathsf P_0\) and \(\mathsf P_1\). With log‑likelihood increments \(\ell_t=\log\frac{d\mathsf P_1}{d\mathsf P_0}(X_t)\), stop at first \(T\) with

   $$
   L_T=\sum_{t=1}^T\ell_t \ge \!\log\!\frac{1-\beta}{\alpha} \quad(\text{accept }\mathsf P_1),\qquad
   L_T\le \log\!\frac{\beta}{1-\alpha} \quad(\text{accept }\mathsf P_0).
   $$

3. **Spectral certification** of the learned operator \(W\): require \(\|W\|\le r\) and margin \(\gamma\) such that \(\sigma_{\max}(W)<1-\gamma\).

4. **Fail‑safe**: on violation of any invariant, revert to baseline configuration \(\theta_\mathrm{safe}\).

### 2.4 Acceptance Tests

- **Contract satisfaction**: all three predicates true on a held‑out audit set.
- **Calibration**: Brier score \(\le b^*\).
- **Latency**: \(\mathbb E[T_{\text{resp}}]\le T^*\); tail bound \(\Pr(T_{\text{resp}}>T^{**})\le \delta\).
- **Logging**: append‑only, cryptographic hash chain.

---

## 3. Core Modules — Compact Specs

Each module \(M\) exposes \(\mathrm{init}(\theta),\ \mathrm{step}(x,s),\ \mathrm{reset}()\). Types are total and single‑assignment per call.

- **Sampler**: draws IID or martingale‑difference sequences with seed discipline.
- **Estimator**: returns \((\hat\mu,\hat\Sigma)\) with concentration certificates.
- **Controller**: computes control \(u=Kx\) or \(u=\arg\min_u J(u;x,\theta)\); returns closed‑loop spectral radius.
- **Certifier**: implements DKW and SPRT; emits pass/fail with proofs and witnesses.
- **Recorder**: immutable logs with digest \(h_t=H(h_{t-1}\Vert m_t)\).

All modules use the units and symbols in §9.

---

## 4. Auxiliary Engines — Contracts

- **Telemetry**: bounded overhead \(\le \eta\) fraction of wall time; lossy compression with error \(\le\epsilon\) in \(\ell_\infty\).
- **Persistence**: ACID snapshot per certified release; monotone version IDs.
- **Diagnostics**: property‑based tests; counterexamples reduced via delta‑debugging.

---

## 5. Poly‑Categorical Foundations (PolyCat)

Let \(\mathcal A\) be a category of **poly‑ontological objects** parameterized by a finite **gauge set** \(P\subseteq\{\text{primes}\}\). Write the fiber over \(P\) as \(\mathcal A_P\).

### 5.1 Extension–Restriction Adjunctions

For each \(p\in P\), there are functors

$$
E_p\colon \mathcal A_P\to\mathcal A_{P\cup\{p\}},\qquad R_p\colon \mathcal A_{P\cup\{p\}}\to\mathcal A_P,
$$

with a natural isomorphism

$$
\mathrm{Hom}_{\mathcal A_{P\cup\{p\}}}(E_p X, Y)\;\cong\;\mathrm{Hom}_{\mathcal A_P}(X, R_p Y)
$$

natural in \(X,Y\). We write \(E_p\dashv R_p\).

### 5.2 Fibration Structure

Let \(\pi:\mathcal A\to\mathsf{Idx}\) map objects to their gauge set and morphisms to inclusions. Cartesian lifts exist by the universal property of \(R_p\), giving \(\pi\) a Grothendieck fibration.

### 5.3 Hubs and Initiality

A **hub** is an object \(e\in\mathcal A\) with a natural isomorphism

$$
\Phi_X:\ \mathrm{Hom}_{\mathcal A}(e,X)\ \xrightarrow{\ \cong\ }\ eX
$$

where \(eX\) denotes evaluation of the functor represented by \(e\).

**Theorem 5.1 (Initiality of hubs).** If \(\Phi_X\) is natural and \(eX\) is a singleton for all \(X\), then \(e\) is initial.\
*Proof.* For each \(X\) there is exactly one arrow \(e\to X\) corresponding to the unique element of \(eX\); naturality gives uniqueness.

### 5.4 Max‑Envelope

Fix \(P\) and a boundary Hilbert space \(H_\partial\). Let \(\rho\colon \mathcal A_P\to \mathcal B(H_\partial)\) be a normal \(*\)-representation with ultraweakly closed image. Define the **max‑envelope**

$$
\overline{\mathcal A_P}^{\,\mathrm{vN}}:=\rho(\mathcal A_P)^{\prime\prime} \subseteq \mathcal B(H_\partial).
$$

**Theorem 5.2 (Universal normal extension).** For any normal \(*\)-homomorphism \(\phi\colon\rho(\mathcal A_P)\to\mathcal B(K)\) there exists a unique normal \(*\)-homomorphism \(\widetilde\phi\colon \overline{\mathcal A_P}^{\,\mathrm{vN}}\to\mathcal B(K)\) with \(\widetilde\phi\!\restriction_{\rho(\mathcal A_P)}=\phi\).\
*Sketch.* Follows from bicommutant closure and the universal property of ultraweak continuity.

### 5.5 Rank Bound from Gauge Primes

Let \(P_o=P\setminus\{2\}\) and suppose there is a commuting family of self‑adjoint idempotents \(\{\Pi_p\}_{p\in P_o}\) acting on a finite‑dimensional subspace \(V\subseteq H_\partial\), with \(\Pi_p\Pi_q=\Pi_q\Pi_p\) and \(\Pi_p\neq I\). Let the **joint decomposition rank** be the number of nonzero joint eigenspaces.

**Proposition 5.3.** The joint decomposition rank satisfies \(\mathrm{rank}\le |P_o|\), with equality when the \(\Pi_p\) are linearly independent and mutually sharp on \(V\).

### 5.6 Size Parameters

For a gauge set \(P\) define

$$
N=2^{12}\prod_{p\in P\setminus\{2\}} p,\qquad L=256\prod_{p\in P\setminus\{2\}} p.
$$

**Examples.** \(P=\{2,3\}\Rightarrow N=12288,\ L=768.\quad P=\{2,3,5\}\Rightarrow N=61440,\ L=3840.\)

---

## 6. SMM Terminal Object

Let \(\mathsf{SMM}\) be the category of structured morphic models used by Engines.

**Conjecture 6.1 (Terminality).** There exists a terminal object \(\top\in\mathsf{SMM}\) such that for all \(X\) there is a unique \(X\to\top\) preserving structure. The terminal candidate is the ultraweak closure of the identity representation on \(H_\partial\).

---

## 7. Poly‑Ontological Formalization

A **poly‑ontological mathematical object** is a tuple \((U,\mathcal S, P,\mathcal R)\) where \(U\) is a universe of discourse, \(\mathcal S\) is a signature, \(P\) a finite gauge set of primes, and \(\mathcal R\) relations compatible with \(P\). Morphisms preserve \(\mathcal S\) and \(P\).

- **Extension/Restriction** coincide with \(E_p,R_p\) in §5.1.
- **Foundational claims** are expressed as theorems when supported by proofs; otherwise as conjectures.

**Conjecture 7.1 (Foundational initiality).** There exists a smallest nontrivial poly‑ontological object representing the empty theory over gauge \(P\).

---

## 8. Spectral Certification Workflows

1. **Operator construction**: build candidate \(W\) from data; enforce \(\|W\|\le r\) by projection.
2. **Gap analysis**: compute eigenvalues; require a gap \(\min_{\lambda\in\sigma(W)} (1-|\lambda|)\ge \gamma\).
3. **Empirical validation**: DKW for distributional fit; SPRT for sequential drift; optional martingale tests.
4. **Release gate**: accept iff all invariants in §2.2 and tests in §2.4 pass.

---

## 9. Symbol and Unit Glossary

- \(x\in\mathcal X\) input; units depend on sensor domain.
- \(y\in\mathcal Y\) output; same dimensionality as task codomain.
- \(\theta\in\Theta\) configuration; dimensionless unless stated.
- \(s\in S\) state; includes persistent buffers.
- \(A\) closed‑loop operator; unitless linear map on normalized state.
- \(W\) learned operator; same space as \(A\).
- \(H_\partial\) boundary Hilbert space; inner product units normalized to 1.
- \(P\) gauge set of primes; finite.
- \(E_p,R_p\) extension/restriction functors.
- \(\Pi_p\) spectral idempotents associated with \(p\).
- \(N,L\) size parameters defined in §5.6.
- \(\alpha,\beta\) Type‑I/II error levels.
- \(\gamma\) spectral margin.
- \(r\) operator‑norm radius.

---

## 10. Implementation Notes

- All modules must be pure functions of their inputs plus explicit state.
- All probabilities and errors are reported with numeric level and sample size.
- Logging is immutable and time‑stamped; every certified artifact has a content hash.
- Engines expose a single "certify()" entry point that runs §2.3–§2.4.

---

## 11. Change Log Key (for internal authorship)

- This unified draft harmonizes notation across Engines/Core/Aux and formal sections (§5–§8).
- Open proofs: Conjecture 6.1, Conjecture 7.1.
- Provide concrete module I/O tables per deployment as appendices.

