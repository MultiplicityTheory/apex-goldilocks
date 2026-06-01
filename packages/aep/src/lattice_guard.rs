use crate::boundary_lattice::{decode, lin_index, verify_address};
use serde_json::{Map, Value};

pub fn encode(class_id: i32, anchor: i32, v_bits: i32) -> Result<i32, String> {
    if !(0..crate::boundary_lattice::CLASSES).contains(&class_id) {
        return Err("bad class".to_string());
    }
    if !(0..crate::boundary_lattice::ANCHORS).contains(&anchor) {
        return Err("bad anchor".to_string());
    }
    if !(0..crate::boundary_lattice::ORBIT).contains(&v_bits) {
        return Err("bad v_bits".to_string());
    }
    lin_index(anchor, v_bits)
}

pub fn guard_context(ctx: &Map<String, Value>) -> Result<Map<String, Value>, String> {
    let class_val = ctx
        .get("class")
        .ok_or_else(|| "missing class in context".to_string())?;

    let class_id = match class_val {
        Value::Bool(b) => {
            if *b {
                1
            } else {
                0
            }
        }
        Value::Number(n) => n
            .as_i64()
            .ok_or_else(|| "class must be an integer".to_string())? as i32,
        Value::String(s) => s
            .parse::<i32>()
            .map_err(|e| format!("cannot parse class: {}", e))?,
        _ => return Err("invalid class type".to_string()),
    };

    let idx = if let Some(coord_val) = ctx.get("coord") {
        coord_val
            .as_i64()
            .ok_or_else(|| "coord must be an integer".to_string())? as i32
    } else if let (Some(anchor_val), Some(v_bits_val)) = (ctx.get("anchor"), ctx.get("v_bits")) {
        let anchor = anchor_val
            .as_i64()
            .ok_or_else(|| "anchor must be an integer".to_string())? as i32;
        let v_bits = v_bits_val
            .as_i64()
            .ok_or_else(|| "v_bits must be an integer".to_string())? as i32;
        encode(class_id, anchor, v_bits)?
    } else {
        return Err("context must include either coord or (anchor,v_bits)".to_string());
    };

    verify_address(class_id, idx)?;
    let d = decode(class_id, idx)?;

    let mut out = ctx.clone();
    out.insert("coord".to_string(), Value::from(d.coord_idx));
    out.insert("anchor".to_string(), Value::from(d.anchor));
    out.insert("v_bits".to_string(), Value::from(d.v_bits));
    out.insert("row".to_string(), Value::from(d.row));
    out.insert("col".to_string(), Value::from(d.col));

    Ok(out)
}
