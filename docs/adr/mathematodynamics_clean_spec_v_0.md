# Mathematodynamics — Clean Spec v0.2

## Abstract
We define a mathematically consistent framework for “mathematodynamics.” The theory specifies its base spaces, metrics, operators, action functional, equations of motion, symplectic structure, observables, estimation procedures, and certification tests. All symbols are uniquely typed. All gradients, Laplacians, and conservation statements are tied to the declared geometry. Optional arithmetic couplings are regularized and live on well-defined Hilbert spaces.

---

## 1. Mathematical setting
**Spacetime.** Let \((X,g)\) be a smooth, oriented, time-orientable, globally hyperbolic Lorentzian manifold of dimension \(d+1\) with metric signature \((-,+\dots +)\). For purely Euclidean models, replace \(g\) by a complete Riemannian metric. Let \(\mathrm{dvol}_g\) be the metric volume form.

**Bundles and fibers.** Let \(E\to X\) be a complex Hermitian vector bundle of rank \(D\) with pointwise inner product \(h\). Sections are denoted \(\psi\in \Gamma(E)\). The Hilbert space is
\[\mathcal H := L^2(X,E;\mathrm{dvol}_g),\quad \langle \phi,\psi\rangle = \int_X h(\phi,\psi)\,\mathrm{dvol}_g.\]
Sobolev spaces \(H^k(X,E)\) are taken with respect to \(g\) and \(h\).

**Gauge structure.** The local gauge group is \(G:=U(1)\) unless otherwise stated. A gauge potential is a connection 1-form \(A\in\Omega^1(X;i\mathbb R)\) with curvature \(F=\mathrm dA\). The covariant derivative on \(E\) is \(\nabla^A:=\nabla+i q A\) for charge \(q\in\mathbb R\). The gauge-covariant d’Alembertian is
\[\Box^A:=\operatorname{tr}_g\nabla^A\nabla^A.\]

**Discrete symmetries.** Optional global discrete symmetries are represented by a finite group \(G_{\mathrm{disc}}\subset\operatorname{Aut}(E)\). Their action on fields is linear and unitary with respect to \(h\).

---

## 2. Fields and potentials
**Primary field.** \(\psi:X\to E\) is the fundamental field. Its mass operator \(M\in \Gamma(\operatorname{End}(E))\) is fiberwise self-adjoint and positive semidefinite.

**Local potential.** \(V_{\text{loc}}:E\to\mathbb R\) is \(C^2\), gauge-invariant, bounded below, and grows at least quadratically for coercivity. Example: \(V_{\text{loc}}(\psi)=\frac{\lambda}{2}\,h(\psi,\psi)^2\) with \(\lambda\ge 0\).

**Coherence operator.** Let \(C\) be a bounded, self-adjoint, positive semidefinite operator on \(\mathcal H\). A canonical choice is a spectral filter \(C=f(\sqrt{-(\Box^A-\mu^2)_+})\) defined by the functional calculus for some \(\mu\ge 0\) and bounded Borel \(f\ge 0\).

---

## 3. Action functional and gauge invariance
Define the action
\[\mathcal S[\psi,A]=\int_X \Big( h(\nabla^A\psi,\nabla^A\psi) - h(\psi,M\psi) - V_{\text{loc}}(\psi) \Big)\,\mathrm{dvol}_g\; -\; \frac{1}{4\mu_0}\int_X F_{\mu\nu}F^{\mu\nu}\,\mathrm{dvol}_g,\]
with constant \(\mu_0>0\). This is invariant under the local \(U(1)\) gauge transformation \(\psi\mapsto e^{iq\chi}\psi,\; A\mapsto A-\mathrm d\chi\) for smooth \(\chi\).

**Coherence energy.** Define \(E_{\text c}(\psi):=\tfrac12\langle\psi, C\psi\rangle\). Its variational derivative in the \(L^2\) metric is \(\delta E_{\text c}/\delta \psi^*=C\psi\). When included, it contributes additively to the Hamiltonian (Section 5) and to the field equation (Section 4).

---

## 4. Euler–Lagrange equations
Assuming variations of compact support and appropriate boundary conditions, critical points of \(\mathcal S\) satisfy
\[\Box^A\psi + M\psi + \partial_{\psi^*}V_{\text{loc}}(\psi) = 0,\]
\[\mathrm d\star F = J,\qquad J_\mu := q\,\operatorname{Im}\, h\big(\psi, (\nabla^A_\mu\psi)\big).\]
If the coherence energy is present, add \(+C\psi\) to the left hand side of the \(\psi\) equation. Standard results give local well-posedness in \(H^1\times L^2\) under the stated regularity and coercivity assumptions.

