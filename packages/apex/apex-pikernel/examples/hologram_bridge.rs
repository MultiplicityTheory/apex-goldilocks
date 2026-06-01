use std::collections::HashMap;
use ndarray::{Array1, Array2};
use apex_pikernel::{
    ProjectorFamily, HologramAdapterManaged, HologramAdapterConfig
};

fn main() -> anyhow::Result<()> {
    // 1. Define families for 16D space
    let a = ProjectorFamily::new(
        vec![
            vec![0, 4, 8, 12],
            vec![1, 5, 9, 13],
            vec![2, 6, 10, 14],
            vec![3, 7, 11, 15]
        ],
        "SpectralBands"
    )?;

    let b = ProjectorFamily::new(
        vec![
            vec![0, 1, 2, 3, 4, 5, 6, 7],
            vec![8, 9, 10, 11, 12, 13, 14, 15]
        ],
        "MemoryHalves"
    )?;

    // 2. Build configuration
    let mut alphas = HashMap::new();
    let mut weights = HashMap::new();
    let mut taus = HashMap::new();

    let families = vec![a.clone(), b.clone()];
    let temp_grid = apex_pikernel::PiIndexGrid::new(families.clone())?;
    
    for pi in &temp_grid.pi_ids {
        alphas.insert(pi.clone(), 0.2);
        let indices = temp_grid.indices(pi).unwrap();
        weights.insert(pi.clone(), Array1::ones(indices.len()));
        taus.insert(pi.clone(), 2.0);
    }

    let m = temp_grid.pi_ids.len();
    let mut k = Array2::from_elem((m, m), 0.05);
    for i in 0..m {
        k[[i, i]] = 0.0;
    }

    let config = HologramAdapterConfig {
        families,
        alphas,
        weights,
        taus,
        k,
        use_poseidon: true,
        ledger_path: None,
        mub_threshold: 3.0,
        tau_shrink_factor: 0.9,
    };

    // 3. Initialize adapter
    let mut adapter = HologramAdapterManaged::new(config)?;

    // 4. Run
    let mut x = Array1::from_elem(16, 0.5);
    
    for t in 0..20 {
        let result = adapter.step(x.view())?;
        x = result.x_new;
        
        if result.audit.alarm {
            println!("Step {}: MUB ALARM! D_t={:.4}", t, result.audit.d_t);
        } else {
            println!("Step {}: x_norm={:.4}", t, x.mapv(|a| a.powi(2)).sum().sqrt());
        }
    }

    println!("Final MUB alarms: {}", adapter.mub_alarms);
    println!("Ledger entries: {}", adapter.ledger.get_entries().len());

    Ok(())
}
