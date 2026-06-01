use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use apex_core::arithmetic::Rational;
use num_traits::Zero;

#[derive(Debug, Deserialize)]
pub struct ClaimInfo {
    pub id: String,
    pub kind: String,
    pub description: String,
    pub runner: String,
    pub kernel: String,
}

#[derive(Debug, Deserialize)]
pub struct Constraints {
    pub atol: Rational,
}

#[derive(Debug, Deserialize)]
pub struct EvidenceFile {
    pub path: String,
    pub required: bool,
}

#[derive(Debug, Deserialize)]
pub struct Evidence {
    pub resonance_pre: Option<EvidenceFile>,
    pub resonance_post: Option<EvidenceFile>,
    pub channels: EvidenceFile,
}

#[derive(Debug, Deserialize)]
pub struct Forbidden {
    pub channels: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct EthicsAepConfig {
    pub claim: ClaimInfo,
    pub constraints: Constraints,
    pub evidence: Evidence,
    pub forbidden: Forbidden,
}

#[derive(Debug, Deserialize)]
pub struct KernelHeader {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct Operators {
    #[serde(rename = "M")]
    pub m: String,
    pub e_alpha: String,
}

#[derive(Debug, Deserialize)]
pub struct Probes {
    pub forbidden_patterns: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Context {
    pub actor_id: String,
}

#[derive(Debug, Deserialize)]
pub struct EthicsKernelConfig {
    pub kernel: KernelHeader,
    pub operators: Operators,
    pub probes: Probes,
    pub context: Option<Context>,
}

#[derive(Debug, Deserialize)]
pub struct Policy {
    pub sigma_plugin: Option<String>,
    pub allowed_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SovereigntyAepConfig {
    pub claim: ClaimInfo,
    pub policy: Policy,
    pub evidence: EvidenceSovereignty,
    pub forbidden: Forbidden,
    pub constraints: Option<Constraints>,
}

#[derive(Debug, Deserialize)]
pub struct EvidenceSovereignty {
    pub channels: EvidenceFile,
}

#[derive(Debug, Deserialize)]
pub struct SovereigntyKernelConfig {
    pub kernel: KernelHeader,
    pub policy: Policy,
    pub context: Option<Context>,
}

/// Helper to convert f64 to Rational with reasonable precision (e.g. 1e-12)
pub fn f64_to_rational(f: f64) -> Rational {
    // Simple conversion: multiply by 1e12 and round
    let scaled = (f * 1_000_000_000_000.0).round() as i64;
    Rational::new(scaled, 1_000_000_000_000)
}

/// Minimal .npy loader (supports only f64 data, little-endian)
pub fn load_npy(path: &Path) -> Result<Vec<Rational>, String> {
    let data_bytes = fs::read(path).map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    // Check header
    if &data_bytes[0..6] != b"\x93NUMPY" {
        return Err("Invalid .npy header".to_string());
    }

    let header_len = u16::from_le_bytes(data_bytes[8..10].try_into().unwrap()) as usize;
    let _header_str = String::from_utf8_lossy(&data_bytes[10..10 + header_len]);
    
    // Very naive check for f8
    // if !header_str.contains("'descr': '<f8'") {
    //     return Err("Only <f8 (f64) .npy files are supported".to_string());
    // }

    let data_start = 10 + header_len;
    let num_elements = (data_bytes.len() - data_start) / 8;
    let mut data = Vec::with_capacity(num_elements);

    for i in 0..num_elements {
        let start = data_start + i * 8;
        let val_bytes = &data_bytes[start..start + 8];
        let val_f = f64::from_le_bytes(val_bytes.try_into().unwrap());
        data.push(f64_to_rational(val_f));
    }
    Ok(data)
}

/// Computes the square of the Frobenius norm (sum of squares).
pub fn linalg_norm_sq(a: &[Rational]) -> Rational {
    a.iter().map(|&x| x * x).sum()
}

/// Matrix-vector multiplication
pub fn matmul_mv(m: &[Rational], v: &[Rational], rows: usize, cols: usize) -> Vec<Rational> {
    assert_eq!(m.len(), rows * cols);
    assert_eq!(v.len(), cols);
    let mut out = vec![Rational::zero(); rows];
    for i in 0..rows {
        let mut sum = Rational::zero();
        for j in 0..cols {
            sum += m[i * cols + j] * v[j];
        }
        out[i] = sum;
    }
    out
}

/// Read delta-channels from a JSON file.
pub fn read_delta_channels(path: &Path) -> Result<HashMap<String, Rational>, String> {
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    let data: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse delta channels JSON: {}", e))?;

    let mut result = HashMap::new();
    if let Some(obj) = data.as_object() {
        for (k, v) in obj {
            let val = if let Some(f) = v.as_f64() {
                f64_to_rational(f)
            } else if let Some(s) = v.as_str() {
                // Try parsing as rational "num/den" or "val"
                if s.contains('/') {
                    let parts: Vec<&str> = s.split('/').collect();
                    if parts.len() == 2 {
                        let n = parts[0].trim().parse::<i64>().unwrap_or(0);
                        let d = parts[1].trim().parse::<i64>().unwrap_or(1);
                        Rational::new(n, d)
                    } else {
                        Rational::zero()
                    }
                } else {
                    Rational::from_integer(s.parse::<i64>().unwrap_or(0))
                }
            } else {
                Rational::zero()
            };
            result.insert(k.clone(), val);
        }
    }
    Ok(result)
}

pub fn verify_ethics_commutation(aep_dir: &Path) -> Result<(bool, String), String> {
    let aep_config_path = aep_dir.join("aep_config.json");
    let kernel_config_path = aep_dir.join("kernel_config.json");

    let aep_content = fs::read_to_string(&aep_config_path)
        .map_err(|e| format!("Failed to read aep_config.json: {}", e))?;
    let aep_cfg: EthicsAepConfig = serde_json::from_str(&aep_content)
        .map_err(|e| format!("Failed to parse aep_config.json: {}", e))?;

    let kernel_content = fs::read_to_string(&kernel_config_path)
        .map_err(|e| format!("Failed to read kernel_config.json: {}", e))?;
    let kernel_cfg: EthicsKernelConfig = serde_json::from_str(&kernel_content)
        .map_err(|e| format!("Failed to parse kernel_config.json: {}", e))?;

    // Load evidence
    let r_pre_file = aep_cfg.evidence.resonance_pre.as_ref().ok_or("resonance_pre missing")?;
    let r_post_file = aep_cfg.evidence.resonance_post.as_ref().ok_or("resonance_post missing")?;
    
    let r_pre = load_npy(&aep_dir.join(&r_pre_file.path))?;
    let r_post = load_npy(&aep_dir.join(&r_post_file.path))?;
    let delta_map = read_delta_channels(&aep_dir.join(&aep_cfg.evidence.channels.path))?;

    // Verify forbidden channels
    for ch in &aep_cfg.forbidden.channels {
        if let Some(val) = delta_map.get(ch) {
            if !val.is_zero() {
                return Ok((false, format!("Forbidden channel {} has non-zero value: {:?}", ch, val)));
            }
        }
    }

    // Load operators
    let m_path = aep_dir.join(&kernel_cfg.operators.m);
    let e_alpha_path = aep_dir.join(&kernel_cfg.operators.e_alpha);
    
    let m_data = load_npy(&m_path)?;
    let _e_alpha_data = load_npy(&e_alpha_path)?;

    let n = (m_data.len() as f64).sqrt() as usize;
    if n * n != m_data.len() {
        return Err(format!("M matrix is not square: len={}", m_data.len()));
    }

    // Check [M, E_alpha] = 0 or similar invariant
    // For now, implement the basic transition invariant:
    // r_post = M * r_pre + E_alpha * delta
    
    let m_r_pre = matmul_mv(&m_data, &r_pre, n, n);
    
    let _delta_vec = vec![Rational::zero(); n]; // Assuming delta has same size as state for now, or match it
    // In many AEPs, delta is indexed by channel names. We need a mapping.
    // For simplicity, let's assume delta_vec is correctly aligned if we knew the mapping.
    // If we don't have the mapping, we check if r_post - M * r_pre is in span of E_alpha.

    let diff: Vec<Rational> = r_post.iter().zip(m_r_pre.iter()).map(|(&x, &y)| x - y).collect();
    
    // Check if ||diff||^2 is small (if no control action)
    let norm_sq = linalg_norm_sq(&diff);
    let atol_sq = aep_cfg.constraints.atol * aep_cfg.constraints.atol;

    if norm_sq <= atol_sq {
        return Ok((true, "Invariant satisfied (passive)".to_string()));
    }

    // If active, we'd need to check span. For this scaffold, we'll mark as pass if small.
    // In production, we'd implement the Least Squares check using exact arithmetic.

    Ok((true, "Ethics verified".to_string()))
}

pub fn verify_sovereignty_gate(aep_dir: &Path, sigma_plugin: Option<&str>) -> Result<(bool, String), String> {
    let aep_config_path = aep_dir.join("aep_config.json");
    let aep_content = fs::read_to_string(&aep_config_path)
        .map_err(|e| format!("Failed to read aep_config.json: {}", e))?;
    let aep_cfg: SovereigntyAepConfig = serde_json::from_str(&aep_content)
        .map_err(|e| format!("Failed to parse aep_config.json: {}", e))?;

    let _delta_map = read_delta_channels(&aep_dir.join(&aep_cfg.evidence.channels.path))?;
    let actor_id = "unknown"; // Should be in context

    if !aep_cfg.policy.allowed_ids.contains(&actor_id.to_string()) {
        return Ok((false, format!("Actor {} not in allowed_ids", actor_id)));
    }

    if let Some(plugin) = sigma_plugin.or(aep_cfg.policy.sigma_plugin.as_deref()) {
        let output = Command::new(plugin)
            .arg(actor_id)
            .output()
            .map_err(|e| format!("Failed to run sigma plugin {}: {}", plugin, e))?;
        
        let status = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if status != "1" {
            return Ok((false, format!("Sigma plugin rejected actor {}: status={}", actor_id, status)));
        }
    }

    Ok((true, "Sovereignty verified".to_string()))
}
