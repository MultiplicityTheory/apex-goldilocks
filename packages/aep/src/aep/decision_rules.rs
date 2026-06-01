use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AEP {
    pub c: Vec<String>,
    pub w: Value,
    pub theta: Value,
}

pub trait Kernel {
    fn eval(&self, ctx: &Value) -> Result<Value, String>;
}

pub trait Predicate {
    fn name(&self) -> &str;
    fn code(&self) -> i32;
    fn check(&self, w: &Value, ctx: &Value) -> (bool, Value);
    fn resolve(&self, w: &Value, ctx: &Value) -> (bool, Value);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub status: String, // "PASS" or "FAULT"
    pub code: i32,
    pub reason: String,
    pub evidence: Value,
    pub log: Vec<Value>,
}

// Deterministic clock support (optional)
static mut CLOCK_FN: Option<fn() -> u64> = None;

pub fn set_clock_ns(fn_ptr: Option<fn() -> u64>) {
    unsafe {
        CLOCK_FN = fn_ptr;
    }
}

fn now_ns() -> u64 {
    unsafe {
        if let Some(f) = CLOCK_FN {
            f()
        } else {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64
        }
    }
}

fn create_event(op: &str, ok: bool, info: Value) -> Value {
    serde_json::json!({
        "op": op,
        "ts": now_ns(),
        "ok": ok,
        "info": info,
    })
}

fn create_check_event(op: &str, claim: &str, ok: bool, ev: Value) -> Value {
    serde_json::json!({
        "op": op,
        "claim": claim,
        "ts": now_ns(),
        "ok": ok,
        "ev": ev,
    })
}

fn create_resolve_event(op: &str, claim: &str, ok: bool, ev: Value) -> Value {
    serde_json::json!({
        "op": op,
        "claim": claim,
        "ts": now_ns(),
        "ok": ok,
        "ev": ev,
    })
}

pub fn canon_decision(d: &Decision) -> Value {
    let log_stripped: Vec<Value> = d.log.iter().map(|e| {
        if let Some(obj) = e.as_object() {
            let mut new_obj = obj.clone();
            new_obj.remove("ts");
            Value::Object(new_obj)
        } else {
            e.clone()
        }
    }).collect();

    serde_json::json!({
        "status": d.status,
        "code": d.code,
        "reason": d.reason,
        "evidence": d.evidence,
        "log": log_stripped,
    })
}

pub fn get_quantum(theta: &Value) -> i64 {
    theta.get("Q")
        .and_then(|v| v.as_i64())
        .unwrap_or(1_000_000)
}

pub fn to_int_vec(vec: &Value, quantum: i64) -> Vec<i64> {
    let mut out = Vec::new();
    if let Some(arr) = vec.as_array() {
        for v in arr {
            let val = if let Some(n) = v.as_i64() {
                n * quantum
            } else if let Some(f) = v.as_f64() {
                // Use a simple fixed-precision conversion for legacy float support
                (f * quantum as f64).round() as i64
            } else if let Some(s) = v.as_str() {
                // Try parsing as rational "num/den" or float string
                if s.contains('/') {
                    let parts: Vec<&str> = s.split('/').collect();
                    if parts.len() == 2 {
                        let n = parts[0].trim().parse::<i64>().unwrap_or(0);
                        let d = parts[1].trim().parse::<i64>().unwrap_or(1);
                        (n * quantum) / d
                    } else {
                        0
                    }
                } else {
                    match s.parse::<f64>() {
                        Ok(f) => (f * quantum as f64).round() as i64,
                        Err(_) => 0,
                    }
                }
            } else {
                0
            };
            out.push(val);
        }
    }
    out
}

pub fn isa_assert_attrs(c: &[String], attrs: &Value) -> (bool, Value) {
    let claims_list = attrs.get("claims")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();

    let mut missing = Vec::new();
    for k in c {
        if !claims_list.contains(k) {
            missing.push(k.clone());
        }
    }

    let ok = missing.is_empty();
    let info = serde_json::json!({
        "missing": missing,
        "attrs_claims": claims_list,
    });
    (ok, info)
}

