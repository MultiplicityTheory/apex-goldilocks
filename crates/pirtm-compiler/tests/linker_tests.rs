use pirtm_compiler::{verify_module_manifest, DynamicOperator, FixedPoint, SCALE_BASE, Sig};
use std::collections::HashMap;

fn create_mock_operator(norm: i64) -> DynamicOperator {
    DynamicOperator {
        signature: Sig::new(), // Placeholder
        norm: FixedPoint(norm),
    }
}

#[test]
fn test_stability_violation() {
    let ops = vec![
        create_mock_operator(600_000), // 0.6
        create_mock_operator(500_000), // 0.5
    ];
    let attrs = pirtm_compiler::ModuleAttributes {
        spectral_radius: FixedPoint(100_000),
        epsilon: FixedPoint(100_000),
        prime_index: 7,
    };
    
    // Sum = 1.1 + 0.1 = 1.2 > 1.0 -> Should fail
    let result = verify_module_manifest(&ops, attrs);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Stability constraint violated"));
}

#[test]
fn test_spectral_radius_violation() {
    let ops = vec![create_mock_operator(100_000)]; // 0.1
    let attrs = pirtm_compiler::ModuleAttributes {
        spectral_radius: FixedPoint(950_000), // r(Λ) = 0.95
        epsilon: FixedPoint(100_000),        // ε = 0.1 -> 1-ε = 0.9
        prime_index: 7,
    };
    
    // r(Λ) = 0.95 >= 0.9 -> Should fail
    let result = verify_module_manifest(&ops, attrs);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Spectral radius violation"));
}

#[test]
fn test_successful_verification() {
    let ops = vec![create_mock_operator(300_000)]; // 0.3
    let attrs = pirtm_compiler::ModuleAttributes {
        spectral_radius: FixedPoint(500_000), // 0.5
        epsilon: FixedPoint(100_000),        // 0.1 -> 1-ε = 0.9
        prime_index: 7,
    };
    
    // Stability: 0.3 + 0.1 = 0.4 < 1.0 -> OK
    // Spectral: 0.5 < 0.9 -> OK
    let result = verify_module_manifest(&ops, attrs);
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Lean 4 Metatheorem"));
}
