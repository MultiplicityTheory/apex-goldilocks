# Coherence Calculus — Unified Clean Specification (v1)

A self-contained specification that unifies the coherence-object calculus with a coherence-centric arithmetic layer. All constructions are mathematically well-posed and ready for formalization.

## 0. Conventions
- Ground field: \(\mathbb R\) unless explicitly complexified.
- Inner products are \(\mathbb R\)-bilinear, positive-definite.
- Sections are measurable; analysis is carried out in \(H^1\) unless noted.
- \(\langle\cdot,\cdot\rangle_H\) denotes the Hilbert inner product on a space \(H\); \(\Vert\cdot\Vert_H\) its norm.
- \nabla denotes a fixed compatible connection.

## 1. Geometry and Hilbert structure
Let \(M\) be a connected, oriented, compact Riemannian manifold with metric \(g\). Let \(\operatorname{Cl}(TM,g)\) be the real Clifford bundle with the fiberwise inner product induced by \(g\). Denote by \(\Gamma(\operatorname{Cl})\) the measurable sections.

Fix a metric- and Clifford-compatible connection \(\nabla\) on \(\operatorname{Cl}(TM,g)\). Define the Hilbert space
\[
H := H^1\big(M,\operatorname{Cl}(TM,g)\big),\qquad \langle \sigma,\tau \rangle_H := \int_M \big(\langle \nabla\sigma,\nabla\tau\rangle + \langle \sigma,\tau\rangle\big)\, d\mathrm{vol}_g.
\]
If \(\partial M\neq\varnothing\) use \(H^1_0\) with Dirichlet boundary conditions.

## 2. Orthonormal coordinates and functionals
Because \(H\) is separable, fix an \(H\)-orthonormal basis (ONB) \(\mathcal B=\{B_i\}_{i\in\mathbb N}\subset H\). Define continuous linear functionals
\[
L_i(\sigma):=\langle\sigma,B_i\rangle_H,\qquad i\in\mathbb N.
\]
If \(\sigma=\sum_i c_i B_i\) (convergent in \(H\)) then \(c_i=L_i(\sigma)\).

> One canonical choice sets \(B_j:=(\lambda_j+1)^{-1/2}\,\phi_j\), where \(\phi_j\) are eigen-sections of \(\nabla^*\nabla\) with eigenvalues \(\lambda_j\), yielding an \(H\)-ONB.

## 3. Arithmetic layer: encoding and decoding
Let \(\mathbb P\) be the set of primes. Fix a bijection \(\psi: \mathbb P\to\mathbb N\). Define the encoder \(F: \mathbb N_{\ge1}\to\ell^2(\mathbb N)\) by
\[
F(n)_i := v_{\psi^{-1}(i)}(n)\in\mathbb N,\quad \text{the exponent of the prime }\psi^{-1}(i)\text{ in }n.
\]
Each \(F(n)\) has finite support. Define the decoder on integer-valued sequences of finite support:
\[
D(e):=\prod_{i\in\mathbb N} \psi^{-1}(i)^{\,e_i} \in \mathbb N_{\ge1}.
\]
Then \(D\circ F=\mathrm{id}\) and \(F(ab)=F(a)+F(b)\) coordinatewise.

## 4. Representation sets
For \(n\ge1\) define the closed affine set
\[
\mathrm{Rep}(n):=\{\,\sigma\in H: L_i(\sigma)=F(n)_i\text{ for all }i\,\}.
\]
Nonemptiness holds because \(F(n)\) has finite support, so \(\sum_{i\in\mathrm{supp}F(n)} F(n)_i B_i\in\mathrm{Rep}(n)\). The tangent space at any point is \(\ker K\) with \(K:H\to\ell^2\), \(K(\sigma)=(L_i(\sigma))_i\).

## 5. Energy and coherence embedding
Define the strictly convex, coercive quadratic energy \(E(\sigma):=\tfrac12\Vert\sigma\Vert_H^2\). For \(n\ge1\) define the embedding
\[
\operatorname{embed}(n):=\underset{\sigma\in\mathrm{Rep}(n)}{\arg\min}\;E(\sigma).
\]
Existence and uniqueness follow from strict convexity and closedness of \(\mathrm{Rep}(n)\). Writing \(e:=F(n)\), the unique minimizer is the finite sum
\[
\operatorname{embed}(n)=\sum_{i\in\mathbb N} e_i\,B_i.
\]

## 6. Algebraic laws
For \(a,b\ge1\) and \(k\in\mathbb N\):
1. \(\operatorname{embed}(1)=0\).
2. \(\operatorname{embed}(ab)=\operatorname{embed}(a)+\operatorname{embed}(b)\).
3. \(\operatorname{embed}(a^k)=k\,\operatorname{embed}(a)\).
These follow from \(F\) and linearity of coefficients.

