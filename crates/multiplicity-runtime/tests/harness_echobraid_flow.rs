use apex_goldilocks_core::GoldVector;
use goldilocks::{GoldilocksField, PrimeMask};
use multiplicity_runtime::{MultiplicityRuntime, CRMFConfig};
use multiplicity_runtime::harness::{NeuralHarness, EchoBraidState, HarnessAdapter};

#[test]
fn test_harness_echobraid_valid_flow() {
    let config = CRMFConfig {
        domain_tag: 0x42,
        prime_index: 256,
        prime_mask: PrimeMask(0xFFFFFFFFFFFFFFFF),
        signature: None,
    };
    let mut runtime = MultiplicityRuntime::new(config, 100);
    let harness = NeuralHarness::new(10);
    
    let initial_theta = GoldVector::new(vec![GoldilocksField::new(1); 10]);
    let current_state = EchoBraidState {
        theta: initial_theta,
        iteration: 0,
    };

    let mut adapter = HarnessAdapter::new(&mut runtime, harness);
    
    // Propose an adaptation
    let proposal = adapter.harness.propose_adaptation(&current_state);
    
    // Commit the proposal
    let result = adapter.commit_proposal(proposal);
    
    assert!(result.is_ok(), "Valid EchoBraid proposal should be committed and certified");
    assert_eq!(adapter.runtime.ace_budget, 99);
}

#[test]
fn test_harness_multiplicity_veto() {
    let config = CRMFConfig {
        domain_tag: 0x42,
        prime_index: 5, // Small capacity
        prime_mask: PrimeMask(0xFFFFFFFFFFFFFFFF),
        signature: None,
    };
    let mut runtime = MultiplicityRuntime::new(config, 100);
    let harness = NeuralHarness::new(10);
    
    // Create a state that exceeds the multiplicity capacity
    let initial_theta = GoldVector::new(vec![GoldilocksField::new(1); 10]);
    let current_state = EchoBraidState {
        theta: initial_theta,
        iteration: 0,
    };

    let mut adapter = HarnessAdapter::new(&mut runtime, harness);
    
    let proposal = adapter.harness.propose_adaptation(&current_state);
    let result = adapter.commit_proposal(proposal);
    
    assert!(result.is_err(), "Proposal exceeding multiplicity capacity should be vetoed");
}

#[test]
fn test_harness_budget_veto() {
    let config = CRMFConfig {
        domain_tag: 0x42,
        prime_index: 256,
        prime_mask: PrimeMask(0xFFFFFFFFFFFFFFFF),
        signature: None,
    };
    let mut runtime = MultiplicityRuntime::new(config, 0); // No budget
    let harness = NeuralHarness::new(10);
    
    let initial_theta = GoldVector::new(vec![GoldilocksField::new(1); 10]);
    let current_state = EchoBraidState {
        theta: initial_theta,
        iteration: 0,
    };

    let mut adapter = HarnessAdapter::new(&mut runtime, harness);
    
    let proposal = adapter.harness.propose_adaptation(&current_state);
    let result = adapter.commit_proposal(proposal);
    
    assert!(result.is_err(), "Proposal with exhausted runtime budget should be vetoed");
}

#[test]
fn test_harness_prime_gate_veto() {
    let config = CRMFConfig {
        domain_tag: 0x42,
        prime_index: 3, // bit index 3
        prime_mask: PrimeMask(0b0011), // only bits 0 and 1 are active (bit 3 is inactive)
        signature: None,
    };
    let mut runtime = MultiplicityRuntime::new(config, 100);
    let harness = NeuralHarness::new(10);
    
    let initial_theta = GoldVector::new(vec![GoldilocksField::new(1); 2]);
    let current_state = EchoBraidState {
        theta: initial_theta,
        iteration: 0,
    };

    let mut adapter = HarnessAdapter::new(&mut runtime, harness);
    
    let proposal = adapter.harness.propose_adaptation(&current_state);
    let result = adapter.commit_proposal(proposal);
    
    assert!(result.is_err(), "Proposal on inactive prime gate should be vetoed");
}

#[test]
fn test_harness_lattice_certified_adaptation() {
    use apex_goldilocks_core::boundary_lattice::LatticeElement;

    let config = CRMFConfig {
        domain_tag: 0x42,
        prime_index: 256,
        prime_mask: PrimeMask(0xFFFFFFFFFFFFFFFF),
        signature: None,
    };
    let mut runtime = MultiplicityRuntime::new(config, 100);
    let harness = NeuralHarness::new(10);
    
    // Create an initial state derived from a lattice element
    let element = LatticeElement::new(0, 0);
    let initial_theta = GoldVector::new(vec![GoldilocksField::new(element.to_index() as u64); 10]);
    
    let current_state = EchoBraidState {
        theta: initial_theta,
        iteration: 0,
    };

    let mut adapter = HarnessAdapter::new(&mut runtime, harness);
    let proposal = adapter.harness.propose_adaptation(&current_state);
    
    // The adaptation should still respect the lattice's classical horizon
    // in its interpretive step, but the runtime ensures the final seal.
    let result = adapter.commit_proposal(proposal);
    assert!(result.is_ok());
    
    // Verify the lattice invariant (index < 12288) is still accessible
    assert!(adapter.runtime.certify_lattice(element));
}
