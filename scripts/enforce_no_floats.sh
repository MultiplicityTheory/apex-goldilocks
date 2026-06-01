#!/bin/bash
# L0 Invariant: Zero Float Policy
if grep -rnE "f32|f64" crates/ | grep -v "build.rs" | grep -v ".json" | grep -q ".rs"; then
    echo "CRITICAL: Float leakage detected in L0 core."
    exit 1
fi
echo "✓ L0 Float Invariant Satisfied."