## 7. Resonance functionals (multiplicative characters)
### 7.1 Positive reals
Choose parameters \(\alpha=(\alpha_i)_{i\in\mathbb N}\in(0,\infty)^{\mathbb N}\) with \(\theta_i:=\log\alpha_i\in\ell^2\). Define
\[
R_\alpha(n):=\prod_i \alpha_i^{\,F(n)_i}=\exp\big(\langle F(n),\theta\rangle\_{\ell^2}\big)\in(0,\infty).
\]
Then \(R_\alpha(ab)=R_\alpha(a)R_\alpha(b)\). Gradients for learning obey
\[
\frac{\partial}{\partial\theta_i}\log R_\alpha(n)=F(n)_i,\qquad \nabla_\theta\log R_\alpha(n)=F(n).
\]
### 7.2 Unit circle
If \(\alpha_i\in\mathrm U(1)\) with \(\alpha_i=e^{\mathrm i\theta_i}\) and \(\theta\in\ell^2\), then
\[
\chi_\alpha(n):=\prod_i \alpha_i^{\,F(n)_i}=e^{\mathrm i\langle F(n),\theta\rangle}
\]
is a unitary multiplicative character. If \(\alpha_i\in\{\pm1\}\) with finite support, \(\chi_\alpha\) is a \(\mathbb Z_2\)-valued character closed under pointwise multiplication (XOR in exponents).

## 8. Symmetries
Let a group \(G\) act on \(H\) by an isometric representation \(\rho:G\to O(H)\) and on the index set by a permutation \(\pi:G\to\mathrm{Sym}(\mathbb N)\) such that
\[
\rho(g)B_i = B_{\pi(g)(i)}\quad \text{for all }g\in G,\ i\in\mathbb N.
\]
Transport \(\psi\) to \(\psi_g:=\psi\circ\pi(g)^{-1}\). Then
\[
\rho(g)\,\operatorname{embed}\_{\psi}(n)=\operatorname{embed}\_{\psi_g}(n),\qquad \Vert\operatorname{embed}(n)\Vert_H\ \text{is }G\text{-invariant}.
\]
If \(\pi\) permutes only indices outside \(\mathrm{supp}F(n)\) then \(\rho(g)\operatorname{embed}(n)=\operatorname{embed}(n)\).

## 9. Decoding and exact recovery
Define the coefficient readout \(\mathrm{coeff}(\sigma):=(L_i(\sigma))_{i\in\mathbb N}\in\ell^2\). When \(\mathrm{coeff}(\sigma)\in\mathbb N^{(\mathbb N)}\), set
\[
\mathrm{decode}(\sigma):=D\big(\mathrm{coeff}(\sigma)\big)\in\mathbb N_{\ge1}.
\]
Then \(\mathrm{decode}(\operatorname{embed}(n))=n\). If \(\tilde\sigma=\operatorname{embed}(n)+\eta\) with \(\eta\in H\), then
\[
\Vert\mathrm{coeff}(\tilde\sigma)-F(n)\Vert_{\ell^2}\le\Vert\eta\Vert_H.
\]
If each coefficient error is \(<\tfrac12\), rounding recovers \(F(n)\) and hence \(n\).

## 10. Finite truncations for computation
Let \(P_N\) be the orthogonal projection onto \(\mathrm{span}\{B_1,\dots,B_N\}\). Define
\[
\operatorname{embed}_N(n):=P_N\,\operatorname{embed}(n)=\sum_{i\le N}F(n)_i B_i.
\]
Since \(F(n)\) has finite support, there exists \(N\) with \(\operatorname{embed}_N(n)=\operatorname{embed}(n)\).

## 11. Optional complexification
When complex scalars are needed, use \(H_\mathbb C:=H\otimes_\mathbb R\mathbb C\) with the canonical sesquilinear inner product \(\langle u\otimes z, v\otimes w\rangle= z\overline w\,\langle u,v\rangle_H\). All definitions above extend verbatim. Characters \(\chi_\alpha\) and symmetry actions lift unitarily.

## 12. Interfaces summary
- **State space:** \(H=H^1(M,\operatorname{Cl}(TM,g))\).
- **Coordinates:** \(L_i(\sigma)=\langle\sigma,B_i\rangle_H\) for an \(H\)-ONB \(\{B_i\}\).
- **Encoder/decoder:** \(F(n)=(v_{\psi^{-1}(i)}(n))_i\), \(D(e)=\prod_i\psi^{-1}(i)^{e_i}\).
- **Representation sets:** \(\mathrm{Rep}(n)=\{\sigma:L_i(\sigma)=F(n)_i\ \forall i\}\).
- **Energy:** \(E(\sigma)=\tfrac12\Vert\sigma\Vert_H^2\).
- **Embedding:** \(\operatorname{embed}(n)=\sum_i F(n)_i B_i\).
- **Resonance:** \(R_\alpha(n)=e^{\langle F(n),\theta\rangle}\) with \(\theta\in\ell^2\); unitary variant \(\chi_\alpha(n)=e^{\mathrm i\langle F(n),\theta\rangle}\).
- **Symmetry:** isometric \(G\)-action permuting the ONB induces equivariance of \(\operatorname{embed}\).
- **Stability:** coefficient error \(\ell^2\)-bounded by \(\Vert\eta\Vert_H\); rounding recovers exact \(n\) under half-integer margins.

This unified calculus ties the geometric Hilbert setting to an arithmetic interface via prime-exponent coordinates, with linear algebra control, multiplicative characters for signal design, and symmetry-aware embeddings suitable for both proof assistants and numerical implementations.

