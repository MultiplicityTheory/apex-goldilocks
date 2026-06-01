# Universal Resonance Calculus — Formal Core

## 1) Universe and Signature
- **Base set**: \(\mathbb N_0 = \{0,1,2,\dots\}\).
- **Lifted set**: \(D = \mathbb N_0 \cup \{\bot\}\).
- **Operations on** \(D\):
  - Addition \(\oplus: D\times D\to D\)
  - Multiplication \(\otimes: D\times D\to D\)
  - Distinguished element \(\bot\) (error/undefined sentinel)

**Axioms**
1. \((\mathbb N_0, +,\cdot, 0,1)\) is the standard commutative semiring.
2. For \(x,y\in D\): if \(x=\bot\) or \(y=\bot\) then \(x\oplus y=\bot\) and \(x\otimes y=\bot\). Otherwise \(x\oplus y=x+y\) and \(x\otimes y=xy\).
3. \(\bot\) is absorbing for both operations and not equal to any natural number.

> This keeps the standard arithmetic on \(\mathbb N_0\) while offering a total, conservative extension via \(\bot\).

---

## 2) Binary Encoding and Weights
- For \(n\in \mathbb N_0\) let \(b_i(n)\in\{0,1\}\) be the \(i\)-th bit of its binary expansion: \(n = \sum_{i\ge 0} b_i(n)\,2^i\).
- Fix a **strictly positive absolutely summable** weight sequence \(w\in \ell^1(\mathbb N_0)\) with \(w_i>0\) and \(\sum_{i\ge 0} w_i = W < \infty\).
- Choose a parameter vector \(\alpha=(\alpha_i)_{i\ge 0}\) with \(\alpha_i\in [\varepsilon,1-\varepsilon]\) for some fixed \(\varepsilon\in(0,\tfrac12]\.\)

**Canonical choice**: \(w_i = 2^{-(i+1)}\) so \(W=1\).

---

## 3) Resonance Map
Define the **resonance** of \(n\) as
\[
  r_\alpha(n) \;=\; \sum_{i=0}^{\infty} \alpha_i\, b_i(n)\, w_i\;\in [0,W].
\]

**Properties**
1. **Boundedness**: \(0\le r_\alpha(n)\le W\) for all \(n\).
2. **Lipschitz in parameters**: for any \(\alpha,\beta\), \(|r_\alpha(n)-r_\beta(n)|\le \|\alpha-\beta\|_\infty\,W\).
3. **Tail error under truncation**: with the truncation level \(K\), let \(r_\alpha^{(K)}(n)=\sum_{i=0}^{K-1} \alpha_i b_i(n) w_i\). Then for all \(n\)
\[ |r_\alpha(n)-r_\alpha^{(K)}(n)|\le \sum_{i\ge K} w_i. \]
4. **Aperiodicity (generic)**: if the set \(\{\alpha_i w_i\}_{i\ge 0}\) is linearly independent over \(\mathbb Q\) and infinitely many \(w_i>0\), then the map \(n\mapsto r_\alpha(n)\) is aperiodic.

> Use the infinite support of \(w\) to avoid finite-bit periodicity. Finite computations use \(K\) with a certified tail bound.

---

## 4) Discrete Calculus on \(r_\alpha\)
Define forward difference and discrete Hessian:
\[
\Delta r_\alpha(n) = r_\alpha(n{+}1)-r_\alpha(n),\quad
\Delta^2 r_\alpha(n) = \Delta r_\alpha(n{+}1)-\Delta r_\alpha(n).
\]

**Local minima (strict)**: \(n\) is a strict local minimum of \(r_\alpha\) if \(\Delta r_\alpha(n{-}1)<0\) and \(\Delta r_\alpha(n)>0\). Ties are broken by choosing the smallest index in any flat run.

All definitions apply verbatim to \(r_\alpha^{(K)}\).

---

## 5) Page Structure (Finite Windows)
Fix \(K\ge 1\) and set the **page base** \(B=2^K\). For \(n\in \mathbb N_0\):
\[
\text{page}(n)=\big\lfloor n/B\big\rfloor,\qquad \text{offset}(n)=n\bmod B.
\]
Analyses and training below operate per page or on a finite union of pages.

---

## 6) Finite-Window Learning Task
Let a finite index set \(W\subset \mathbb N_0\) be given. Optionally let labels \(y:W\to\{0,1\}\) be given (e.g., any binary property of integers). Define the feature map
\[
\varphi_\alpha^{(K)}(n)\;=\;\big( r_\alpha^{(K)}(n),\; \Delta r_\alpha^{(K)}(n),\; \Delta^2 r_\alpha^{(K)}(n) \big)\in \mathbb R^3.
\]
Pick a smooth loss \(\ell: \mathbb R^3\times\{0,1\}\to\mathbb R_{\ge 0}\) and set the empirical loss
\[
L_{W,K}(\alpha)\;=\;\frac{1}{|W|}\sum_{n\in W} \ell\big(\varphi_\alpha^{(K)}(n),\, y(n)\big).
\]

