use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budgets {
    pub b: Vec<i64>,
    pub a: Vec<i64>,
    pub limit1_Q: i64,
    pub limit2_Q: i64,
    pub q: i64, // Keep as 'q' in Rust (serialized as 'Q' using serde attribute or we can rename to match Python serialized field if needed. In python attributes are limit1_Q, limit2_Q, Q)
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjResult {
    pub w_star_Q: Vec<i64>,
    pub lam_Q: i64,
    pub mu_Q: i64,
    pub sum1: i64,
    pub sum2: i64,
    pub iters: i64,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AceMetrics {
    pub slope_scaled: i64,
    pub gap_scaled: i64,
    pub sum_abs_w_Q: i64,
    pub sum_norms_Q: i64,
    pub rho_hat_scaled_opt: Option<i64>,
}

pub fn weighted_l1(weights: &[i64], coeffs: &[i64]) -> i64 {
    let mut total = 0i64;
    for (&w, &c) in weights.iter().zip(coeffs.iter()) {
        total = total.saturating_add(c.abs().saturating_mul(w.abs()));
    }
    total
}

/// Return the smaller fraction when both are positive.
/// Fractions are encoded as (num, den) with den > 0.
pub fn min_fraction(current: (i64, i64), candidate: (i64, i64)) -> (i64, i64) {
    let (cur_num, cur_den) = current;
    let (cand_num, cand_den) = candidate;
    if cand_den <= 0 {
        return current;
    }
    if cur_den <= 0 {
        return candidate;
    }
    // Compare cur_num/cur_den and cand_num/cand_den:
    // cand_num * cur_den < cur_num * cand_den
    // To prevent overflow in multiplication, we cast to i128
    let lhs = (cand_num as i128) * (cur_den as i128);
    let rhs = (cur_num as i128) * (cand_den as i128);
    if lhs < rhs {
        candidate
    } else {
        current
    }
}

pub fn project_dual_int(wtilde_Q: &[i64], budgets: &Budgets) -> ProjResult {
    let w_vec: Vec<i64> = wtilde_Q.to_vec();
    let mut b = budgets.b.clone();
    let mut a = budgets.a.clone();

    if b.len() < w_vec.len() {
        b.resize(w_vec.len(), 0);
    }
    if a.len() < w_vec.len() {
        a.resize(w_vec.len(), 0);
    }

    let total1 = weighted_l1(&w_vec, &b);
    let total2 = weighted_l1(&w_vec, &a);

    let mut scale = (1i64, 1i64); // numerator, denominator
    if total1 > budgets.limit1_Q && total1 > 0 {
        scale = min_fraction(scale, (budgets.limit1_Q, total1));
    }
    if total2 > budgets.limit2_Q && total2 > 0 {
        scale = min_fraction(scale, (budgets.limit2_Q, total2));
    }

    let (mut num, mut den) = scale;
    if den <= 0 {
        den = 1;
    }
    if num <= 0 {
        num = 0;
    }

    let w_star = if num < den {
        w_vec.iter().map(|&v| (v * num) / den).collect()
    } else {
        w_vec.clone()
    };

    let sum1 = weighted_l1(&w_star, &b);
    let sum2 = weighted_l1(&w_star, &a);
    let lam_Q = (total1 - budgets.limit1_Q).max(0);
    let mu_Q = (total2 - budgets.limit2_Q).max(0);

    ProjResult {
        w_star_Q: w_star,
        sum1,
        sum2,
        lam_Q,
        mu_Q,
        iters: 0,
    }
}

pub fn slope_bounds_scaled(
    w_star_Q: &[i64],
    b_norms_Q: &[i64],
    q: i64,
) -> (i64, i64, AceMetrics) {
    let mut slope = 0i64;
    let mut sum_abs_w = 0i64;
    let mut sum_norms = 0i64;

    for (&w, &n) in w_star_Q.iter().zip(b_norms_Q.iter()) {
        let aw = w.abs();
        let an = n.abs();
        slope = slope.saturating_add(aw.saturating_mul(an));
        sum_abs_w = sum_abs_w.saturating_add(aw);
        sum_norms = sum_norms.saturating_add(an);
    }

    let mut gap = q.saturating_mul(q) - slope;
    if gap < 0 {
        gap = 0;
    }

    let metrics = AceMetrics {
        slope_scaled: slope,
        gap_scaled: gap,
        sum_abs_w_Q: sum_abs_w,
        sum_norms_Q: sum_norms,
        rho_hat_scaled_opt: None,
    };

    (slope, gap, metrics)
}

pub fn ace_accept(
    w_star_Q: &[i64],
    b_norms_Q: &[i64],
    q: i64,
    rho_hat_scaled_opt: Option<i64>,
) -> (bool, AceMetrics) {
    let (slope, _gap, mut metrics) = slope_bounds_scaled(w_star_Q, b_norms_Q, q);
    let sum_abs = metrics.sum_abs_w_Q;
    
    let mut ok = if sum_abs == 0 {
        true
    } else {
        slope < q.saturating_mul(sum_abs)
    };

    if let Some(rho_hat) = rho_hat_scaled_opt {
        ok = ok && (slope <= rho_hat);
    }

    metrics.rho_hat_scaled_opt = rho_hat_scaled_opt;
    (ok, metrics)
}
