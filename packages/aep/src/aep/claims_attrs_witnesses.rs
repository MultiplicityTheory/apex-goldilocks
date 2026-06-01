use serde::{Serialize, Deserialize};
use serde_json::Value;

pub const ALLOWED_CLAIMS: &[&str] = &[
    "mirror_safe",
    "unity_neutral",
    "boundary_window",
    "phase_window",
    "classes_mask",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundaryWindow {
    pub shape: String,
    pub limits: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseWindow {
    pub phi0: String, // decimal-as-string
    pub span: String, // decimal-as-string
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AEPAttrs {
    pub claims: Vec<String>,
    pub boundary_window: Option<BoundaryWindow>,
    pub phase_window: Option<PhaseWindow>,
    pub classes_mask: Option<Vec<i64>>,
}

pub fn load_aep_toml(path: &str) -> Result<AEPAttrs, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let table: toml::Table = toml::from_str(&content).map_err(|e| e.to_string())?;

    let claims = table.get("claims")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|val| val.as_str().map(|s| s.to_string()))
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();

    for c in &claims {
        if !ALLOWED_CLAIMS.contains(&c.as_str()) {
            return Err(format!("unknown claim: {}", c));
        }
    }

    let mut boundary_window = None;
    if let Some(bw_val) = table.get("boundary_window") {
        if let Some(bw_table) = bw_val.as_table() {
            let shape = bw_table.get("shape")
                .and_then(|s| s.as_str())
                .ok_or("boundary_window requires shape")?
                .to_string();
            let limits = bw_table.get("limits")
                .and_then(|lim| lim.as_array())
                .ok_or("boundary_window requires limits")?
                .iter()
                .filter_map(|val| val.as_integer())
                .collect::<Vec<i64>>();
            boundary_window = Some(BoundaryWindow { shape, limits });
        }
    }

    let mut phase_window = None;
    if let Some(pw_val) = table.get("phase_window") {
        if let Some(pw_table) = pw_val.as_table() {
            let phi0 = pw_table.get("phi0")
                .and_then(|s| s.as_str())
                .unwrap_or("0")
                .to_string();
            let span = pw_table.get("span")
                .and_then(|s| s.as_str())
                .unwrap_or("0")
                .to_string();
            phase_window = Some(PhaseWindow { phi0, span });
        }
    }

    let mut classes_mask = None;
    if let Some(cm_val) = table.get("classes_mask") {
        if let Some(cm_arr) = cm_val.as_array() {
            classes_mask = Some(
                cm_arr.iter()
                    .filter_map(|val| val.as_integer())
                    .collect::<Vec<i64>>(),
            );
        }
    }

    Ok(AEPAttrs {
        claims,
        boundary_window,
        phase_window,
        classes_mask,
    })
}

pub fn kernel_attrs_from_attrs(attrs: &AEPAttrs) -> Value {
    let mut out = serde_json::Map::new();
    out.insert("claims".to_string(), serde_json::to_value(&attrs.claims).unwrap());
    if let Some(ref bw) = attrs.boundary_window {
        out.insert("boundary_window".to_string(), serde_json::json!({
            "shape": bw.shape,
            "limits": bw.limits,
        }));
    }
    if let Some(ref pw) = attrs.phase_window {
        out.insert("phase_window".to_string(), serde_json::json!({
            "phi0": pw.phi0,
            "span": pw.span,
        }));
    }
    if let Some(ref cm) = attrs.classes_mask {
        out.insert("classes_mask".to_string(), serde_json::to_value(cm).unwrap());
    }
    Value::Object(out)
}

fn get_q(theta: &Value) -> i64 {
    theta.get("Q")
        .and_then(|v| v.as_i64())
        .unwrap_or(1_000_000)
}

fn dec_str_to_int(s: &str, q: i64) -> i64 {
    if s.contains('/') {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() == 2 {
            let n = parts[0].trim().parse::<i64>().unwrap_or(0);
            let d = parts[1].trim().parse::<i64>().unwrap_or(1);
            return (n * q) / d;
        }
    }
    let val: f64 = s.parse().unwrap_or(0.0);
    (val * (q as f64)).round() as i64
}

fn to_scaled_ints(vals: &Value, q: i64) -> Vec<i64> {
    let mut out = Vec::new();
    if let Some(arr) = vals.as_array() {
        for v in arr {
            let val = if let Some(n) = v.as_i64() {
                n * q
            } else if let Some(f) = v.as_f64() {
                (f * q as f64).round() as i64
            } else if let Some(s) = v.as_str() {
                dec_str_to_int(s, q)
            } else {
                0
            };
            out.push(val);
        }
    }
    out
}

pub fn normalize_witness(w_in: &Value, theta: &Value) -> Value {
    let q = get_q(theta);
    let mut out = if let Some(obj) = w_in.as_object() {
        obj.clone()
    } else {
        serde_json::Map::new()
    };

    if !out.contains_key("delta_R") && out.contains_key("R_pre") && out.contains_key("R_post") {
        let pre = to_scaled_ints(&out["R_pre"], q);
        let post = to_scaled_ints(&out["R_post"], q);
        let n = pre.len().min(post.len());
        let delta: Vec<i64> = (0..n).map(|i| post[i] - pre[i]).collect();
        out.insert("delta_R".to_string(), serde_json::to_value(&delta).unwrap());
    }

    if let Some(p_probes) = out.get_mut("P_probes").and_then(|v| v.as_array_mut()) {
        for p in p_probes {
            if let Some(p_obj) = p.as_object_mut() {
                if let Some(phi_val) = p_obj.get("phi") {
                    let phi_str = if let Some(s) = phi_val.as_str() {
                        s.to_string()
                    } else {
                        phi_val.to_string()
                    };
                    p_obj.insert("phi_Q".to_string(), Value::from(dec_str_to_int(&phi_str, q)));
                }
            }
        }
    }

    out.insert("Q".to_string(), Value::from(q));
    Value::Object(out)
}

pub fn canonical_unity_neutral(w: &Value) -> (bool, Value) {
    let delta_r = match w.get("delta_R") {
        Some(d) => d,
        None => return (false, serde_json::json!({ "reason": "missing delta_R" })),
    };
    let q = w.get("Q").and_then(|v| v.as_i64()).unwrap_or(0);
    let delta_ints = to_scaled_ints(delta_r, 1); // delta_R are already scaled ints
    let s: i64 = delta_ints.iter().map(|x| x.abs()).sum();
    (s == 0, serde_json::json!({ "delta_R_L1_scaled": s, "Q": q }))
}

pub fn canonical_boundary_window(w: &Value, attrs: &AEPAttrs) -> (bool, Value) {
    let bw = match &attrs.boundary_window {
        Some(b) => b,
        None => return (true, serde_json::json!({ "skipped": true })),
    };
    let limits = &bw.limits;
    let fp = match w.get("footprint") {
        Some(f) => f,
        None => return (false, serde_json::json!({ "reason": "missing footprint" })),
    };

    let inside = |box_vec: &[i64]| -> bool {
        if limits.len() != 4 || box_vec.len() != 4 {
            return false;
        }
        let (xmin, xmax, ymin, ymax) = (box_vec[0], box_vec[1], box_vec[2], box_vec[3]);
        let (lxmin, lxmax, lymin, lymax) = (limits[0], limits[1], limits[2], limits[3]);
        (lxmin <= xmin && xmin <= lxmax)
            && (lxmin <= xmax && xmax <= lxmax)
            && (lymin <= ymin && ymin <= lymax)
            && (lymin <= ymax && ymax <= lymax)
    };

    if let Some(arr) = fp.as_array() {
        if arr.iter().all(|v| v.is_i64()) {
            let box_vec: Vec<i64> = arr.iter().filter_map(|v| v.as_i64()).collect();
            let ok = inside(&box_vec);
            return (ok, serde_json::json!({ "footprint": box_vec, "limits": limits }));
        }
        if arr.iter().all(|v| v.is_array()) {
            let mut oks = Vec::new();
            for sub_val in arr {
                let box_vec: Vec<i64> = sub_val.as_array().unwrap().iter().filter_map(|v| v.as_i64()).collect();
                oks.push(inside(&box_vec));
            }
            let ok = oks.iter().all(|&b| b);
            let violations = oks.iter().filter(|&&b| !b).count() as i64;
            return (ok, serde_json::json!({ "count": arr.len(), "violations": violations }));
        }
    }

    (false, serde_json::json!({ "reason": "unsupported footprint type" }))
}

pub fn canonical_phase_window(w: &Value, attrs: &AEPAttrs, theta: &Value) -> (bool, Value) {
    let pw = match &attrs.phase_window {
        Some(p) => p,
        None => return (true, serde_json::json!({ "skipped": true })),
    };
    let q = get_q(theta);
    let phi0_q = dec_str_to_int(&pw.phi0, q);
    let span_q = dec_str_to_int(&pw.span, q);
    let (lo, hi) = (phi0_q, phi0_q + span_q);

    let mut vals = Vec::new();
    if let Some(phase_vals) = w.get("phase_values") {
        vals = to_scaled_ints(phase_vals, q);
    } else if let Some(p_probes) = w.get("P_probes").and_then(|v| v.as_array()) {
        for p in p_probes {
            if let Some(phi_q) = p.get("phi_Q").and_then(|v| v.as_i64()) {
                vals.push(phi_q);
            }
        }
    } else {
        return (false, serde_json::json!({ "reason": "missing phase probes" }));
    }

    let ok = vals.iter().all(|&v| lo <= v && v <= hi);
    (ok, serde_json::json!({ "lo_Q": lo, "hi_Q": hi, "count": vals.len() }))
}

pub fn canonical_mirror_safe(w: &Value) -> (bool, Value) {
    let ok = w.get("mirror_equal").and_then(|v| v.as_bool()).unwrap_or(false);
    (ok, serde_json::json!({ "mirror_equal": ok }))
}

pub fn canonical_classes_mask(w: &Value, attrs: &AEPAttrs) -> (bool, Value) {
    if attrs.classes_mask.is_none() {
        return (true, serde_json::json!({ "skipped": true }));
    }
    let ok = w.get("classes_ok").and_then(|v| v.as_bool()).unwrap_or(false);
    (ok, serde_json::json!({ "classes_ok": ok }))
}

pub fn prepare_claims_attrs_witness(
    aep_toml_path: &str,
    w_in: &Value,
    theta: &Value,
) -> Result<(Value, Value, Value), String> {
    let attrs = load_aep_toml(aep_toml_path)?;
    let attrs_val = kernel_attrs_from_attrs(&attrs);
    let mut w = normalize_witness(w_in, theta);

    let mut unity_res = None;
    if attrs.claims.contains(&"unity_neutral".to_string()) {
        unity_res = Some(canonical_unity_neutral(&w));
    }
    let mut mirror_res = None;
    if attrs.claims.contains(&"mirror_safe".to_string()) {
        mirror_res = Some(canonical_mirror_safe(&w));
    }
    let mut boundary_res = None;
    if attrs.claims.contains(&"boundary_window".to_string()) {
        boundary_res = Some(canonical_boundary_window(&w, &attrs));
    }
    let mut phase_res = None;
    if attrs.claims.contains(&"phase_window".to_string()) {
        phase_res = Some(canonical_phase_window(&w, &attrs, theta));
    }
    let mut classes_res = None;
    if attrs.claims.contains(&"classes_mask".to_string()) {
        classes_res = Some(canonical_classes_mask(&w, &attrs));
    }

    if let Some(w_obj) = w.as_object_mut() {
        if let Some((ok, ev)) = unity_res {
            w_obj.insert("unity_ok".to_string(), Value::Bool(ok));
            w_obj.insert("unity_ev".to_string(), ev);
        }
        if let Some((ok, ev)) = mirror_res {
            w_obj.insert("mirror_equal".to_string(), Value::Bool(ok));
            w_obj.insert("mirror_ev".to_string(), ev);
        }
        if let Some((ok, ev)) = boundary_res {
            w_obj.insert("boundary_ok".to_string(), Value::Bool(ok));
            w_obj.insert("boundary_ev".to_string(), ev);
        }
        if let Some((ok, ev)) = phase_res {
            w_obj.insert("phase_ok".to_string(), Value::Bool(ok));
            w_obj.insert("phase_ev".to_string(), ev);
        }
        if let Some((ok, ev)) = classes_res {
            w_obj.insert("classes_ok".to_string(), Value::Bool(ok));
            w_obj.insert("classes_ev".to_string(), ev);
        }
    }

    Ok((attrs_val.clone(), attrs_val, w))
}
