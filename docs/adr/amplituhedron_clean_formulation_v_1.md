# Amplituhedron — Clean Formulation v1

## 0. Scope
A self-contained definition of the amplituhedron \(\mathcal A_{k,m,n}(Z)\), its basic properties, and a minimal, correct path to the canonical form. Assumes familiarity with linear algebra and differential forms.

## 1. Notation and preliminaries
- Work over \(\mathbb R\). Projective space \(\mathbb P^{r}\) is lines through the origin in \(\mathbb R^{r+1}\).
- The Grassmannian \(\mathrm{Gr}(k,N)\) is the set of \(k\)-planes in \(\mathbb R^N\). Points are represented by full-rank \(k\times N\) matrices modulo left \(\mathrm{GL}_k\).
- Plücker coordinates for \(\mathrm{Gr}(k,N)\) are the \(k\times k\) minors of a representative matrix. They satisfy the Plücker relations and are defined up to an overall scale.
- The **positive Grassmannian** \(G_+(k,n)\) consists of \(k\times n\) real matrices of rank \(k\) whose maximal ordered minors are all strictly positive, modulo left \(\mathrm{GL}_k\).
- Indices are cyclic modulo \(n\): \(i\equiv i+n\).

## 2. External data \(Z\)
Let \(Z\in \mathbb R^{n\times (k+m)}\) have full rank \(k+m\). Assume **positivity of \(Z\)**: all ordered \((k+m)\) minors of \(Z\) are strictly positive. Columns are \(Z_1,\dots,Z_n\) with a fixed cyclic order.

Two data sets \(Z, Z'\) related by \(Z' = Z\,g\) with \(g\in \mathrm{GL}_{k+m}^+\) encode the same amplituhedron as subsets of projective space. We work projectively in \(\mathrm{Gr}(k,k+m)\) or \(\mathbb P^{\binom{k+m}{k}-1}\) as convenient.

## 3. Definition of the amplituhedron
Let \(C\in G_+(k,n)\). Define a map
\[
\Phi_Z:\ G_+(k,n) \to \mathrm{Gr}(k,k+m),\qquad C\mapsto Y= \mathrm{rowspan}(C Z).
\]
The **amplituhedron** is the image
\[
\mathcal A_{k,m,n}(Z) \coloneqq \{\, Y = \mathrm{rowspan}(C Z)\ :\ C\in G_+(k,n)\,\} \subset \mathrm{Gr}(k,k+m).
\]
Properties:
- \(\dim \mathcal A_{k,m,n}(Z) = k\,m\).
- \(\mathcal A_{k,m,n}(Z)\) is invariant under left multiplication \(C\mapsto gC\) with \(g\in \mathrm{GL}_k\) and right multiplication \(Z\mapsto Z h\) with \(h\in \mathrm{GL}_{k+m}^+\) (projectively).
- Boundaries of \(\mathcal A_{k,m,n}(Z)\) are images of boundaries of positroid cells in \(G_+(k,n)\) under \(\Phi_Z\).

## 4. Canonical form
There exists a unique top-degree differential form \(\Omega_{k,m,n}(Z)\) on \(\mathcal A_{k,m,n}(Z)\) characterized by:
1. **Logarithmic singularities only** on all boundary components of \(\mathcal A_{k,m,n}(Z)\).
2. **Unit leading residues** on each top-dimensional cell in any triangulation of \(\mathcal A_{k,m,n}(Z)\).

Computation strategy:
- Choose a positroid triangulation \(\{\mathcal C_\alpha\}\) of \(G_+(k,n)\) such that \(\Phi_Z\) is birational on each \(\mathcal C_\alpha\).
- On each \(\mathcal C_\alpha\), use positive coordinates \((x_1,\dots,x_{km})\) in which the pullback of \(\Omega\) is a product of \(\mathrm d\!\log\) factors:
  \[ \Phi_Z^*\Omega\big|_{\mathcal C_\alpha} = \mathrm d\!\log x_1\wedge\cdots\wedge \mathrm d\!\log x_{km}. \]
