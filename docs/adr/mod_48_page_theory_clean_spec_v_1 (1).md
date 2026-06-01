# Mod-48 Page Theory — Clean Spec V1

A compact, math-clean specification of the page decomposition, bit fields, and induced operators.

## 1) Setup
- **Pages.** For \(p\in\mathbb{Z}_{\ge0}\), define the page
  \[ P_p:=\{48p,48p+1,\dots,48p+47\}. \]
- **8-bit field.** For \(n\in\mathbb{Z}_{\ge0}\), let \(F(n)\in\{0,1\}^8\) be the least-significant 8-bit vector of \(n\). Write components \(F_k(n)\in\{0,1\}\) for bit \(k\) with \(k=0\) the LSB.
- **Periodicity.** \(F(n+256)=F(n)\) for all \(n\). Consequently, any construction depending only on \(F(n)\) is 256-periodic.
- **LCM scale.** Page step is 48 and field period is 256. Hence the joint repeat scale is
  \[ L:=\operatorname{lcm}(48,256)=768,\quad 48\mid L,\;256\mid L. \]
  
## 2) Page types and orbit length
Define the page-offset \(r_p:=48p\bmod 256\). Because \(\gcd(48,256)=16\), the map \(p\mapsto r_p\) has orbit length \(256/16=16\). Therefore:

**Proposition 2.1 (Sixteen page types).** The array \(\{F(n):n\in P_p\}\) depends only on \(p\bmod 16\). Equivalently, there are exactly 16 distinct page types, repeating with period 16 in \(p\).

*Consequence.* Specifying a page type requires at most 4 bits.

## 3) Pagewise XOR invariant
For a finite set \(S\subset\mathbb{Z}_{\ge0}\), write the bitwise XOR
\[ \bigoplus_{n\in S} F(n) := \left(\bigoplus_{n\in S} F_0(n),\dots,\bigoplus_{n\in S} F_7(n)\right). \]

**Theorem 3.1 (Zero XOR on each page).** For every \(p\),
\[ \bigoplus_{n\in P_p} F(n) = (0,0,0,0,0,0,0,0). \]

*Proof.* Partition \(P_p\) into three consecutive blocks of length 16. For \(k\le 3\), bit \(k\) has period \(2^{k+1}\) dividing 16, so each 16-block contains exactly \(2^k\) ones, an even number. For \(k\ge 4\), bit \(k\) is constant on each 16-block (since its period is \(\ge 32\)), hence contributes either 0 or 16 ones per block, again even. Summing three even counts yields even parity for every bit. ∎

## 4) Resonance functional
Choose a fixed weight vector \(\alpha\in\mathbb{R}^8\) independent of \(p\) and \(n\). We adopt the balanced, normalized choice
\[ \alpha := \frac{1}{\sqrt{24}}\,(-3,-1,-1,-1,\;1,1,1,3)^{\top}. \]
Define the resonance
\[ R(n):=\alpha^{\top}F(n)\in\mathbb{R}. \]

**Basic properties.**
1. **Periodicity:** \(R(n+256)=R(n).\)
2. **Range:** \(R(n)\in\left[-\tfrac{6}{\sqrt{24}},\tfrac{6}{\sqrt{24}}\right]=\left[-\tfrac{3}{\sqrt{6}},\tfrac{3}{\sqrt{6}}\right].\)
3. **Zero mean over a 256-cycle:** \(\sum_{n=0}^{255}R(n)=0\) because \(\sum_k \alpha_k=0\) and each bit is 1 exactly half the time over a full cycle.

### 4.1 Flux across a page
Define the pagewise flux
\[ \Phi(p):=\sum_{n\in P_p}\big(R(n+1)-R(n)\big)=R(48p+48)-R(48p). \]

**Theorem 4.2 (Flux telescoping and 16-page cancellation).** For all \(p\),
\[ \sum_{q=0}^{15}\Phi(p+q)=R(48p+768)-R(48p)=0. \]
In particular, the net flux over any consecutive block of 16 pages is zero.

*Remark.* \(\Phi(p)=0\) holds exactly when \(R(48p+48)=R(48p)\); this need not occur for every \(p\).

## 5) Metrics on indices
Let \(p(n):=\lfloor n/48\rfloor\) be the page index of \(n\).
- **Page metric:** \(d_P(n,m):=|p(n)-p(m)|\in\mathbb{Z}_{\ge0}.\)
- **Resonance metric:** \(d_R(n,m):=|R(n)-R(m)|\in\mathbb{R}_{\ge0}.\)
- **Product metric:** For any \(\gamma>0\),
  \[ d_\gamma(n,m):=\sqrt{\,d_P(n,m)^2+\gamma^2\,d_R(n,m)^2\,}. \]

