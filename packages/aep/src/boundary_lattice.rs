use serde_json::Value;
use std::collections::HashSet;

pub const CLASSES: i32 = 96;
pub const ANCHORS: i32 = 6;
pub const ORBIT: i32 = 2048;
pub const FOLD_ROWS: i32 = 48;
pub const FOLD_COLS: i32 = 256;
pub const COORDINATES: i32 = 12288;

pub fn lin_index(anchor: i32, v_bits: i32) -> Result<i32, String> {
    if !(0..ANCHORS).contains(&anchor) {
        return Err(format!(
            "anchor must be in [0, {}], got {}",
            ANCHORS - 1,
            anchor
        ));
    }
    if !(0..ORBIT).contains(&v_bits) {
        return Err(format!(
            "v_bits must be in [0, {}], got {}",
            ORBIT - 1,
            v_bits
        ));
    }
    Ok(anchor * ORBIT + v_bits)
}

pub fn inv_lin_index(coord_idx: i32) -> Result<(i32, i32), String> {
    if !(0..COORDINATES).contains(&coord_idx) {
        return Err(format!(
            "coord_idx must be in [0, {}], got {}",
            COORDINATES - 1,
            coord_idx
        ));
    }
    let anchor = coord_idx / ORBIT;
    let v_bits = coord_idx % ORBIT;
    Ok((anchor, v_bits))
}

pub fn boundary_fold_48x256(coord_idx: i32) -> Result<(i32, i32), String> {
    if !(0..COORDINATES).contains(&coord_idx) {
        return Err(format!(
            "coord_idx must be in [0, {}], got {}",
            COORDINATES - 1,
            coord_idx
        ));
    }
    let row = coord_idx / FOLD_COLS;
    let col = coord_idx % FOLD_COLS;
    Ok((row, col))
}

pub fn boundary_unfold_48x256(row: i32, col: i32) -> Result<i32, String> {
    if !(0..FOLD_ROWS).contains(&row) {
        return Err(format!("row must be in [0, {}], got {}", FOLD_ROWS - 1, row));
    }
    if !(0..FOLD_COLS).contains(&col) {
        return Err(format!("col must be in [0, {}], got {}", FOLD_COLS - 1, col));
    }
    Ok(row * FOLD_COLS + col)
}

pub fn verify_address(class_id: i32, coord_idx: i32) -> Result<(), String> {
    if !(0..CLASSES).contains(&class_id) {
        return Err(format!(
            "class_id must be in [0, {}], got {}",
            CLASSES - 1,
            class_id
        ));
    }
    if !(0..COORDINATES).contains(&coord_idx) {
        return Err(format!(
            "coord_idx must be in [0, {}], got {}",
            COORDINATES - 1,
            coord_idx
        ));
    }
    Ok(())
}

pub fn apply_subgroup_action(anchor: i32, v_bits: i32, u: i32) -> Result<(i32, i32), String> {
    if !(0..ANCHORS).contains(&anchor) {
        return Err(format!(
            "anchor must be in [0, {}], got {}",
            ANCHORS - 1,
            anchor
        ));
    }
    if !(0..ORBIT).contains(&v_bits) {
        return Err(format!(
            "v_bits must be in [0, {}], got {}",
            ORBIT - 1,
            v_bits
        ));
    }
    if !(0..ORBIT).contains(&u) {
        return Err(format!("u must be in [0, {}], got {}", ORBIT - 1, u));
    }
    let v_bits_new = (v_bits + u) % ORBIT;
    Ok((anchor, v_bits_new))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationResults {
    pub freeness: bool,
    pub transitivity: bool,
    pub orbit_sizes_correct: bool,
    pub failures: Vec<String>,
    pub verified: bool,
}

pub fn verify_subgroup_action() -> VerificationResults {
    let mut failures = Vec::new();
    let mut freeness = true;

    let test_u_values = vec![1, 2, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047];
    let sample_v_bits = vec![0, 1, 127, 255, 511, 1023, 2047];

    for anchor in 0..ANCHORS {
        for &v in &sample_v_bits {
            for &u in &test_u_values {
                if let Ok((_, v_new)) = apply_subgroup_action(anchor, v, u) {
                    if v_new == v {
                        freeness = false;
                        failures.push(format!(
                            "Fixed point: anchor={}, v={}, u={}",
                            anchor, v, u
                        ));
                    }
                }
            }
        }
    }

    let mut orbit_sizes_correct = true;
    let mut transitivity = true;

    for anchor in 0..ANCHORS {
        let mut orbit = HashSet::new();
        let start_v = 0;
        for u in 0..ORBIT {
            if let Ok((_, v_new)) = apply_subgroup_action(anchor, start_v, u) {
                orbit.insert(v_new);
            }
        }

        if orbit.len() != ORBIT as usize {
            orbit_sizes_correct = false;
            failures.push(format!(
                "Orbit size {} != {} for anchor {}",
                orbit.len(),
                ORBIT,
                anchor
            ));
        }

        let expected_orbit: HashSet<i32> = (0..ORBIT).collect();
        if orbit != expected_orbit {
            transitivity = false;
            failures.push(format!(
                "Orbit doesn't cover all elements for anchor {}",
                anchor
            ));
        }
    }

    let verified = freeness && transitivity && orbit_sizes_correct;

    VerificationResults {
        freeness,
        transitivity,
        orbit_sizes_correct,
        failures,
        verified,
    }
}

pub fn generate_certificate() -> Value {
    let verification = verify_subgroup_action();
    let structure_data = serde_json::json!({
        "classes": CLASSES,
        "anchors": ANCHORS,
        "orbit": ORBIT,
        "fold_rows": FOLD_ROWS,
        "fold_cols": FOLD_COLS,
        "coordinates": COORDINATES
    });

    let canon_structure = crate::petc::to_canonical_json(&structure_data);
    let checksum = crate::petc::sha256_hex(&canon_structure);

    let anchor_points: Vec<Value> = (0..ANCHORS)
        .map(|a| serde_json::json!({ "anchor": a, "coord_base": a * ORBIT }))
        .collect();

    serde_json::json!({
        "version": "1.0",
        "group_structure": "(Z/2)^11",
        "structure": structure_data,
        "structure_checksum": checksum,
        "verification": {
            "freeness": verification.freeness,
            "transitivity": verification.transitivity,
            "orbit_sizes_correct": verification.orbit_sizes_correct,
            "failures": verification.failures,
            "verified": verification.verified
        },
        "anchor_points": anchor_points,
        "properties": {
            "group_order": ORBIT,
            "num_orbits": ANCHORS,
            "total_elements": COORDINATES,
            "action_free": verification.freeness,
            "action_transitive": verification.transitivity
        }
    })
}

pub fn save_certificate(filepath: &str) -> Result<(), std::io::Error> {
    let cert = generate_certificate();
    let content = serde_json::to_string_pretty(&cert)?;
    std::fs::write(filepath, content)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Decoded {
    pub class_id: i32,
    pub coord_idx: i32,
    pub anchor: i32,
    pub v_bits: i32,
    pub row: i32,
    pub col: i32,
}

pub fn decode(class_id: i32, coord_idx: i32) -> Result<Decoded, String> {
    verify_address(class_id, coord_idx)?;
    let (a, v) = inv_lin_index(coord_idx)?;
    let (r, c) = boundary_fold_48x256(coord_idx)?;
    Ok(Decoded {
        class_id,
        coord_idx,
        anchor: a,
        v_bits: v,
        row: r,
        col: c,
    })
}