pub fn isa_eval_kernel<K: Kernel>(kernel: &K, ctx: &Value) -> (bool, Value) {
    match kernel.eval(ctx) {
        Ok(out) => (true, serde_json::json!({ "kernel_out": out })),
        Err(e) => (false, serde_json::json!({ "error": e })),
    }
}

pub fn isa_audit_witness(w: &Value) -> (bool, Value) {
    let has_r_pre = w.get("R_pre").is_some();
    let has_r_post = w.get("R_post").is_some();
    let has_delta_r = w.get("delta_R").is_some();
    let ok = (has_r_pre && has_r_post) || has_delta_r;

    let mut keys = Vec::new();
    if let Some(obj) = w.as_object() {
        keys = obj.keys().cloned().collect();
        keys.sort();
    }

    (ok, serde_json::json!({ "keys": keys }))
}

pub fn launch<K: Kernel>(
    aep: &AEP,
    predicates: &HashMap<String, Box<dyn Predicate>>,
    attrs: &Value,
    kernel: &K,
) -> Decision {
    let mut log = Vec::new();

    let (ok, info_attrs) = isa_assert_attrs(&aep.c, attrs);
    log.push(create_event("ASSERT", ok, info_attrs.clone()));
    if !ok {
        return Decision {
            status: "FAULT".to_string(),
            code: 100,
            reason: "attrs missing declared claims".to_string(),
            evidence: info_attrs,
            log,
        };
    }

    let kernel_ctx = serde_json::json!({
        "theta": aep.theta,
    });
    let (ok, info_eval) = isa_eval_kernel(kernel, &kernel_ctx);
    log.push(create_event("EVAL", ok, info_eval.clone()));
    if !ok {
        return Decision {
            status: "FAULT".to_string(),
            code: 120,
            reason: "kernel eval failed".to_string(),
            evidence: info_eval,
            log,
        };
    }

    let (ok, info_audit) = isa_audit_witness(&aep.w);
    log.push(create_event("AUDIT", ok, info_audit.clone()));
    if !ok {
        return Decision {
            status: "FAULT".to_string(),
            code: 140,
            reason: "witness malformed".to_string(),
            evidence: info_audit,
            log,
        };
    }

    let kernel_out = info_eval.get("kernel_out").cloned().unwrap_or(Value::Null);
    let run_ctx = serde_json::json!({
        "kernel_out": kernel_out,
        "theta": aep.theta,
    });

    for cname in &aep.c {
        if let Some(predicate) = predicates.get(cname) {
            let (ok, ev_check) = predicate.check(&aep.w, &run_ctx);
            log.push(create_check_event("CHECK", cname, ok, ev_check.clone()));
            if !ok {
                let (resolved, ev_resolve) = predicate.resolve(&aep.w, &run_ctx);
                log.push(create_resolve_event("RESOLVE", cname, resolved, ev_resolve.clone()));
                if !resolved {
                    let evidence = serde_json::json!({
                        "check": ev_check,
                        "resolve": ev_resolve,
                    });
                    return Decision {
                        status: "FAULT".to_string(),
                        code: predicate.code(),
                        reason: format!("predicate {} failed and unresolved", cname),
                        evidence,
                        log,
                    };
                }
            }
        }
    }

    Decision {
        status: "PASS".to_string(),
        code: 0,
        reason: "all predicates satisfied or resolved".to_string(),
        evidence: serde_json::json!({}),
        log,
    }
}

pub struct UnityNeutral;

impl Predicate for UnityNeutral {
    fn name(&self) -> &str {
        "unity_neutral"
    }

    fn code(&self) -> i32 {
        2001
    }

