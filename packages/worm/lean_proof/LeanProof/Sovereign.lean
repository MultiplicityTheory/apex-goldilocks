import Lean
import LeanProof.Multiplicity

namespace Worm

/-!
  # Sovereign Boundary Formalization
  Formalizes the RegHom registry and boundary enforcement.
-/

/-- The Registry of Lawful Homomorphisms (RegHom) -/
structure RegHom where
  morphisms : List (Prime × Prime)

/-- A transition is registered if its (src, tgt) pair is in RegHom -/
def is_registered (reg : RegHom) (p_src p_tgt : Prime) : Prop :=
  (p_src, p_tgt) ∈ reg.morphisms

/-- The Terminal Rejection State (bot_R) -/
inductive Outcome where
  | admitted (s : State)
  | bot_R (error : String)
  deriving Repr

/-- The L0 Prime Gate Interrogator -/
def interrogate (reg : RegHom) (phi : Morphism) (s : State) : Outcome :=
  if is_registered reg phi.src_p phi.tgt_p then
    let s' := phi.apply s
    if is_mp01_valid s s' ∧ is_contractive s s' then
      Outcome.admitted s'
    else
      Outcome.bot_R "Constitutional Violation"
  else
    Outcome.bot_R "Sovereign Boundary Violation"

/-- Theorem: No Leakage
    An unregistered transition always results in bot_R. -/
theorem sovereign_boundary_enforcement (reg : RegHom) (phi : Morphism) (s : State)
  (h_unregistered : ¬ is_registered reg phi.src_p phi.tgt_p) :
  interrogate reg phi s = Outcome.bot_R "Sovereign Boundary Violation" := by
    unfold interrogate
    -- Use decidability of is_registered
    by_cases h : is_registered reg phi.src_p phi.tgt_p
    · contradiction
    · simp [h]

end Worm
