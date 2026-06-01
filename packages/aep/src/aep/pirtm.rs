use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormTick {
    pub v_next: Vec<i64>,
    pub norm_hat_Q: i64,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeumannEstimate {
    pub xN_Q: Vec<i64>,
    pub tail_bound_Q: i64,
    pub n: i32, // Rename N to n to avoid uppercase warnings
    pub norm_hat_Q: i64,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margins {
    pub gap_lb_Q: i64,
    pub tighten_tau: bool,
    pub threshold_Q: i64,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorReport {
    pub norm_hat_Q: i64,
    pub gap_lb_Q: i64,
    pub tail_bound_Q: i64,
    pub xN_Q: Vec<i64>,
    pub tighten_tau: bool,
    pub sig_ok: bool,
    pub v_next: Vec<i64>,
}

fn ceil_div(a: i128, b: i128) -> i64 {
    if b <= 0 {
        panic!("b must be > 0");
    }
    ((a + b - 1) / b) as i64
}

fn l1_norm(v: &[i64]) -> i64 {
    v.iter().map(|x| x.abs()).sum()
}

pub fn power_iter_tick<F>(k_fn: F, v: &[i64], q: i64) -> NormTick
where
    F: Fn(&[i64]) -> Vec<i64>,
{
    let kv = k_fn(v);
    let num = l1_norm(&kv);
    let den = l1_norm(v).max(1);

    // Compute (num * q) / den with rounding up
    let ratio_q = ceil_div((num as i128) * (q as i128), den as i128);
    let div = ratio_q.max(1);

    let mut v_next: Vec<i64> = kv.iter().map(|&x| x / div).collect();
    if v_next.iter().all(|&x| x == 0) {
        v_next = vec![0; kv.len()];
        if !v_next.is_empty() {
            v_next[0] = 1;
        }
    }

    NormTick {
        v_next,
        norm_hat_Q: ratio_q,
    }
}

fn ipow(base: i128, exp: i32) -> i128 {
    let mut r = 1i128;
    let mut b = base;
    let mut e = exp;
    while e > 0 {
        if e & 1 != 0 {
            r = r.saturating_mul(b);
        }
        b = b.saturating_mul(b);
        e >>= 1;
    }
    r
}

pub fn neumann_estimate<F>(
    k_fn: F,
    f_q: &[i64],
    n: i32,
    q: i64,
    norm_hat_q: i64,
) -> NeumannEstimate
where
    F: Fn(&[i64]) -> Vec<i64>,
{
    if n < 0 {
        panic!("n must be >= 0");
    }
    if !(0..q).contains(&norm_hat_q) {
        panic!("norm_hat_q must be in [0, q)");
    }

    let mut x = vec![0i64; f_q.len()];
    let mut term = f_q.to_vec();

    for _ in 0..=n {
        for (i, &val) in term.iter().enumerate() {
            x[i] = x[i].saturating_add(val);
        }
        term = k_fn(&term);
    }

    let f_norm_q = l1_norm(f_q);

    // tail = ||F|| * norm_hat^{n+1} / ((q - norm_hat) * q^n)
    let k_pow = ipow(norm_hat_q as i128, n + 1);
    let num = (f_norm_q as i128).saturating_mul(k_pow);

    let q_pow_n = ipow(q as i128, n);
    let den = ((q - norm_hat_q) as i128).saturating_mul(q_pow_n);

    let tail_q = ceil_div(num, den);

    NeumannEstimate {
        xN_Q: x,
        tail_bound_Q: tail_q,
        n,
        norm_hat_Q: norm_hat_q,
    }
}

pub fn eval_margins(norm_hat_q: i64, q: i64, threshold_q: i64) -> Margins {
    if !(0..q).contains(&threshold_q) {
        panic!("threshold_q must be in [0, q)");
    }
    let gap = q - norm_hat_q;
    Margins {
        gap_lb_Q: gap,
        tighten_tau: gap < threshold_q,
        threshold_Q: threshold_q,
    }
}

pub fn infinite_prime_convergence_ok(
    sig_history: &[HashMap<i64, i64>],
    window: usize,
) -> bool {
    if sig_history.len() < 2 {
        return true;
    }
    let h_len = window.min(sig_history.len());
    let h = &sig_history[sig_history.len() - h_len..];

    let mut primes = HashSet::new();
    for s in h {
        for &p in s.keys() {
            primes.insert(p);
        }
    }

    for p in primes {
        let seq: Vec<i64> = h.iter().map(|s| *s.get(&p).unwrap_or(&0)).collect();
        if seq.len() >= 2 {
            let last = seq[seq.len() - 1];
            let prev_max = seq[0..seq.len() - 1].iter().copied().max().unwrap_or(0);
            if last > prev_max {
                return false;
            }
        }
    }

    true
}

pub fn monitor_step<F>(
    k_fn: F,
    f_q: &[i64],
    v_power: &[i64],
    q: i64,
    n: i32,
    threshold_q: i64,
    sig_history: &[HashMap<i64, i64>],
) -> MonitorReport
where
    F: Fn(&[i64]) -> Vec<i64>,
{
    let tick = power_iter_tick(&k_fn, v_power, q);
    let norm_hat_q = tick.norm_hat_Q;

    if norm_hat_q >= q {
        let tail = l1_norm(f_q);
        let margins = Margins {
            gap_lb_Q: 0,
            tighten_tau: true,
            threshold_Q: threshold_q,
        };
        let sig_ok = infinite_prime_convergence_ok(sig_history, 4);
        return MonitorReport {
            norm_hat_Q: norm_hat_q,
            gap_lb_Q: margins.gap_lb_Q,
            tail_bound_Q: tail,
            xN_Q: f_q.to_vec(),
            tighten_tau: margins.tighten_tau,
            sig_ok,
            v_next: tick.v_next,
        };
    }

    let ne = neumann_estimate(&k_fn, f_q, n, q, norm_hat_q);
    let margins = eval_margins(norm_hat_q, q, threshold_q);
    let sig_ok = infinite_prime_convergence_ok(sig_history, 4);

    MonitorReport {
        norm_hat_Q: norm_hat_q,
        gap_lb_Q: margins.gap_lb_Q,
        tail_bound_Q: ne.tail_bound_Q,
        xN_Q: ne.xN_Q,
        tighten_tau: margins.tighten_tau,
        sig_ok,
        v_next: tick.v_next,
    }
}