    fn check(&self, w: &Value, ctx: &Value) -> (bool, Value) {
        let theta = ctx.get("theta").unwrap_or(&Value::Null);
        let quantum = get_quantum(theta);

        if w.get("delta_R").is_some() {
            let delta = to_int_vec(w.get("delta_R").unwrap(), quantum);
            let l1: i64 = delta.iter().map(|x| x.abs()).sum();
            let ok = delta.iter().all(|&x| x == 0);
            return (ok, serde_json::json!({ "delta_R_L1_scaled": l1, "Q": quantum }));
        }

        if w.get("R_pre").is_some() && w.get("R_post").is_some() {
            let pre = to_int_vec(w.get("R_pre").unwrap(), quantum);
            let post = to_int_vec(w.get("R_post").unwrap(), quantum);
            let length = pre.len().min(post.len());
            let mut diff = Vec::with_capacity(length);
            for i in 0..length {
                diff.push(post[i] - pre[i]);
            }
            let l1: i64 = diff.iter().map(|x| x.abs()).sum();
            let ok = diff.iter().all(|&x| x == 0);
            return (ok, serde_json::json!({
                "delta_R_L1_scaled": l1,
                "len_pre": pre.len(),
                "len_post": post.len(),
                "Q": quantum,
            }));
        }

        (false, serde_json::json!({ "reason": "missing resonance fields" }))
    }

    fn resolve(&self, _w: &Value, _ctx: &Value) -> (bool, Value) {
        (false, serde_json::json!({ "reason": "unresolvable at launch" }))
    }
}

pub struct MirrorSafe;

impl Predicate for MirrorSafe {
    fn name(&self) -> &str {
        "mirror_safe"
    }

    fn code(&self) -> i32 {
        2002
    }

    fn check(&self, w: &Value, _ctx: &Value) -> (bool, Value) {
        let ok = w.get("mirror_equal").and_then(|v| v.as_bool()).unwrap_or(false);
        (ok, serde_json::json!({ "mirror_equal": ok }))
    }

    fn resolve(&self, _w: &Value, _ctx: &Value) -> (bool, Value) {
        (false, serde_json::json!({ "reason": "unresolvable at launch" }))
    }
}

pub struct BoundaryWindow;

impl Predicate for BoundaryWindow {
    fn name(&self) -> &str {
        "boundary_window"
    }

    fn code(&self) -> i32 {
        2003
    }

    fn check(&self, w: &Value, _ctx: &Value) -> (bool, Value) {
        let ok = w.get("boundary_ok").and_then(|v| v.as_bool()).unwrap_or(false);
        (ok, serde_json::json!({ "boundary_ok": ok }))
    }

    fn resolve(&self, _w: &Value, _ctx: &Value) -> (bool, Value) {
        (false, serde_json::json!({ "reason": "unresolvable at launch" }))
    }
}

pub struct PhaseWindow;

impl Predicate for PhaseWindow {
    fn name(&self) -> &str {
        "phase_window"
    }

    fn code(&self) -> i32 {
        2004
    }

    fn check(&self, w: &Value, _ctx: &Value) -> (bool, Value) {
        let ok = w.get("phase_ok").and_then(|v| v.as_bool()).unwrap_or(false);
        (ok, serde_json::json!({ "phase_ok": ok }))
    }

    fn resolve(&self, _w: &Value, _ctx: &Value) -> (bool, Value) {
        (false, serde_json::json!({ "reason": "unresolvable at launch" }))
    }
}

pub struct ClassesMask;

impl Predicate for ClassesMask {
    fn name(&self) -> &str {
        "classes_mask"
    }

    fn code(&self) -> i32 {
        2005
    }

    fn check(&self, w: &Value, _ctx: &Value) -> (bool, Value) {
        let ok = w.get("classes_ok").and_then(|v| v.as_bool()).unwrap_or(false);
        (ok, serde_json::json!({ "classes_ok": ok }))
    }

    fn resolve(&self, _w: &Value, _ctx: &Value) -> (bool, Value) {
        (false, serde_json::json!({ "reason": "unresolvable at launch" }))
    }
}

pub fn get_default_predicates() -> HashMap<String, Box<dyn Predicate>> {
    let mut map = HashMap::new();
    map.insert("unity_neutral".to_string(), Box::new(UnityNeutral) as Box<dyn Predicate>);
    map.insert("mirror_safe".to_string(), Box::new(MirrorSafe) as Box<dyn Predicate>);
    map.insert("boundary_window".to_string(), Box::new(BoundaryWindow) as Box<dyn Predicate>);
    map.insert("phase_window".to_string(), Box::new(PhaseWindow) as Box<dyn Predicate>);
    map.insert("classes_mask".to_string(), Box::new(ClassesMask) as Box<dyn Predicate>);
    map
}
