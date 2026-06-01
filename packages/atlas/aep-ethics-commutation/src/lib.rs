use serde::{Deserialize, Serialize};
use std::path::Path;
use ndarray::Array2;
use ndarray_npy::ReadNpyExt;
use std::fs::File;
use glob::Pattern;

#[derive(Debug, Deserialize)]
pub struct AepConfig {
    pub claim: Claim,
    pub constraints: Constraints,
    pub evidence: Evidence,
    pub forbidden: Forbidden,
}

#[derive(Debug, Deserialize)]
pub struct Claim {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Constraints {
    pub atol: f64,
}

#[derive(Debug, Deserialize)]
pub struct Evidence {
    pub resonance_pre: Option<FileConfig>,
    pub resonance_post: Option<FileConfig>,
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
    pub operators: Operators,
    pub probes: Option<Probes>,
}

#[derive(Debug, Deserialize)]
pub struct Operators {
    #[serde(rename = "M")]
    pub m: String,
    #[serde(rename = "E_alpha")]
    pub e_alpha: String,
}

#[derive(Debug, Deserialize)]
pub struct Probes {
    pub forbidden_patterns: Vec<String>,
}

pub fn load_operator<P: AsRef<Path>>(path: P) -> anyhow::Result<Array2<f64>> {
    let mut file = File::open(path)?;
    let arr = Array2::<f64>::read_npy(&mut file)?;
    Ok(arr)
}

pub fn check_commutation(m: &Array2<f64>, e: &Array2<f64>, atol: f64) -> (bool, f64) {
    let me = m.dot(e);
    let em = e.dot(m);
    let diff = &me - &em;
    
    // Frobenius norm
    let norm = diff.mapv(|x| x * x).sum().sqrt();
    (norm <= atol, norm)
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
    pub ok_commutation: bool,
    pub ok_forbidden_channels: bool,
    pub comm_norm: f64,
    pub atol: f64,
    pub delta_violations: Vec<String>,
    pub verified: bool,
}
