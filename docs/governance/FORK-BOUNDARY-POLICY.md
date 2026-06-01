# Fork Boundary Policy

## Purpose
Define migration rules and deprecation path for the legacy `projects/apex` stack.

## Policy
1. **Goldilocks-First**: All new development MUST occur in `projects/apex-goldilocks`.
2. **Quarantine**: Legacy crates in `projects/apex` are quarantined; no new features are permitted.
3. **Migration**: Core logic must be ported to `GoldilocksField` arithmetic; porting is blocked by the "No Float" invariant check.