**Projection set**: \(A_K=[\varepsilon,1{-}\varepsilon]^K\) (compact, convex in \(\mathbb R^K\)).

**Update map** (fixed step \(\eta>0\)):
\[
F(\alpha)\;=\;\Pi_{A_K}\big(\alpha - \eta\,\nabla L_{W,K}(\alpha)\big),\quad F:A_K\to A_K.
\]
If \(\ell\) is \(C^1\), then \(F\) is continuous. By **Brouwer**, \(F\) admits a fixed point \(\alpha^\star\in A_K\), i.e., \(F(\alpha^\star)=\alpha^\star\).

> \(\alpha^\star\) is a stationary point of the projected learning dynamics on the finite-dimensional parameter space.

---

## 7) Guarantees and Bounds
- **Operator-norm bound**: define the multiplication operator \((M_{r_\alpha}f)(n)=r_\alpha(n)f(n)\) on \(\ell^2(\mathbb N_0)\). Then \(\|M_{r_\alpha}\|=\sup_n |r_\alpha(n)|\le W\).
- **Shift-coupled operator**: with the shift \((Sf)(n)=f(n{+}1)\) and any \(\rho\in\mathbb R\), set \(T_{\alpha,\rho}=S-\rho M_{r_\alpha}\). Then \(\|T_{\alpha,\rho}\|\le 1+|\rho|\,W\).
- **Truncation error**: replacing \(r_\alpha\) by \(r_\alpha^{(K)}\) perturbs all values and finite-difference features by at most \(2\sum_{i\ge K} w_i\) pointwise.
- **Aperiodicity in practice**: choosing \(\alpha_i\) i.i.d. from a continuous distribution on \([\varepsilon,1{-}\varepsilon]\) yields aperiodicity with probability 1.

---

## 8) Algorithms (Reference)
**A) Encoding and Resonance (truncated)**
1. Input: \(n, K, \alpha\in A_K, w\).
2. Compute bits \(b_0(n),\dots,b_{K-1}(n)\).
3. Return \(r_\alpha^{(K)}(n)=\sum_{i=0}^{K-1} \alpha_i b_i(n) w_i\).

**B) Minima Detection on a Window**
1. Input: window \(W=\{m,\dots,m+L\}\).
2. Compute \(r,\Delta r,\Delta^2 r\) for all indices in \(W\).
3. Output strict local minima \(\{ n\in W: \Delta r(n{-}1)<0,\; \Delta r(n)>0\}\) with tie-breaking by smallest index.

**C) Projected Gradient Step**
1. Input: \(\alpha\in A_K\), labels \(y\) on \(W\), loss \(\ell\), step \(\eta\).
2. Compute \(\nabla L_{W,K}(\alpha)\) via autodiff or analytic gradients.
3. Update \(\alpha\leftarrow \Pi_{A_K}(\alpha-\eta\,\nabla L_{W,K}(\alpha))\).

---

## 9) Optional Spectral View
Define \(T_{\alpha,\rho}=S-\rho M_{r_\alpha}\) on \(\ell^2\). Spectral radii \(\rho(T_{\alpha,\rho})\) control linear stability of the coupled shift–resonance dynamics. With \(\|M_{r_\alpha}\|\le W\), any small enough \(|\rho|\) ensures \(\|T_{\alpha,\rho}\|<1+\delta\) for prescribed \(\delta>0\). This gives a tunable stability certificate for algorithms driven by \(r_\alpha\).

---

## 10) Parameter Choices and Defaults
- Weights: \(w_i=2^{-(i+1)}\).
- Truncation: choose \(K\) so that \(\sum_{i\ge K} w_i \le \tau\) for tolerance \(\tau\).
- Parameter box: \(A_K=[\varepsilon,1{-}\varepsilon]^K\) with \(\varepsilon=10^{-3}\) unless otherwise required.
- Loss example: logistic or squared loss on the features \(\varphi_\alpha^{(K)}(n)\).

---

## 11) Notation Summary
- \(\mathbb N_0\): nonnegative integers. \(D=\mathbb N_0\cup\{\bot\}\).
- \(b_i(n)\): binary bits. \(w\in\ell^1\): positive weights, sum \(W\).
- \(\alpha\): parameter vector; \(A_K\): compact convex parameter set.
- \(r_\alpha\), \(r_\alpha^{(K)}\): resonance (full and truncated).
- \(\Delta,\Delta^2\): discrete gradient and Hessian.
- **page**, **offset** with base \(B=2^K\).
- \(L_{W,K}\): empirical loss; \(F\): projected gradient map; \(\alpha^\star\): Brouwer fixed point.

---

### Minimal Example (Concrete Numbers)
Let \(w_i=2^{-(i+1)}\), \(K=12\), \(\varepsilon=10^{-3}\). For a single page take \(W=\{0,1,\dots,2^{12}-1\}\). Initialize \(\alpha_i=\tfrac12\). Compute \(r_\alpha^{(12)}\), its differences, and the minima set. If labels \(y\) are supplied on \(W\), run the projected gradient map \(F\) until a stationary point is reached. Tail error is \(\le 2^{-12}\).

