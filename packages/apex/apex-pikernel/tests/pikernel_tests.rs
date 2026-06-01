use ndarray::{Array1, Array2};
use apex_pikernel::*;
use std::collections::HashMap;

#[test]
fn test_projector_family_disjoint() {
    let a = ProjectorFamily::new(
        vec![vec![0, 1], vec![2, 3]],
        "A"
    ).unwrap();
    assert_eq!(a.num_blocks, 2);
    assert_eq!(a.dim, 4);

    let b = ProjectorFamily::new(
        vec![vec![0, 2], vec![0, 3]],
        "B"
    );
    assert!(b.is_err());
}

#[test]
fn test_pi_index_grid() {
    let a = ProjectorFamily::new(
        vec![vec![0, 2, 4, 6], vec![1, 3, 5, 7]],
        "FunctionType"
    ).unwrap();
    let b = ProjectorFamily::new(
        vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7]],
        "MemoryRegion"
    ).unwrap();

    let grid = PiIndexGrid::new(vec![a, b]).unwrap();
    assert_eq!(grid.pi_ids.len(), 4);
    
    // Test intersection (0,0) -> {0, 2}
    let pi00 = vec![0, 0];
    let indices00 = grid.indices(&pi00).unwrap();
    assert_eq!(indices00, &[0, 2]);

    // Test recomposition
    let x = Array1::from_elem(8, 1.0);
    let err = grid.recomposition_error(x.view());
    assert!(err < 1e-12);
}

#[test]
fn test_weighted_l1_projection() {
    let v = Array1::from_vec(vec![1.0, 2.0, 3.0]);
    let w = Array1::from_vec(vec![1.0, 1.0, 1.0]);
    let tau = 3.0;

    let (x, lam) = project_weighted_l1_ball(v.view(), w.view(), tau, 1e-12, 10000);
    
    let sum_abs: f64 = x.iter().map(|&xi| xi.abs()).sum();
    assert!((sum_abs - tau).abs() < 1e-10);
    assert!(lam > 0.0);
}

#[test]
fn test_contraction_certificate() {
    let alphas = Array1::from_vec(vec![0.2, 0.2]);
    let k = Array2::from_shape_vec((2, 2), vec![0.0, 0.1, 0.1, 0.0]).unwrap();
    
    let cert = verify_contraction(alphas.view(), k.view());
    assert!(cert.is_contractive);
    assert!(cert.slope_ub < 1.0);
}

#[test]
fn test_ledger_sha256() {
    let mut ledger = DefaultLedger::new(None);
    let mut record = HashMap::new();
    record.insert("step".to_string(), 0.into());
    record.insert("pi".to_string(), vec![0, 0].into());
    record.insert("alpha".to_string(), 0.1.into());
    record.insert("tau".to_string(), 1.0.into());
    record.insert("lambda_soft".to_string(), 0.0.into());
    record.insert("l1_weight_sum".to_string(), 0.5.into());
    record.insert("change_norm".to_string(), 0.1.into());
    
    let digest = ledger.append(record).unwrap();
    assert_eq!(digest.len(), 64);
    assert_eq!(ledger.get_entries().len(), 1);
}

#[test]
fn test_mub_audit() {
    let x = Array1::from_vec(vec![1.0, 0.0, 0.0, 0.0]); // Concentrated in standard -> Uniform in MUB
    let result = mub_drift_audit(x.view(), 1.0);
    assert!(!result.alarm);
    
    let y = Array1::from_vec(vec![0.5, 0.5, 0.5, 0.5]); // Uniform in standard -> Concentrated in MUB
    let result_y = mub_drift_audit(y.view(), 1.0);
    assert!(result_y.alarm);
}

#[test]
fn test_poseidon_educational() {
    let inputs = vec![num_bigint::BigUint::from(1u32), num_bigint::BigUint::from(2u32)];
    let hash = poseidon_hash(inputs);
    assert!(hash > num_bigint::BigUint::from(0u32));
    
    let hash_bytes = poseidon_hash_bytes(b"hello world");
    assert!(hash_bytes > num_bigint::BigUint::from(0u32));
}

#[test]
fn test_rns_crt() {
    let m1 = num_bigint::BigInt::from(3u32);
    let m2 = num_bigint::BigInt::from(5u32);
    let moduli = vec![m1, m2];
    
    let x = num_bigint::BigInt::from(7u32);
    let residues = rns_encode(&x, &moduli);
    let decoded = rns_decode(&residues, &moduli);
    
    assert_eq!(x, decoded);
}
