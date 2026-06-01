use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use sha2::{Digest as ShaDigest, Sha256};
use crate::aep::ace::{Budgets, ProjResult, ace_accept, project_dual_int};

pub type GradFn = dyn Fn(&[i64], i32) -> Vec<i64>;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpascConfig {
    pub q: i64,
    pub eta_Q: i64,
    pub budgets: Budgets,
    pub b_norms_Q: Vec<i64>,
    pub ablate_mask: Option<Vec<i64>>,
    pub seed: i64,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpascState {
    pub step: i32,
    pub w_Q: Vec<i64>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpascStepLog {
    pub step: i32,
    pub w_before_Q: Vec<i64>,
    pub grad_Q: Vec<i64>,
    pub w_tilde_Q: Vec<i64>,
    pub proj: ProjResult,
    pub slope_scaled: i64,
    pub gap_scaled: i64,
    pub accepted: bool,
    pub digest_hex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpascResult {
    pub final_state: SpascState,
    pub logs: Vec<SpascStepLog>,
    pub pass_all: bool,
}

fn apply_ablation(vec: &[i64], mask: Option<&[i64]>) -> Vec<i64> {
    let m = match mask {
        Some(mask_slice) => mask_slice,
        None => return vec.to_vec(),
    };
    let mut out = Vec::with_capacity(vec.len());
    for (idx, &value) in vec.iter().enumerate() {
        let mask_val = if idx >= m.len() { 1 } else { m[idx] };
        out.push(if mask_val != 0 { value } else { 0 });
    }
    out
}

fn sgd_step_int(w_q: &[i64], grad_q: &[i64], eta_q: i64, q: i64) -> Result<Vec<i64>, String> {
    if w_q.len() != grad_q.len() {
        return Err("grad length mismatch".to_string());
    }
    let mut step = Vec::with_capacity(w_q.len());
    for (&w, &g) in w_q.iter().zip(grad_q.iter()) {
        let delta = ((eta_q as i128 * g as i128) / q as i128) as i64;
        step.push(w.saturating_sub(delta));
    }
    Ok(step)
}

fn sha256_hex(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn stable_digest(log_payload: &Value) -> String {
    let canon = crate::petc::to_canonical_json(log_payload);
    sha256_hex(&canon)
}

pub fn spasc_step(
    state: &SpascState,
    cfg: &SpascConfig,
    grad_fn: &GradFn,
) -> Result<(SpascState, SpascStepLog), String> {
    let q = cfg.q;
    let w = state.w_Q.clone();
    let grad = grad_fn(&w, state.step);
    if grad.len() != w.len() {
        return Err("grad length mismatch".to_string());
    }

    let w_tilde = sgd_step_int(&w, &grad, cfg.eta_Q, q)?;
    let proj = project_dual_int(&w_tilde, &cfg.budgets);
    let w_star = apply_ablation(&proj.w_star_Q, cfg.ablate_mask.as_deref());

    let (accepted, metrics) = ace_accept(&w_star, &cfg.b_norms_Q, q, None);
    let slope = metrics.slope_scaled;

    let next_state = SpascState {
        step: state.step + 1,
        w_Q: w_star.clone(),
    };

    let mut b = cfg.budgets.b.clone();
    let mut a = cfg.budgets.a.clone();
    if b.len() < w_star.len() {
        b.resize(w_star.len(), 0);
    }
    if a.len() < w_star.len() {
        a.resize(w_star.len(), 0);
    }

    let mut sum1 = 0i64;
    let mut sum2 = 0i64;
    for i in 0..w_star.len() {
        sum1 = sum1.saturating_add(w_star[i].abs().saturating_mul(b[i].abs()));
        sum2 = sum2.saturating_add(w_star[i].abs().saturating_mul(a[i].abs()));
    }

    let log_payload = serde_json::json!({
        "step": state.step,
        "w_before_Q": w,
        "grad_Q": grad,
        "w_tilde_Q": w_tilde,
        "w_star_Q": w_star,
        "sum1": sum1,
        "sum2": sum2,
        "lam_Q": proj.lam_Q,
        "mu_Q": proj.mu_Q,
        "slope_scaled": slope,
        "gap_scaled": metrics.gap_scaled,
        "accepted": accepted,
        "Q": q,
    });
    let digest = stable_digest(&log_payload);

    let log = SpascStepLog {
        step: state.step,
        w_before_Q: w,
        grad_Q: grad,
        w_tilde_Q: w_tilde,
        proj: ProjResult {
            w_star_Q: w_star,
            sum1,
            sum2,
            lam_Q: proj.lam_Q,
            mu_Q: proj.mu_Q,
            iters: 0,
        },
        slope_scaled: slope,
        gap_scaled: metrics.gap_scaled,
        accepted,
        digest_hex: digest,
    };

    Ok((next_state, log))
}

pub fn spasc_train(
    w0_q: &[i64],
    cfg: &SpascConfig,
    grad_fn: &GradFn,
    max_steps: i32,
    stop_on_fault: bool,
) -> Result<SpascResult, String> {
    let mut state = SpascState {
        step: 0,
        w_Q: w0_q.to_vec(),
    };
    let mut logs = Vec::new();
    let mut pass_all = true;

    for _ in 0..max_steps {
        let (next_state, slog) = spasc_step(&state, cfg, grad_fn)?;
        logs.push(slog.clone());
        if !slog.accepted {
            pass_all = false;
            if stop_on_fault {
                state = next_state;
                break;
            }
        }
        state = next_state;
    }

    Ok(SpascResult {
        final_state: state,
        logs,
        pass_all,
    })
}
