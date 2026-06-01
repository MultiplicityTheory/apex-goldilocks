use std::collections::HashMap;
use ndarray::{Array1, Array2};
use apex_pikernel::{
    ProjectorFamily, PiIndexGrid, PiKernel,
    DefaultLedger
};

fn main() -> anyhow::Result<()> {
    // 1. Define projector families (orthogonal partitions)
    let a = ProjectorFamily::new(
        vec![vec![0, 2, 4, 6], vec![1, 3, 5, 7]],
        "FunctionType"
    )?;
    let b = ProjectorFamily::new(
        vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7]],
        "MemoryRegion"
    )?;

    // 2. Build PI-atom grid
    let grid = PiIndexGrid::new(vec![a, b])?;
    let piids = &grid.pi_ids;

    // 3. Configure dynamics
    let mut alphas = HashMap::new();
    let mut weights = HashMap::new();
    let mut taus = HashMap::new();

    for pi in piids {
        alphas.insert(pi.clone(), 0.25);
        weights.insert(pi.clone(), Array1::ones(8)); // Note: Support size varies, but let's use 8 for simplicity or actual size
        // Correctly, weights should match indices size
        let indices = grid.indices(pi).unwrap();
        weights.insert(pi.clone(), Array1::ones(indices.len()));
        taus.insert(pi.clone(), 1.5);
    }

    // Coupling matrix
    let m = piids.len();
    let mut k = Array2::from_elem((m, m), 0.05);
    for i in 0..m {
        k[[i, i]] = 0.0;
    }

    // 4. Initialize kernel
    let ledger = Box::new(DefaultLedger::new(None));
    let mut kernel = PiKernel::new(
        &grid,
        alphas,
        weights,
        taus,
        k,
        None,
        Some(ledger),
    )?;

    // 5. Run
    let mut x = Array1::from_elem(8, 1.0); // Simple initial state
    
    for t in 0..10 {
        let result = kernel.step(x.view())?;
        x = result.x_new;
        
        println!("Step {}: SlopeUB={:.4}, GapLB={:.4}, Touched={}", 
            t, result.slope_ub, result.gap_lb, result.touched.len());
        
        assert!(result.gap_lb > 0.0, "Contraction violated!");
    }

    Ok(())
}
