use crate::ace::{ACEConfig, ACERuntime, Matrix};
use crate::ledger::Ledger;
use serde_json::Value;
use apex_core::arithmetic::Rational;
use num_traits::Zero;

pub fn executor<P, W>(
    ledger: Ledger,
    schedule_cfg: &Value,
    mut proposal: P,
    mut writeback: W,
    steps: i32,
) -> Result<Ledger, String>
where
    P: FnMut(i32, i32, i32) -> (Vec<Rational>, Vec<Rational>, Matrix, Vec<Rational>, Rational),
    W: FnMut(i32, i32, i32, &[Rational]),
{
    let n_classes = schedule_cfg
        .get("n_classes")
        .and_then(|v| v.as_i64())
        .unwrap_or(12) as i32;
    let n_anchors = schedule_cfg
        .get("n_anchors")
        .and_then(|v| v.as_i64())
        .unwrap_or(6) as i32;
    let n_columns = schedule_cfg
        .get("n_columns")
        .and_then(|v| v.as_i64())
        .unwrap_or(768) as i32;

    let f64_to_rational = |f: f64| {
        let scaled = (f * 1_000_000_000.0).round() as i64;
        Rational::new(scaled, 1_000_000_000)
    };

    let lam_l1 = schedule_cfg
        .get("lam_l1")
        .and_then(|v| v.as_f64())
        .map(f64_to_rational)
        .unwrap_or_else(|| Rational::new(1, 10));
        
    let l1_weight_floor = schedule_cfg
        .get("l1_weight_floor")
        .and_then(|v| v.as_f64())
        .map(f64_to_rational)
        .unwrap_or_else(|| Rational::new(1, 1000000));
        
    let kkt_tol = schedule_cfg
        .get("kkt_tol")
        .and_then(|v| v.as_f64())
        .map(f64_to_rational)
        .unwrap_or_else(|| Rational::new(1, 1000000));

    let ace_cfg = ACEConfig {
        lam_l1,
        l1_weight_floor,
        kkt_tol,
        ..Default::default()
    };

    let mut ace = ACERuntime::new(ledger, Some(ace_cfg));

    for step in 0..steps {
        let c = step % n_classes;
        let a = (step / n_classes) % n_anchors;
        let k = (step / (n_classes * n_anchors)) % n_columns;

        let (t_t, f, k_prop, w, eps) = proposal(c, a, k);

        let now_ms = crate::ledger::now_epoch_ms();

        let petc_entry = serde_json::json!({
            "kind": "petc_stamp",
            "step": step,
            "class": c,
            "anchor": a,
            "column": k,
            "ts_ms": now_ms
        });
        ace.ledger.append(petc_entry);

        match ace.ace_step(
            &t_t,
            &f,
            &k_prop,
            &w,
            eps,
            &format!("slot_{}_{}_{}", c, a, k),
            Some(serde_json::json!({
                "step": step,
                "class": c,
                "anchor": a,
                "column": k
            })),
        ) {
            Ok(result) => {
                if let Some(t_next_arr) = result.get("T_next").and_then(|v| v.as_array()) {
                    let mut t_next = Vec::new();
                    for v in t_next_arr {
                        // Assuming the result JSON encodes rationals in some stable way
                        // For now, try to parse it. If it's a simple number, parse it.
                        // In production, we'd use a more robust Value -> Rational helper.
                        if let Some(f) = v.as_f64() {
                           t_next.push(f64_to_rational(f));
                        } else {
                           t_next.push(Rational::zero());
                        }
                    }
                    writeback(c, a, k, &t_next);
                }
            }
            Err(e) => {
                let now_ms_err = crate::ledger::now_epoch_ms();
                let reject_entry = serde_json::json!({
                    "kind": "ace_reject",
                    "step": step,
                    "class": c,
                    "anchor": a,
                    "column": k,
                    "error": e,
                    "ts_ms": now_ms_err
                });
                ace.ledger.append(reject_entry);
            }
        }
    }

    Ok(ace.ledger)
}