---

## 5. Hamiltonian and symplectic structure
Take a smooth Cauchy hypersurface \(\Sigma\subset X\) with induced Riemannian metric \(\gamma\) and volume form \(\mathrm{dvol}_\gamma\). Define canonical momentum \(\pi:=n^\mu h(\nabla^A_\mu\psi,\cdot\,)\) where \(n\) is the future unit normal to \(\Sigma\). The phase space is
\[\Gamma := H^1(\Sigma,E)\times L^2(\Sigma,E)\times \mathcal A/\mathcal G,\]
where \(\mathcal A\) is the space of admissible spatial connections and \(\mathcal G\) the gauge group on \(\Sigma\). The symplectic form on the matter sector is
\[\omega\big((\delta\psi,\delta\pi),(\delta'\!\psi,\delta'\!\pi)\big)=\int_\Sigma \Big(h(\delta\psi,\delta'\!\pi)-h(\delta'\!\psi,\delta\pi)\Big)\,\mathrm{dvol}_\gamma.
\]
A compatible Poisson bracket \(\{\cdot,\cdot\}\) is induced by \(\omega\). The matter Hamiltonian density is
\[\mathcal H_\text{m} = h(\pi,\pi) + h(\nabla^A\psi,\nabla^A\psi) + h(\psi,M\psi) + V_{\text{loc}}(\psi) + h(\psi, C\psi),\]
with gauge-field energy \(\tfrac{1}{2\mu_0}(|E|^2+|B|^2)\). Hamilton’s equations reproduce Section 4. Canonical quantization replaces the equal-time Poisson bracket by the CCR \([\widehat\Psi(x),\widehat\Pi(y)]=i\,\delta_\Sigma(x,y)\,\mathbf 1\).

---

## 6. Observables and conservation
**Noether currents.** For any continuous symmetry of \(\mathcal S\) there is a conserved current \(j^\mu\) with \(\nabla_\mu j^\mu=0\). Gauge invariance yields the charge \(Q=\int_\Sigma J^0\,\mathrm{dvol}_\gamma\).

**Bounded observables.** Observables are self-adjoint bounded operators on \(\mathcal H\) or integrals of bounded densities. Examples: energy \(E=\int_\Sigma \mathcal H\,\mathrm{dvol}_\gamma\); coherence \(E_{\text c}\); correlation functionals \(\mathcal C_K(\psi)=\langle\psi,K\psi\rangle\) for compact \(K\).

---

## 7. Estimation, experiments, and falsification
**Data model.** Let \(\mathcal D=\{(x_i,y_i)\}_{i=1}^N\) be observations generated by an instrument model \(y_i = \mathcal O(\psi)(x_i) + \varepsilon_i\) with i.i.d. noise \(\varepsilon_i\sim \mathcal N(0,\sigma^2)\) unless otherwise specified. \(\mathcal O\) is a bounded observable.

**Estimators.** For each observable \(O\), define \(\widehat O\) as the MLE or a regularized estimator with explicit penalty and hyperparameters. Confidence intervals follow from the Fisher information or bootstrap with declared seeds.

**Hypothesis tests.** Pre-register nulls \(H_0\) and alternatives \(H_1\), significance \(\alpha\), and effect sizes. Report p-values and power. A claim is accepted only if criteria are met on unseen test data.

**Reproducibility.** Each numerical result is accompanied by: action hash, mesh spec, integrator, time step, wall-clock budget, random seed, and KKT or residual tolerances.

---

## 8. Optional arithmetic sector (regularized)
**Signal space.** Work on \(L^2(\mathbb R,\mathrm dx)\). For \(\varepsilon>0\) let \(\varphi_\varepsilon\) be a Schwartz function with \(\int \varphi_\varepsilon=1\) and bandwidth \(\varepsilon\).