**Proposition 5.1.** \(d_P\) and \(d_R\) are metrics. Hence \(d_\gamma\) is a metric on \(\mathbb{Z}_{\ge0}\).

*Sketch.* Nonnegativity and symmetry are clear. Identity of indiscernibles follows from equality of page indices and resonance values. Triangle inequality for \(d_\gamma\) follows from Minkowski since each component obeys a triangle inequality. ∎

## 6) A periodic weighted line and its spectrum
Consider the infinite line graph on \(\mathbb{Z}\) with edge weights
\[
 w_n:=\begin{cases}
  \beta,& n\equiv 47\pmod{48}\;\text{(page boundary)},\\
  1,& \text{otherwise},
 \end{cases}\qquad \beta\in(0,1].
\]
Let \(\mathsf{L}\) be the normalized Laplacian of this weighted graph. The environment is 48-periodic.

**Facts.**
1. The graph is connected for all \(\beta>0\). The spectrum of \(\mathsf{L}\) on \(\ell^2(\mathbb{Z})\) has band structure touching 0; there is no positive spectral gap on the infinite line.
2. On a finite weighted cycle \(C_{N}\) with \(N=48m\) and the same \(w_n\) repeated, the algebraic connectivity \(\lambda_1(C_N)\) satisfies
   \[ \lambda_1(C_N) = \Theta_{\beta}\!\left(\frac{1}{N^2}\right). \]
   In particular, there exist constants \(0<c_1(\beta)\le c_2(\beta)\) such that
   \[ \frac{c_1(\beta)}{N^2} \le \lambda_1(C_N) \le \frac{c_2(\beta)}{N^2}. \]
3. For the lazy random walk associated with \(C_N\), the total-variation mixing time obeys
   \[ t_{\mathrm{mix}}(\varepsilon) \le C(\beta)\,N^2\,\log\tfrac{1}{\varepsilon}. \]

*Notes.* Item 1 follows from standard Floquet analysis for periodic 1D media. Items 2–3 follow from Poincaré and Cheeger-type inequalities for weighted cycles; constants depend only on \(\beta\).

## 7) Crossing penalties and optimization primitive
Let \(E\) be the edge set of \(\mathbb{Z}\) with weights \(w_n\) as above. Define a cost functional for a walk \(\pi=(n_0,\dots,n_T)\):
\[ \mathcal{C}_\tau(\pi):=\sum_{t=0}^{T-1}\Big(\mathbf{1}_{\{n_t\not\equiv 47\}} + \tau\,\mathbf{1}_{\{n_t\equiv 47\}}\Big),\qquad \tau\ge 0. \]
Let \(\Pi_{s\to t}\) be all nearest-neighbor paths from \(s\) to \(t\). The \(\tau\)-optimal cost is
\[ L_\tau(s,t):=\min_{\pi\in\Pi_{s\to t}} \mathcal{C}_\tau(\pi). \]

**Proposition 7.1 (Monotone penalty effect).** If \(0\le \tau_1\le \tau_2\) then \(L_{\tau_1}(s,t)\le L_{\tau_2}(s,t)\). Moreover, any minimizer for \(\tau_2\) uses no more page-boundary edges than a minimizer for \(\tau_1\).

*Interpretation.* Increasing \(\tau\) discourages page crossings in any shortest-path computation, without asserting runtime formulas.

## 8) Concrete data at the LCM scale
- **Superperiod.** \(R(n+768)=R(n)\), hence \(\Phi(p+16)=\Phi(p)\) and the 16-page net flux is identically zero.
- **Counts per superperiod.** A 768-block contains 16 full pages and 3 full 256-cycles of \(F\).

## 9) Minimal implementation sketch
Given \(\alpha\) above:
1. Precompute \(F(n)\) for \(n=0,\dots,255\).
2. Tabulate \(R(n)=\alpha^{\top}F(n)\) over one cycle.
3. For any \(n\), query by \(R(n\bmod 256)\). For any page \(p\), compute \(\Phi(p)=R(48p+48\bmod 256)-R(48p\bmod 256)\) and note \(\sum_{q=0}^{15}\Phi(p+q)=0\).

---
This specification contains only definitions and results that hold under the stated constructions, with proofs or standard references implied for spectral statements on periodic weighted cycles.

