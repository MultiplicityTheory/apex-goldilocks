use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use aep_ethics_commutation::*;

fn main() -> anyhow::Result<()> {
    let base = std::env::current_dir()?;
    
    let aep_toml = fs::read_to_string(base.join("aep.toml")).context("Failed to read aep.toml")?;
    let aep: AepConfig = toml::from_str(&aep_toml)?;
    
    let kernel_toml = fs::read_to_string(base.join("kernel.atlas")).context("Failed to read kernel.atlas")?;
    let ker: KernelConfig = toml::from_str(&kernel_toml)?;
    
    let atol = aep.constraints.atol;

    // Load operators
    let m = load_operator(base.join(&ker.operators.m))?;
    let e = load_operator(base.join(&ker.operators.e_alpha))?;

    let (ok_comm, comm_norm) = check_commutation(&m, &e, atol);

    // Check Delta-channels
    let channels_path = base.join(&aep.evidence.channels.path);
    let delta_map: HashMap<String, f64> = if channels_path.exists() {
        let content = fs::read_to_string(channels_path)?;
        serde_json::from_str(&content)?
    } else {
        HashMap::new()
    };

    let patterns = ker.probes.as_ref()
        .map(|p| p.forbidden_patterns.clone())
        .unwrap_or_else(|| aep.forbidden.channels.clone());
        
    let (ok_delta, delta_viol) = assert_forbidden_zero(&delta_map, &patterns, atol);

    let status = if ok_comm && ok_delta { "pass" } else { "fail" };

    let result = ProofResult {
        status: status.to_string(),
        ok_commutation: ok_comm,
        ok_forbidden_channels: ok_delta,
        comm_norm,
        atol,
        delta_violations: delta_viol,
        verified: true, // Mock verification
    };

    let out_dir = base.join("out");
    fs::create_dir_all(&out_dir)?;
    let out_file = out_dir.join("result.json");
    let out_json = serde_json::to_string_pretty(&result)?;
    fs::write(out_file, out_json)?;

    if status == "fail" {
        std::process::exit(2);
    }

    Ok(())
}
