# Modular Structural Arithmetic (MSA): Clean Formulation

## 0. Notation and Basic Objects
- Let $\mathbb Z$ denote the integers. For a prime $p$, write $\mathbb Z/p\mathbb Z$ for the residue ring and $(\mathbb Z/p\mathbb Z)^\times$ for its multiplicative group.
- Reduction mod $p$ is $\rho_p:\mathbb Z\to\mathbb Z/p\mathbb Z$, $\rho_p(x)\equiv x\pmod p$.
- The doubling map is $\delta:\mathbb Z\to\mathbb Z$, $\delta(x)=2x$.
- Distinguished constants: $\gamma=24$, $\mu=48$, $\varepsilon=96$.

## 1. Induced Modular Dynamics
For each prime $p$, define $\delta_p:\mathbb Z/p\mathbb Z\to\mathbb Z/p\mathbb Z$ by $\delta_p([x])=[2x]$.

### Theorem 1 (Doubling commutes with reduction)
For every prime $p$ and every $x\in\mathbb Z$,
\[
\rho_p(\delta(x))=\delta_p(\rho_p(x)).
\]
**Proof.** $\rho_p(2x)=[2][x]=\delta_p([x])$ by ring homomorphism properties. $\square$

We abbreviate this global commutation property as **DP($p$)**.

### Definition 1 (Order of doubling mod $p$)
For an odd prime $p$, let $r=\mathrm{ord}_p(2)$ be the least positive integer with $2^r\equiv1\pmod p$.

### Theorem 2 (Resonance length)
For any odd prime $p$ with $r=\mathrm{ord}_p(2)$ and any $x\in\mathbb Z$,
\[
\delta^r(x)\equiv x\pmod p.
\]
In particular, every orbit of $\delta_p$ in $\mathbb Z/p\mathbb Z$ has period dividing $r$; orbits inside $(\mathbb Z/p\mathbb Z)^\times$ are cycles whose lengths divide $r$.

**Proof.** $\delta^r(x)=2^r x\equiv x\pmod p$. $\square$

### Lemma 1 (Distinctness of seeds)
If $p\ge5$ is prime, then $[\gamma],[\mu],[\varepsilon]$ are nonzero and pairwise distinct in $\mathbb Z/p\mathbb Z$.

**Proof.** $p\nmid 24,48,96$ and $\mu-\gamma=24$, $\varepsilon-\mu=48$ are not divisible by any prime $\ge5$. $\square$

## 2. Role System and Modularity
Introduce three unary predicates on nonzero integers
\[
\mathrm{Gen}(x),\quad \mathrm{Med}(x),\quad \mathrm{Man}(x),\qquad x\in\mathbb Z\setminus\{0\}.
\]

### Axiom R1 (Partition)
For every $x\ne0$, exactly one of $\mathrm{Gen}(x)$, $\mathrm{Med}(x)$, $\mathrm{Man}(x)$ holds.

### Axiom R2 (Seed labels)
$\mathrm{Gen}(\gamma)$, $\mathrm{Med}(\mu)$, and $\mathrm{Man}(\varepsilon)$ hold.

### Axiom R3 (Congruence compatibility on units)
For every odd prime $p$, if $x\equiv y\pmod p$ and $xy\not\equiv0\pmod p$, then
\[
\mathrm{Gen}(x)\Leftrightarrow\mathrm{Gen}(y),\quad
\mathrm{Med}(x)\Leftrightarrow\mathrm{Med}(y),\quad
\mathrm{Man}(x)\Leftrightarrow\mathrm{Man}(y).
\]
Equivalently, the role map factors through $\rho_p$ on $\mathbb Z\setminus p\mathbb Z$.

### Definition 2 (Structural property SP($p$))
For a prime $p$, **SP($p$)** holds if:
1. $p$ is odd;
2. $[\gamma],[\mu],[\varepsilon]$ are nonzero and pairwise distinct in $\mathbb Z/p\mathbb Z$ (automatic for $p\ge5$ by Lemma 1);
3. Axiom R3 applies at $p$.

Under SP($p$), roles induce well-defined predicates on $(\mathbb Z/p\mathbb Z)^\times$:
\[
\mathrm{Gen}_p([x])\iff \mathrm{Gen}(x),\quad
\mathrm{Med}_p([x])\iff \mathrm{Med}(x),\quad
\mathrm{Man}_p([x])\iff \mathrm{Man}(x).
\]

### Theorem 3 (Role partition mod $p$)
If SP($p$) holds, then $\{\mathrm{Gen}_p,\mathrm{Med}_p,\mathrm{Man}_p\}$ is a disjoint partition of $(\mathbb Z/p\mathbb Z)^\times$.

**Proof.** By R1 on representatives and R3 for well-definedness. $\square$

## 3. Dynamics and Roles
No role-dynamical constraints are assumed by default. When needed, one may adopt the following optional compatibility, which then propagates labels along doubling orbits.

### Optional Axiom D1 (Doubling invariance of roles)
For all $x\ne0$, $\mathrm{Gen}(x)\Leftrightarrow\mathrm{Gen}(2x)$, and similarly for $\mathrm{Med}$ and $\mathrm{Man}$.

Under D1 and SP($p$), roles are constant on $\delta_p$-orbits in $(\mathbb Z/p\mathbb Z)^\times$.

## 4. Consolidated Statements
- **DP($p$)** holds for every prime $p$ (Theorem 1).
- For every odd prime $p$, the resonance length $r=\mathrm{ord}_p(2)$ controls all $\delta_p$-orbit periods (Theorem 2).
- For every prime $p\ge5$, seeds $[\gamma],[\mu],[\varepsilon]$ are distinct and nonzero (Lemma 1). Hence SP($p$) holds once R3 is adopted.
- If SP($p$) holds, then roles descend to $(\mathbb Z/p\mathbb Z)^\times$ and form a partition (Theorem 3).
- If D1 is also assumed, each $\delta_p$-orbit sits entirely inside one of the three role classes.

## 5. Minimal Data to Instantiate MSA
1. Base set and operator: $(\mathbb Z,\delta)$ with $\delta(x)=2x)$.
2. Distinguished constants: $\gamma=24,\ \mu=48,\ \varepsilon=96$.
3. Role predicates $\mathrm{Gen},\mathrm{Med},\mathrm{Man}$ satisfying R1–R2, optionally D1.
4. For modular analysis at prime $p$: invoke DP($p$) always; invoke SP($p$) to transport roles to $(\mathbb Z/p\mathbb Z)^\times$; compute $r=\mathrm{ord}_p(2)$ to describe $\delta_p$-dynamics.

---
**Remark.** The framework separates arithmetic facts (DP($p$), resonance via $\mathrm{ord}_p(2)$) from structural labeling (R1–R3, optional D1). This allows safe reuse of the arithmetic layer independent of any chosen role semantics.

