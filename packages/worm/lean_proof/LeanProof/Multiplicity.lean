import Lean

namespace Worm

/-!
  # Multiplicity Theory Formalization
  Formalizes core L0 invariants for the Sedona Spine.
-/

/-- The irreducible prime domains (e.g., P_2 Treasury, P_3 Clinical) -/
structure Prime where
  index : Nat
  deriving DecidableEq, Repr

/-- A State is defined by its prime-indexed multiplicity -/
structure State where
  factors : List Prime
  anchors : Nat
  passes  : Nat
  deriving Repr

/-- Multiplicity Thickness: |primes| + |anchors| + |passes| -/
def thickness (s : State) : Nat :=
  s.factors.length + s.anchors + s.passes

/-- A Morphism is a state transition -/
structure Morphism where
  src_p : Prime
  tgt_p : Prime
  authorized_anchors : Nat
  apply : State -> State

/-- Axiom MP-01: Multiplicity Preservation
    The output state's prime factors must be a subset of the input factors. -/
def is_mp01_valid (s : State) (s' : State) : Prop :=
  ∀ p, p ∈ s'.factors → p ∈ s.factors

/-- Contractivity: thickness(s') <= thickness(s) + epsilon -/
def is_contractive (s : State) (s' : State) (epsilon : Nat := 0) : Prop :=
  thickness s' <= thickness s + epsilon

/-- L0 Integrity Theorem: Rule-HO-01
    A transition is valid only if it satisfies MP-01 and Contractivity. -/
theorem rule_ho_01_integrity (s : State) (s' : State)
  (h_valid : is_mp01_valid s s') (h_contractive : is_contractive s s') :
  thickness s' <= thickness s := by
    -- In this minimal model, h_contractive with epsilon=0 already proves the goal.
    exact h_contractive

/-- Multiplicity Composition Lemma:
    Sequential transitions preserve the contractivity bound. -/
lemma multiplicity_composition (s1 s2 s3 : State)
  (h1 : is_contractive s1 s2) (h2 : is_contractive s2 s3) :
  is_contractive s1 s3 := by
    unfold is_contractive at *
    exact Nat.le_trans h2 h1

end Worm
