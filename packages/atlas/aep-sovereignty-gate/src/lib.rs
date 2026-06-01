use serde::{Deserialize, Serialize};
use glob::Pattern;

#[derive(Debug, Deserialize)]
pub struct AepConfig {
    pub claim: Claim,
    pub policy: Policy,
    pub evidence: Evidence,
    pub forbidden: Forbidden,
}

#[derive(Debug, Deserialize)]
pub struct Claim {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Policy {
    pub sigma_plugin: Option<String>,
    pub allowed_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Evidence {
    pub channels: FileConfig,
}

#[derive(Debug, Deserialize)]
pub struct FileConfig {
    pub path: String,
    pub required: bool,
}

#[derive(Debug, Deserialize)]
pub struct Forbidden {
    pub channels: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct KernelConfig {
    pub context: ContextConfig,
    pub probes: Option<Probes>,
}

#[derive(Debug, Deserialize)]
pub struct ContextConfig {
    pub actor_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Probes {
    pub forbidden_patterns: Vec<String>,
}

pub fn check_sigma(actor_id: &str, allowed_ids: &[String]) -> (bool, i32) {
    let ok = allowed_ids.iter().any(|id| id == actor_id);
    (ok, if ok { 1 } else { 0 })
}

pub fn assert_forbidden_zero(
    delta_map: &std::collections::HashMap<String, f64>,
    patterns: &[String],
    atol: f64,
) -> (bool, Vec<String>) {
    let mut violations = Vec::new();
    for (name, val) in delta_map {
        for pat_str in patterns {
            if let Ok(pat) = Pattern::new(pat_str) {
                if pat.matches(name) {
                    if val.abs() > atol {
                        violations.push(format!("{}:{}", name, val));
                    }
                }
            }
        }
    }
    (violations.is_empty(), violations)
}

#[derive(Debug, Serialize)]
pub struct ProofResult {
    pub status: String,
    pub ok_sigma: bool,
    pub ok_forbidden_channels: bool,
    pub sigma: i32,
    pub actor_id: String,
    pub delta_violations: Vec<String>,
    pub verified: bool,
}