**Smoothed von Mangoldt field.** Define
\[a_\varepsilon(x):=\sum_{n\ge 2} \Lambda(n)\,\varphi_\varepsilon(x-\log n),\]
which converges in \(\mathcal S'(\mathbb R)\) and belongs to \(L^2\) for each fixed \(\varepsilon>0\). This yields a bounded convolution operator
\[(S_\varepsilon f)(x)=\int_{\mathbb R} a_\varepsilon(x-t) f(t)\,\mathrm dt.\]

**Arithmetic energy.** For any bounded \(Q\in\mathcal B(L^2)\), set \(E_{\text{arith}}(f):=\tfrac12\,\|Q S_\varepsilon f\|_{L^2}^2\). This is finite and differentiable with gradient \(Q^*Q S_\varepsilon^* S_\varepsilon f\).

**Coupling.** A mathematically consistent coupling to the field \(\psi\) is through an auxiliary scalar trace \(u:=\Phi(\psi)\in L^2(\mathbb R)\) given by a bounded linear functional \(\Phi: \mathcal H\to L^2\). The total energy includes \(E_{\text{arith}}(u)\). No divergent terms appear, and all operators are bounded.

---

## 9. Well-posedness and stability conditions
Assume: (i) \(V_{\text{loc}}\) is \(C^2\), gauge-invariant, and \(V_{\text{loc}}(\psi)\ge c_1\,h(\psi,\psi)-c_0\); (ii) \(M\ge 0\), (iii) \(C\) bounded self-adjoint \(\ge 0\). Then for initial data \((\psi_0,\pi_0)\in H^1\times L^2\) on a Cauchy slice, the Cauchy problem is locally well-posed. Conservation of energy controls global existence in the defocusing case and small data regimes. Linearization about equilibria \(\psi_\star\) yields the self-adjoint operator \(L\); spectral gap \(\gamma=\inf\sigma(L)>0\) certifies exponential stability of the equilibrium in the energy norm.

---

## 10. Numerical protocol and certification
**Discretization.** Use a gauge-invariant variational discretization on a mesh \(\mathcal T\) with timestep \(\Delta t\). Employ a symplectic integrator for the matter sector and a discrete Hodge for Maxwell terms.

**Tolerances.** Report: CFL number, relative residual \(\le 10^{-8}\), KKT duality gap \(\le 10^{-6}\) for constrained solves, and conservation drift \(\le 10^{-5}\) per 1000 steps.

**Spectral certification.** Estimate the smallest nonzero eigenvalue of \(L\) by Lanczos with Ritz verification. Accept stability only if the certified lower bound \(\widehat\gamma\) satisfies \(\widehat\gamma-\delta>0\) where \(\delta\) bounds eigenvalue error from a posteriori estimates.

---

## 11. Units, scaling, and nondimensionalization
All quantities are dimensionless by default. If interfacing with empirical data, define an explicit scaling map \(\mathcal U: \) model units \(\to\) SI units. Every reported number states its unit system and scaling factors.

---

## 12. Minimal working example
Let \(X=\mathbb R\times \Omega\) with \(\Omega\subset\mathbb R^d\) bounded and smooth, \(A\equiv 0\), \(M=m^2\mathbf 1\), \(V_{\text{loc}}(\psi)=\tfrac{\lambda}{2}\,h(\psi,\psi)^2\), and \(C=\alpha(\mu^2-\Delta)^{-1/2}\) defined by spectral calculus on \(L^2(\Omega)\) with Dirichlet boundary conditions. The field equation becomes
\[\partial_t^2\psi-\Delta\psi + m^2\psi + \lambda\,h(\psi,\psi)\psi + \alpha\,(\mu^2-\Delta)^{-1/2}\psi=0.\]
This PDE is well-posed on \(H^1_0(\Omega)\times L^2(\Omega)\), admits an energy functional strictly bounded below for \(\lambda\ge 0\), and can be advanced by a second-order symplectic scheme. Observables include total energy, \(E_{\text c}\), and correlation functionals.

---

## 13. Verification checklist (to be attached to each study)
1. Geometry and bundles stated, including \(g\), \(E\), \(h\).
2. Action and all operators defined with domains and codomains.
3. Boundary and initial conditions specified.
4. Numerical discretization and tolerances recorded.
5. Pre-registered hypotheses and acceptance criteria listed.
6. Raw data, seeds, and action hash archived.

---

## 14. Symbols
\(X\): manifold; \(g\): metric; \(E\to X\): Hermitian bundle; \(\psi\): section; \(h\): fiber metric; \(A\): gauge potential; \(F\): curvature; \(\nabla^A\): covariant derivative; \(\Box^A\): d’Alembertian; \(M\): mass operator; \(V_{\text{loc}}\): local potential; \(C\): coherence operator; \(\Gamma\): phase space; \(\omega\): symplectic form; \(H\): Hamiltonian; \(J\): gauge current; \(\mathcal H\): Hilbert space; \(\varphi_\varepsilon\): mollifier; \(S_\varepsilon\): arithmetic convolution operator; \(Q\): bounded linear operator; \(L\): linearized operator; \(\gamma\): spectral gap.

---

## 15. Scope and limitations
The framework is agnostic to physical interpretation. Claims about empirical phenomena require an explicit mapping from mathematical observables to measured quantities, with declared noise models and error bars. Arithmetic couplings are optional and always regularized; they do not assert properties of \(\zeta\) or primes without separately stated conjectural assumptions.