- Pushforward to \(Y\)-space and sum over \(\alpha\):
  \[ \Omega_{k,m,n}(Z) = \sum_\alpha (\Phi_Z)_*\big( \mathrm d\!\log x_1\wedge\cdots\wedge \mathrm d\!\log x_{km} \big). \]
Independence of triangulation follows from local compatibility of leading residues on shared boundaries.

## 5. Examples
### 5.1 Polygon case: \(k=1,\ m=2\)
- Data: \(Z\in \mathbb R^{n\times 3}\) with all ordered \(3\times 3\) minors positive. The image \(\mathcal A_{1,2,n}(Z)\subset \mathbb P^2\) is the convex \(n\)-gon with vertices given by consecutive pairs \([Z_i Z_{i+1}]\).
- Canonical form: Let a fan triangulation at vertex 1 give triangles \((1,i,i{+}1)\) for \(i=2,\dots,n{-}1\). The canonical form is the sum of triangle canonical forms pulled forward from positive coordinates on each cell. Different triangulations produce the same \(\Omega_{1,2,n}(Z)\).

### 5.2 \(k=1,\ m=4\)
- \(\mathcal A_{1,4,n}(Z)\subset \mathbb P^4\) is a cyclic polytope. The canonical form is computed by any regular triangulation compatible with the cyclic order; it again equals the sum of simplex canonical forms with unit residues.

### 5.3 Minimal nontrivial \(k>1\)
- Take \(k=2, m=2, n\ge 4\). Then \(\dim \mathcal A=4\). Choose a plabic-graph triangulation of \(G_+(2,n)\), assign cluster-like positive coordinates to each top cell, evaluate \(\mathrm d\!\log\) forms, and pushforward.

## 6. Boundaries and positroid stratification
- Positroid cells in \(G_+(k,n)\) are labeled by decorated permutations or plabic graphs. Codimension-one boundaries correspond to the vanishing of a single Plücker coordinate consistent with positivity.
- Under \(\Phi_Z\), these cells map to boundaries of \(\mathcal A_{k,m,n}(Z)\). The canonical form can be reconstructed by requiring that residues on all such boundaries match those induced from lower-dimensional amplituhedra.

## 7. Symmetries
- \(Z\) with ordered positive minors implies invariance under the dihedral action on indices \(i\mapsto i\pm 1\) and reversal, realized projectively.
- \(\mathrm{GL}_{k+m}^+\) acts by right multiplication on \(Z\), yielding equivalent embeddings of \(\mathcal A_{k,m,n}\).

## 8. Algorithms at a glance
1. Fix \(Z\) with positive ordered minors.
2. Choose a positroid triangulation of \(G_+(k,n)\) (e.g., via a plabic graph).
3. For each top cell, pick positive coordinates making the pullback a \(\mathrm d\!\log\) volume form.
4. Pushforward through \(\Phi_Z\) and sum over cells.
5. Verify triangulation-independence by matching residues on shared boundaries.

## 9. Sanity checks
- **Dimension:** \(\dim G_+(k,n)=k(n{-}k)\). The map \(\Phi_Z\) has fiber dimension \(k(n{-}k{-}m)\), hence \(\dim \mathcal A=k m\).
- **Convexity for \(k=1\):** positivity of ordered minors implies the image is a cyclic polytope, thus convex.
- **Projective invariance:** \(\Omega\) is defined up to overall scale fixed by the unit-residue normalization.

## 10. Minimal glossary
- **Plücker bracket** \(\langle i_1\cdots i_k\rangle\): the \(k\times k\) minor of a matrix formed from columns \(i_1,\dots,i_k\).
- **Canonical form:** a differential form with logarithmic singularities on all boundaries and unit residues on top cells; unique on a positive geometry.
- **Positive geometry:** a pair \((X,X_{\ge 0})\) where \(X\) is a complex projective variety and \(X_{\ge 0}\subset X(\mathbb R)\) carries a unique canonical form.

## 11. References for further study
- Positive Grassmannian, plabic graphs, and cluster coordinates.
- Amplituhedron and positive geometries in scattering amplitudes.

---
**End of document.**

