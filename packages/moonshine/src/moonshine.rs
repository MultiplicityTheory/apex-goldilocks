use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::{HashMap, BTreeMap};
use sha2::{Digest as ShaDigest, Sha256};
use apex_aep::aep::petc::{eq_signatures, PETCChannelLedger};
use apex_aep::petc::{to_canonical_json};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorDef {
    pub id: String,
    pub in_sig: HashMap<i64, i64>,
    pub out_sig: HashMap<i64, i64>,
    pub norm_Q: i64,
    pub commutator_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoonshineConfig {
    pub q: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmallGainDecision {
    pub ok: bool,
    pub product_scaled: i64,
    pub threshold: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackAcceptance {
    pub typed_ok: bool,
    pub small_gain: SmallGainDecision,
    pub quarantine: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZKSketch {
    pub ledger_digest: String,
    pub ace_digest: String,
    pub challenge_hex: String,
    pub checks: HashMap<String, i64>,
}

pub fn check_typing(g1: &OperatorDef, g2: &OperatorDef) -> bool {
    eq_signatures(&g1.out_sig, &g2.in_sig)
}

pub fn decrement_budgets_for_feedback(
    ledger: &mut PETCChannelLedger,
    g1: &OperatorDef,
    g2: &OperatorDef,
) -> (HashMap<String, i64>, bool) {
    let mut remaining = HashMap::new();
    let mut quarantine = false;

    for cls in &[&g1.commutator_class, &g2.commutator_class] {
        match ledger.decrement_commutator(cls, 1) {
            Ok(rem) => {
                remaining.insert((*cls).clone(), rem);
            }
            Err(_) => {
                quarantine = true;
            }
        }
    }

    for (cls, &rem) in &ledger.commutator_budget {
        if !remaining.contains_key(cls) {
            remaining.insert(cls.clone(), rem);
        }
    }

    (remaining, quarantine)
}

pub fn small_gain_accept(g1_norm_q: i64, g2_norm_q: i64, q: i64) -> SmallGainDecision {
    let prod = (g1_norm_q as i128) * (g2_norm_q as i128);
    let thr = (q as i128) * (q as i128);
    SmallGainDecision {
        ok: prod < thr,
        product_scaled: prod as i64,
        threshold: thr as i64,
    }
}

fn sha256_hex(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn pow_mod(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    let mut res = 1i128;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }
    res
}

pub fn zk_sketch(
    ledger: &PETCChannelLedger,
    ace_metrics: &[HashMap<String, i64>],
    seed_hex: &str,
) -> ZKSketch {
    // 1. Sort commutator_budget by key
    let mut bud = BTreeMap::new();
    for (k, &v) in &ledger.commutator_budget {
        bud.insert(k.clone(), v);
    }
    let bud_val = serde_json::to_value(&bud).unwrap();
    let ledger_payload = serde_json::json!({
        "commutator_budget": bud_val
    });
    let ledger_digest = sha256_hex(&to_canonical_json(&ledger_payload));

    // 2. Format and sort ace metrics
    let mut ace_sorted = Vec::new();
    for m in ace_metrics {
        let mut sorted_m = BTreeMap::new();
        for (k, &v) in m {
            sorted_m.insert(k.clone(), v);
        }
        ace_sorted.push(sorted_m);
    }
    let ace_val = serde_json::to_value(&ace_sorted).unwrap();
    let ace_payload = serde_json::json!({
        "ace": ace_val
    });
    let ace_digest = sha256_hex(&to_canonical_json(&ace_payload));

    // 3. Challenge
    let mut hasher = Sha256::new();
    hasher.update(seed_hex.as_bytes());
    hasher.update(ledger_digest.as_bytes());
    hasher.update(ace_digest.as_bytes());
    let chal = format!("{:x}", hasher.finalize());

    // 4. Checks modulo MOD = 2^61 - 1
    let modulus = (1i128 << 61) - 1;
    let mut r = i128::from_str_radix(&chal, 16).unwrap_or(0) % (modulus - 1);
    if r == 0 {
        r = 1;
    }

    let mut c1 = 0i128;
    let mut i = 0i32;
    for &v in bud.values() {
        i += 1;
        let term = (v as i128 * pow_mod(r, i as i128, modulus)) % modulus;
        c1 = (c1 + term) % modulus;
    }

    let mut c2 = 0i128;
    let mut j = 0i32;
    for m in &ace_sorted {
        j += 1;
        let slope = *m.get("slope_scaled").unwrap_or(&0) as i128;
        let gap = *m.get("gap_scaled").unwrap_or(&0) as i128;

        let term1 = (slope * pow_mod(r, (2 * j) as i128, modulus)) % modulus;
        let term2 = (gap * pow_mod(r, (2 * j + 1) as i128, modulus)) % modulus;

        c2 = (c2 + term1 + term2) % modulus;
    }

    let mut checks = HashMap::new();
    checks.insert("c1".to_string(), c1 as i64);
    checks.insert("c2".to_string(), c2 as i64);
    checks.insert("mod".to_string(), modulus as i64);
    checks.insert("r".to_string(), r as i64);

    ZKSketch {
        ledger_digest,
        ace_digest,
        challenge_hex: chal,
        checks,
    }
}

pub fn accept_feedback_interconnection(
    g1: &OperatorDef,
    g2: &OperatorDef,
    cfg: &MoonshineConfig,
    ledger: &mut PETCChannelLedger,
    ace_metrics: &[HashMap<String, i64>],
    seed_hex: &str,
) -> (FeedbackAcceptance, ZKSketch, HashMap<String, i64>) {
    let typed_ok = check_typing(g1, g2);
    let sg = small_gain_accept(g1.norm_Q, g2.norm_Q, cfg.q);
    let (remaining, quarantine) = decrement_budgets_for_feedback(ledger, g1, g2);
    let sketch = zk_sketch(ledger, ace_metrics, seed_hex);
    (
        FeedbackAcceptance {
            typed_ok,
            small_gain: sg,
            quarantine,
        },
        sketch,
        remaining,
    )
}
