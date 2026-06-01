# Hologram — Resonant Homomorphic Framework — Math‑Clean Revision

## 0. Overview

We formalize a compact algebra for resonance computation, a toggle‑space model with a multiplicative evaluation map, a logic of resource budgets over \(\mathbb Z_{96}\), and a homomorphic collapse to \(\mathbb Z_2\). A factoring scaffold using homomorphic resonance is specified at the level of algebraic contracts and verifiable identities.

## 1. Algebraic base and notation

- Let \(C:=\mathbb Z_{96}\) with operations \(\oplus\) (addition mod 96) and \(\otimes\) (multiplication mod 96). \((C,\oplus,\otimes)\) is a commutative semiring with units \(0\) and \(1\).
- Let \(B:=\mathbb Z_2^8\) be the 8‑bit toggle space. Elements are written as bit‑vectors \(b=(b_0,\dots,b_7)\) or integers \(b\in\{0,\dots,255\}\). The group law is bitwise XOR, denoted \(b\oplus_2 b'\).
- Parameters: positive reals \(\alpha_i>0\) with constraints
  1. \(\alpha_0=1\),
  2. unity pair \((\alpha_4,\alpha_5)\) with \(\alpha_4\alpha_5=1\).
- **Non‑degeneracy.** Products formed from \(\{\alpha_1,\alpha_2,\alpha_3,\alpha_6,\alpha_7\}\) are all distinct; moreover \(\alpha_4\notin\{1,\alpha_5\}\).

## 2. Resonance evaluation on toggles

Define the **pair‑normalized evaluation** \(R:B\to\mathbb R_{>0}\): for \(b\in B\),

$$
R(b)\;:=\;\prod_{i\in\{0,1,2,3,6,7\}} \alpha_i^{b_i}\;\cdot\;\begin{cases}
\alpha_4 & (b_4,b_5)=(1,0),\\
1 & (b_4,b_5)\in\{(0,0),(1,1)\},\\
\alpha_5 & (b_4,b_5)=(0,1).\end{cases}
$$

This enforces the unity‑pair constraint by using exponent \(d(b)\in\{-1,0,1\}\) on \(\alpha_4\) with \(\alpha_5=\alpha_4^{-1}\). \(\alpha_0=1\) makes bit 0 neutral.

**Homomorphism on restricted groups.** If \(H\le B\) satisfies \(b_4=b_5\) for all \(b\in H\), then \(R(b\oplus_2 b')=R(b)R(b')\) for \(b,b'\in H\).

## 3. Image size and basic statistics

**Theorem (R96).** Under non‑degeneracy, \(|R(B)|=96\).

*Reason.* Bits \(\{1,2,3,6,7\}\) contribute \(2^5\) distinct products. The pair \((b_4,b_5)\) yields three distinct multiplicative states \(\{\alpha_4,1,\alpha_5\}\). Total \(3\cdot 2^5=96\).

**Fairness schedule.** One cycle enumerates \(b=0,1,\dots,255\). Three cycles give 768 steps. Define

$$
\mu:=\frac{1}{256}\prod_{i=0}^7(1+\alpha_i),\]
where the product uses \(\alpha_5=\alpha_4^{-1}\) and \(\alpha_0=1\). The per‑cycle variance is
\[\mathrm{Var}=\frac{1}{256}\prod_{i=0}^7(1+\alpha_i^2)-\mu^2.\]

## 4. Homomorphic substructures in \(B\)
Let \(S:=\{0,1,2,3,6,7\}\). For any subgroup \(T\le \mathbb Z_2^S\), the set
\[H_T:=\{b\in B: b_4=b_5\text{ and }(b_i)_{i\in S}\in T\}
$$

is a subgroup of \(B\) on which \(R\) is a group homomorphism into \((\mathbb R_{>0},\cdot)\). Examples include the trivial group, coordinate axes in \(S\), and their direct sums.

## 5. Resonance Logic (RL)

**Budgets.** Associate to each sequent a budget \(r\in C\). Primitive composition is
\(\mathrm{RI:}\quad r_1,\dots,r_n\;\vdash\; \rho:=r_1\otimes\cdots\otimes r_n\in C.\)
Structural rules use \(\oplus\) for aggregation.

**Klein complement.** Define \(\perp[r]:=[-r]_{96}\). Then \(\perp(\perp[r])=r\) and \(\perp[r\oplus s]=\perp[r]\oplus\perp[s]\). No general De Morgan law is claimed for \(\otimes\).

**Soundness contract.** Any derivation computes its conclusion budget as the \(\otimes\)‑product of the premise budgets, up to commutativity and associativity in \(C\).

## 6. Collapse semantics

Define the parity morphism \(\kappa:C\to\mathbb Z_2\) by \(\kappa([r]_{96})=r\bmod 2\). Then
\(\kappa(r\oplus s)=\kappa(r)\oplus_2\kappa(s),\qquad \kappa(r\otimes s)=\kappa(r)\cdot\kappa(s).\)
Extend \(\kappa\) to sequents by replacing every numeric budget with its image.

**Proposition (Homomorphic collapse).** If a numeric RL derivation yields conclusion budget \(\rho\) from premises \(r_1,\dots,r_n\), then the collapsed derivation over \(\mathbb Z_2\) yields \(\kappa(\rho)=\prod_k \kappa(r_k)\). Thus, parity‑level proofs are images of numeric proofs.

## 7. Homomorphic Resonance Factorization (HRF‑F)

This section specifies an abstract interface sufficient to design and test factoring pipelines without fixing an embedding.

### 7.1 Encodings and lifts

- **Encoding.** Choose a map \(E:\mathbb N\to B^{*}\) into finite sequences of toggles. Requirement: \(E(1)=\epsilon\) (empty sequence) and \(E(mn)=E(m)\concat E(n)\).
- **Sequence evaluation.** Lift \(R\) to sequences by multiplicative accumulation:

$$
$$

- **Quantization to budgets.** Fix a quantizer \(q:\mathbb R_{>0}\to C\). Define the budget of a sequence by \(Q:=q\circ R^{*}\).

### 7.2 Interference and accumulation

- **Histogram lift.** For a sequence \(s\in B^{*}\), let \(\mathrm{hist}(s)\in C^{B}\) record counts of each \(b\) reduced modulo 96.
- **Accumulator.** Choose weights \(w\in C^{B}\) and define \(\Phi(s):=\sum_{b\in B} w_b\otimes \mathrm{hist}(s)[b]\in C.\)
- **Interference.** For sequences \(s,t\), define \(I(s,t):=\Phi(s)\oplus\Phi(t)\oplus\Phi(s\concat t)\). Other symmetric bilinear forms over \(C\) are admissible.

### 7.3 Factor signal

- **Design goal.** Select \(E,q,w\) so that for composite \(N=pq\) with primes \(p\ne q\), the statistic \(I(E(p),E(q))\) separates from the null distribution induced by \(E(r)\) for random \(r\).
- **Verification protocol.** Specify a test function \(\mathrm{match}:C\to\{0,1\}\) and evaluate false‑positive/negative rates over sampled ranges. All thresholds are stated in \(C\); optional parity collapse via \(\kappa\) gives a binary surrogate.

## 8. Acceptance tests and reference identities

The following are implementation‑level acceptance tests:

1. **R96 count.** With non‑degenerate parameters, \(|R(B)|=96\); with any explicit degeneracy (e.g., \(\alpha_1=\alpha_2\)) the count drops below 96.
2. **Triple‑cycle identity.** Enumerated sum over three cycles equals \(3\prod_i(1+\alpha_i)\); per‑cycle mean and variance match the closed forms.
3. **Homomorphism checks.** For any subgroup \(H_T\) as in §4 and any \(b,b'\in H_T\), \(R(b\oplus_2 b')=R(b)R(b')\).
4. **Collapse homomorphism.** For random budgets \(r,s\in C\), verify \(\kappa(r\oplus s)=\kappa(r)\oplus_2\kappa(s)\) and \(\kappa(r\otimes s)=\kappa(r)\cdot\kappa(s)\); extend to RI compositions.

## 9. Appendices

### A. Proof sketch for R96

Group the 256 bit‑patterns by the pair \((b_4,b_5)\). Pair‑normalization maps these four cases to three multiplicative states \(\{\alpha_4,1,\alpha_5\}\). The remaining five independent toggles generate \(2^5\) distinct products under non‑degeneracy. Therefore the image size is \(3\cdot 2^5=96\).

### B. Triple‑cycle formulas

For one cycle, \(\sum_{b\in B} R(b)=\prod_{i=0}^{7}(1+\alpha_i)\) by independence of bits under multiplication. The three‑cycle constant is three times this value. The variance follows by replacing \(\alpha_i\) with \(\alpha_i^2\) in the first moment and subtracting \(\mu^2\).

If \(b_4=b_5\) and \(b'_4=b'_5\), then in pair‑normalized evaluation the \(\alpha_4/\alpha_5\) factors are both 1, and the map reduces to a pure product over indices in \(S\). Because exponents add under XOR in \(\mathbb Z_2\), multiplication of the corresponding \(\alpha_i\) realizes the group law.

### D. Collapse proposition

\(\kappa\) is a semiring morphism. Any numeric derivation in RL computes a polynomial in the premise budgets with coefficients in \(\mathbb N\). Applying \(\kappa\) maps that polynomial to its image over \(\mathbb Z_2\), yielding the stated result.

---

**Implementation tip.** Use pair‑normalized evaluation for \((\alpha_4,\alpha_5)\) to avoid numerical drift and to keep the three‑state structure explicit.

