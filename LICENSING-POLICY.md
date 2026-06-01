# LICENSING-POLICY: apex-goldilocks

## Overview
This document defines the licensing and open-core boundaries for the `apex-goldilocks` workspace.

## License
All crates in `projects/apex-goldilocks` are licensed under the **Proprietary Phase Mirror License** unless explicitly stated otherwise in a subdirectory `LICENSE` file.

## Open-Core Boundaries
1.  **Open Core**: 
    *   `apex-goldilocks-core`: The basic field arithmetic and lattice structures.
    *   `goldilocks`: The underlying arithmetic kernel.
2.  **Sealed Pro Tier (Proprietary)**:
    *   `multiplicity-runtime`: The ACE/PETC projection logic and CSC certifiers.
    *   `apex-hologram`: The sigmatics circuit engine and recursive proof generation.
    *   `apex-goldilocks-cli`: The unified delivery tool.

## Governance
- **Zero Drift**: No modifications to the proprietary certifier logic are permitted without explicit CH LABS MOU approval.
- **Redistribution**: Redistribution of any binary containing the Sealed Pro Tier is prohibited without a commercial license.
