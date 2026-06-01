use crate::ledger::Ledger;
use serde_json::Value;
use std::collections::HashMap;
use apex_core::arithmetic::Rational;
use num_traits::{Zero, One, Signed};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ACEParams {
    pub x_t: Rational,   // measured magnitude or telemetry bound
    pub eps_t: Rational, // gap parameter in (0,1)
}

impl ACEParams {
    pub fn new(x_t: Rational, eps_t: Rational) -> Self {
        Self { x_t, eps_t }
    }

    pub fn radius(&self) -> Rational {
        let r = Rational::one() - self.eps_t - self.x_t;
        if r < Rational::zero() {
            Rational::zero()
        } else {
            r
        }
    }
}

pub struct ACEProjector;

impl ACEProjector {
    pub fn project_weighted_l1(
        w_hat: &HashMap<i32, Rational>,
        b: &HashMap<i32, Rational>,
        radius: Rational,
    ) -> Result<(HashMap<i32, Rational>, Rational), String> {
        let keys: Vec<i32> = w_hat.keys().cloned().collect();
        let mut y: Vec<Rational> = Vec::with_capacity(keys.len());
        for &k in &keys {
            let bk = b.get(&k).copied().unwrap_or_else(Rational::zero).abs();
            let whk = w_hat.get(&k).copied().unwrap_or_else(Rational::zero);
            y.push(bk * whk);
        }
        
        let mut l1 = Rational::zero();
        for val in &y {
            l1 += val.abs();
        }

        if radius >= l1 {
            return Ok((w_hat.clone(), (radius - l1).max(Rational::zero())));
        }

        if radius <= Rational::zero() || keys.is_empty() {
            let mut w = HashMap::new();
            for &k in &keys {
                w.insert(k, Rational::zero());
            }
            return Ok((w, Rational::zero()));
        }

        // Sort descending by absolute value
        let mut pairs: Vec<(Rational, usize)> = y
            .iter()
            .map(|v| v.abs())
            .enumerate()
            .map(|(i, val)| (val, i))
            .collect();
        
        pairs.sort_by(|a, b| b.0.cmp(&a.0));

        let mut cumsum = Rational::zero();
        let mut rho = 0;

        for (j_idx, &(u, _)) in pairs.iter().enumerate() {
            let j = (j_idx + 1) as i64;
            cumsum += u;
            // t = (cumsum - radius) / j
            let t = (cumsum - radius) / Rational::from_integer(j);
            if u > t {
                rho = j_idx + 1;
            }
        }

        let y_proj: Vec<Rational> = if rho == 0 {
            vec![Rational::zero(); keys.len()]
        } else {
            let sum_pairs_rho: Rational = pairs.iter().take(rho).map(|p| p.0).sum();
            let tau = (sum_pairs_rho - radius) / Rational::from_integer(rho as i64);
            y.iter()
                .zip(keys.iter())
                .map(|(&v, &k)| {
                    let whk = w_hat.get(&k).copied().unwrap_or_else(Rational::zero);
                    let sgn = if whk >= Rational::zero() { Rational::one() } else { -Rational::one() };
                    let abs_v = v.abs();
                    if abs_v > tau {
                        (abs_v - tau) * sgn
                    } else {
                        Rational::zero()
                    }
                })
                .collect()
        };

        let mut w = HashMap::new();
        for (&k, &vproj) in keys.iter().zip(y_proj.iter()) {
            let bi = b.get(&k).copied().unwrap_or_else(Rational::zero).abs();
            if bi <= Rational::zero() {
                return Err(format!("Nonpositive bound b[{}]={}", k, bi));
            }
            w.insert(k, vproj / bi);
        }

        let mut sum_bw = Rational::zero();
        for &k in &keys {
            let bk = b.get(&k).copied().unwrap_or_else(Rational::zero).abs();
            let wk = w.get(&k).copied().unwrap_or_else(Rational::zero).abs();
            sum_bw += bk * wk;
        }
        let gap_lb = radius - sum_bw;

        Ok((w, gap_lb))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KKTCertificate {
    pub residual: Rational,
    pub max_stationarity: Rational,
    pub max_comp_slack: Rational,
    pub active_frac: Rational,
}

pub fn soft_threshold_l1w(y: &[Rational], lam: Rational, w: &[Rational]) -> (Vec<Rational>, KKTCertificate) {
    assert_eq!(y.len(), w.len());
    let mut z = vec![Rational::zero(); y.len()];
    let mut active_count = 0;

    let mut max_stat = Rational::zero();
    let mut max_slack = Rational::zero();

    for i in 0..y.len() {
        let ay = y[i].abs();
        let thr = lam * w[i];
        let val = if ay > thr {
            (ay - thr) * (if y[i] >= Rational::zero() { Rational::one() } else { -Rational::one() })
        } else {
            Rational::zero()
        };
        z[i] = val;

        if !val.is_zero() {
            active_count += 1;
            let stat = (ay - val.abs() - thr).abs();
            if stat > max_stat {
                max_stat = stat;
            }
        } else {
            let slack = if ay > thr { ay - thr } else { Rational::zero() };
            if slack > max_slack {
                max_slack = slack;
            }
        }
    }

    let residual = max_stat.max(max_slack);
    let active_frac = if y.is_empty() {
        Rational::zero()
    } else {
        Rational::new(active_count as i64, y.len() as i64)
    };

    let cert = KKTCertificate {
        residual,
        max_stationarity: max_stat,
        max_comp_slack: max_slack,
        active_frac,
    };

    (z, cert)
}

pub use apex_core::arithmetic::DynamicRationalMatrix as Matrix;

pub fn project_operator_norm(k: &Matrix, radius: Rational) -> (Matrix, Rational, Rational) {
    let norm = k.row_sum_norm();
    if norm <= radius {
        return (k.clone(), Rational::one(), norm);
    }

    let scale = radius / norm;
    let mut k_proj = k.clone();
    for x in k_proj.data.iter_mut() {
        *x *= scale;
    }
    (k_proj, scale, norm)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ACEConfig {
    pub lam_l1: Rational,
    pub l1_weight_floor: Rational,
    pub kkt_tol: Rational,
    pub cauchy_tolerance: Rational,
}

impl Default for ACEConfig {
    fn default() -> Self {
        Self {
            lam_l1: Rational::new(1, 10),
            l1_weight_floor: Rational::new(1, 1000000),
            kkt_tol: Rational::new(1, 1000000),
            cauchy_tolerance: Rational::new(1, 1000000000000000i64),
        }
    }
}

pub struct ACERuntime {
    pub ledger: Ledger,
    pub cfg: ACEConfig,
    t: i32,
    last_err: Option<Rational>,
}

impl ACERuntime {
    pub fn new(ledger: Ledger, cfg: Option<ACEConfig>) -> Self {
        Self {
            ledger,
            cfg: cfg.unwrap_or_default(),
            t: 0,
            last_err: None,
        }
    }

    pub fn ace_step(
        &mut self,
        t_t: &[Rational],
        f: &[Rational],
        k_prop: &Matrix,
        w_l1: &[Rational],
        eps_t: Rational,
        actor_id: &str,
        context: Option<Value>,
    ) -> Result<Value, String> {
        let radius = Rational::one() - eps_t;
        let (k_proj, scale, norm) = project_operator_norm(k_prop, radius);

        assert_eq!(k_proj.cols, t_t.len());
        let mut t_cand = f.to_vec();
        let kv = k_proj.matvec(t_t);
        for (i, val) in t_cand.iter_mut().enumerate() {
            *val += kv[i];
        }

        let w: Vec<Rational> = w_l1
            .iter()
            .map(|&val| val.max(self.cfg.l1_weight_floor))
            .collect();
        let (z, cert) = soft_threshold_l1w(&t_cand, self.cfg.lam_l1, &w);

        assert_eq!(z.len(), t_t.len());
        let mut err = Rational::zero();
        for i in 0..z.len() {
            let diff = z[i] - t_t[i];
            err += diff * diff;
        }
        // err is now norm squared. In No Floats, we keep it as squared or use an approx.
        // For Cauchy check, err_sq <= last_err_sq is equivalent for positive err.

        let last_sq = self.last_err.unwrap_or(err);
        let cauchy_ok = err <= last_sq + self.cfg.cauchy_tolerance;

        let ok = cert.residual <= self.cfg.kkt_tol && cauchy_ok;

        let now_ms = crate::ledger::now_epoch_ms();

        let mut entry = serde_json::json!({
            "kind": "ace_step",
            "t": self.t,
            "actor_id": actor_id,
            "ts_ms": now_ms,
            "eps_t": eps_t,
            "radius": radius,
            "norm_before": norm,
            "scale_applied": scale,
            "kkt_residual": cert.residual,
            "kkt_max_stationarity": cert.max_stationarity,
            "kkt_max_comp_slack": cert.max_comp_slack,
            "active_frac": cert.active_frac,
            "cauchy_ok": cauchy_ok,
            "ok": ok,
            "backend": "rust",
            "context": context.unwrap_or(Value::Null)
        });

        if !ok {
            if let Some(obj) = entry.as_object_mut() {
                obj.insert("status".to_string(), Value::from("fail_closed"));
            }
            let eid = self.ledger.append(entry);
            self.ledger.broadcast(&eid);
            return Err(format!(
                "ACE step rejected: kkt={:?}, cauchy_ok={}",
                cert.residual, cauchy_ok
            ));
        }

        self.t += 1;
        self.last_err = Some(err);
        if let Some(obj) = entry.as_object_mut() {
            obj.insert("status".to_string(), Value::from("committed"));
        }
        let eid = self.ledger.append(entry);
        self.ledger.broadcast(&eid);

        let result = serde_json::json!({
            "T_next": z,
            "K_proj": {
                "rows": k_proj.rows,
                "cols": k_proj.cols,
                "data": k_proj.data
            },
            "ledger_entry_id": eid,
            "metrics": {
                "norm_before": norm,
                "scale_applied": scale,
                "kkt_residual": cert.residual,
                "kkt_max_stationarity": cert.max_stationarity,
                "kkt_max_comp_slack": cert.max_comp_slack,
                "active_frac": cert.active_frac,
                "cauchy_ok": cauchy_ok,
                "eps_t": eps_t,
                "backend": "rust"
            }
        });
        Ok(result)
    }
}
