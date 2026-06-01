use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

pub fn factor_int(n: i64) -> HashMap<i64, i64> {
    if n < 1 {
        panic!("n must be >= 1");
    }
    let mut out = HashMap::new();
    let mut x = n;
    let mut p = 2;
    while p * p <= x {
        while x % p == 0 {
            *out.entry(p).or_insert(0) += 1;
            x /= p;
        }
        p = if p == 2 { 3 } else { p + 2 };
    }
    if x > 1 {
        *out.entry(x).or_insert(0) += 1;
    }
    out
}

pub fn axis_signature(length: i64) -> HashMap<i64, i64> {
    factor_int(length)
}

pub fn tensor_signature(axis_lengths: &[i64]) -> HashMap<i64, i64> {
    let mut sig = HashMap::new();
    for &n in axis_lengths {
        for (p, e) in axis_signature(n) {
            *sig.entry(p).or_insert(0) += e;
        }
    }
    sig
}

pub fn add_signatures(
    sig_a: &HashMap<i64, i64>,
    sig_b: &HashMap<i64, i64>,
) -> HashMap<i64, i64> {
    let mut out = sig_a.clone();
    for (&p, &e) in sig_b {
        *out.entry(p).or_insert(0) += e;
    }
    out
}

pub fn eq_signatures(
    sig_a: &HashMap<i64, i64>,
    sig_b: &HashMap<i64, i64>,
) -> bool {
    let all_keys: HashSet<i64> = sig_a.keys().copied().chain(sig_b.keys().copied()).collect();
    for &k in &all_keys {
        if sig_a.get(&k).copied().unwrap_or(0) != sig_b.get(&k).copied().unwrap_or(0) {
            return false;
        }
    }
    true
}

pub fn cert_tensor_product(
    sig_a: &HashMap<i64, i64>,
    sig_b: &HashMap<i64, i64>,
    sig_out: &HashMap<i64, i64>,
) -> bool {
    eq_signatures(&add_signatures(sig_a, sig_b), sig_out)
}

pub fn cert_contraction(
    axes_a: &[i64],
    axes_b: &[i64],
    pairings: &[(usize, usize)],
) -> bool {
    for &(i, j) in pairings {
        if i >= axes_a.len() || j >= axes_b.len() {
            return false;
        }
        if !eq_signatures(&axis_signature(axes_a[i]), &axis_signature(axes_b[j])) {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelRow {
    pub id: String,
    pub sigma: HashMap<i64, i64>,
    pub budget: i64,
    pub commutator_class: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PETCChannelLedger {
    pub rows: HashMap<String, ChannelRow>,
    pub commutator_budget: HashMap<String, i64>,
}

impl PETCChannelLedger {
    pub fn new() -> Self {
        Self {
            rows: HashMap::new(),
            commutator_budget: HashMap::new(),
        }
    }

    pub fn add_channel(&mut self, row: ChannelRow) -> Result<(), String> {
        if self.rows.contains_key(&row.id) {
            return Err("duplicate channel id".to_string());
        }
        if !self.commutator_budget.contains_key(&row.commutator_class) {
            self.commutator_budget.insert(row.commutator_class.clone(), row.budget);
        }
        self.rows.insert(row.id.clone(), row);
        Ok(())
    }

    pub fn decrement_commutator(&mut self, comm_class: &str, amount: i64) -> Result<i64, String> {
        if amount < 0 {
            return Err("amount must be >= 0".to_string());
        }
        let rem = self.commutator_budget.get_mut(comm_class).ok_or_else(|| "unknown commutator class".to_string())?;
        if *rem < amount {
            return Err("commutator budget breach".to_string());
        }
        *rem -= amount;
        Ok(*rem)
    }

    pub fn get_budget(&self, comm_class: &str) -> i64 {
        self.commutator_budget.get(comm_class).copied().unwrap_or(0)
    }
}

pub fn pack_rb(p: i32, b: i32) -> Result<(i32, i32), String> {
    if !(0..48).contains(&p) || !(0..256).contains(&b) {
        return Err("p in [0,47], b in [0,255]".to_string());
    }
    let r = p / 8;
    let idx = (p % 8) * 256 + b;
    Ok((r, idx))
}

pub fn unpack_rb(r: i32, idx: i32) -> Result<(i32, i32), String> {
    if !(0..6).contains(&r) || !(0..2048).contains(&idx) {
        return Err("r in [0,5], idx in [0,2047]".to_string());
    }
    let p = 8 * r + (idx / 256);
    let b = idx % 256;
    Ok((p, b))
}

pub fn act_u(p: i32, b: i32, u: i32) -> (i32, i32) {
    let (r, idx) = pack_rb(p, b).unwrap();
    let u_mod = u & 2047;
    let idx2 = (idx + u_mod) & 2047;
    unpack_rb(r, idx2).unwrap()
}

pub fn anchors_s() -> Vec<(i32, i32)> {
    (0..6).map(|m| (8 * m, 0)).collect()
}

pub fn phi(p: i32, b: i32) -> (i32, i32) {
    ((p + 16) % 48, (b + 1) % 256)
}

pub fn phi_pow(p: i32, b: i32, k: i32) -> (i32, i32) {
    let p2 = (p + (16 * (k % 48))) % 48;
    let b2 = (b + (k % 256)) % 256;
    (p2, b2)
}

pub fn verify_freeness_sample(samples: usize) -> bool {
    let mut gs = anchors_s();
    for r in 0..6 {
        for &j in &[1, 255, 511, 1023, 2047] {
            if let Ok(g) = unpack_rb(r, j) {
                gs.push(g);
            }
        }
    }
    let us = vec![1, 2, 3, 7, 11, 17, 255, 1023, 2047];
    let mut cnt = 0;
    for (p, b) in gs {
        for &u in &us {
            cnt += 1;
            if act_u(p, b, u) == (p, b) {
                return false;
            }
            if cnt >= samples {
                return true;
            }
        }
    }
    true
}

pub fn verify_orbit_counts() -> bool {
    let mut seen = HashSet::new();
    for r in 0..6 {
        let start = (8 * r, 0);
        let mut orb = HashSet::new();
        for u in 0..2048 {
            orb.insert(act_u(start.0, start.1, u));
        }
        if orb.len() != 2048 {
            return false;
        }
        if seen.intersection(&orb).next().is_some() {
            return false;
        }
        seen.extend(orb);
    }
    seen.len() == 48 * 256
}

pub fn verify_c768_closure() -> bool {
    for (p, b) in anchors_s() {
        if phi_pow(p, b, 768) != (p, b) {
            return false;
        }
    }
    true
}

pub fn verify_phi_equivariance_sample(samples: usize, debug: bool) -> bool {
    let mut gs = anchors_s();
    for r in 0..6 {
        for &j in &[5, 73, 511, 777, 1311] {
            if let Ok(g) = unpack_rb(r, j) {
                gs.push(g);
            }
        }
    }
    let us = vec![0, 1, 2, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047];
    let mut cnt = 0;
    for g in gs {
        let y = phi(g.0, g.1);
        for &u in &us {
            let lhs = act_u(y.0, y.1, u);
            let act_g = act_u(g.0, g.1, u);
            let rhs = phi(act_g.0, act_g.1);
            if lhs != rhs {
                if debug {
                    return false;
                }
            }
            cnt += 1;
            if cnt >= samples {
                return true;
            }
        }
    }
    true
}
