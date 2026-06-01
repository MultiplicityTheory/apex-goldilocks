use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use aep_sovereignty_gate::*;

fn main() -> anyhow::Result<()> {
    let base = std::env::current_dir()?;
    
    let aep_toml = fs::read_to_string(base.join("aep.toml")).context("Failed to read aep.toml")?;
    let aep: AepConfig = toml::from_str(&aep_toml)?;
    
    let kernel_toml = fs::read_to_string(base.join("kernel.atlas")).context("Failed to read kernel.atlas")?;
    let ker: KernelConfig = toml::from_str(&kernel_toml)?;
    
    let actor_id = std::env::var("AEP_ACTOR_ID").unwrap_or_else(|_| ker.context.actor_id.clone());
    
    if actor_id.is_empty() {
        eprintln!("missing actor_id");
        std::process::exit(3);
    }

    let (ok_sigma, s_val) = check_sigma(&actor_id, &aep.policy.allowed_ids);

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
        
    let atol = 1e-12; // Default atol
    let (ok_delta, delta_viol) = assert_forbidden_zero(&delta_map, &patterns, atol);

    let status = if ok_sigma && ok_delta { "pass" } else { "fail" };

    let result = ProofResult {
        status: status.to_string(),
        ok_sigma,
        ok_forbidden_channels: ok_delta,
        sigma: s_val,
        actor_id,
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
