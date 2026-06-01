use crate::ledger::Ledger;
use crate::proofs::ProofManager;
use serde_json::Value;

pub fn matmul(m: &[Vec<i64>], v: &[i64], q: i64) -> Vec<i64> {
    let mut result = Vec::with_capacity(m.len());
    for row in m {
        let mut sum = 0i128;
        for (&a, &b) in row.iter().zip(v.iter()) {
            sum += (a as i128) * (b as i128);
        }
        result.push((sum / q as i128) as i64);
    }
    result
}

pub fn subtract(a: &[i64], b: &[i64]) -> Vec<i64> {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).map(|(&x, &y)| x - y).collect()
}

pub fn allclose(a: &[i64], b: &[i64], atol_q: i64) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).all(|(&x, &y)| (x - y).abs() <= atol_q)
}

pub fn lstsq(a: &[Vec<i64>], b: &[i64], q: i64, atol_q: i64) -> (Vec<i64>, Vec<i64>) {
    let m = a.len();
    let n = if a.is_empty() { 0 } else { a[0].len() };

    if m == n {
        let mut x = Vec::with_capacity(n);
        for i in 0..n {
            if a[i][i].abs() > atol_q {
                // (b[i] * q) / a[i][i]
                let val = ((b[i] as i128 * q as i128) / a[i][i] as i128) as i64;
                x.push(val);
            } else {
                x.push(0);
            }
        }
        let ax = matmul(a, &x, q);
        let residual = subtract(&ax, b);
        (x, residual)
    } else {
        let x = vec![0; n];
        let residual = b.to_vec();
        (x, residual)
    }
}

#[derive(Debug, Clone)]
pub struct StateStore {
    current: Vec<i64>,
    history: Vec<Vec<i64>>,
}

impl StateStore {
    pub fn new(initial_state: Vec<i64>) -> Self {
        Self {
            current: initial_state.clone(),
            history: vec![initial_state],
        }
    }

    pub fn current(&self) -> Vec<i64> {
        self.current.clone()
    }

    pub fn update(&mut self, new_state: Vec<i64>) {
        self.history.push(new_state.clone());
        self.current = new_state;
    }

    pub fn rollback(&mut self) {
        if self.history.len() > 1 {
            self.history.pop();
            self.current = self.history.last().unwrap().clone();
        }
    }
}

pub type SigmaFn = Box<dyn Fn(&str, i64) -> i32 + Send + Sync>;

pub struct Watchdog {
    pub sigma: SigmaFn,
    pub ledger: Ledger,
    pub proofs: ProofManager,
    pub q: i64,
    pub atol_q: i64,
    pub locked: bool,
}

impl Watchdog {
    pub fn new(sigma: SigmaFn, ledger: Ledger, proofs: ProofManager, q: i64, atol_q: i64) -> Self {
        Self {
            sigma,
            ledger,
            proofs,
            q,
            atol_q,
            locked: false,
        }
    }

    pub fn lockdown(&mut self, val: bool) {
        self.locked = val;
    }

    pub fn enforce(
        &mut self,
        store: &mut StateStore,
        next_state: Vec<i64>,
        actor_id: &str,
        m: &[Vec<i64>],
        e_alpha: &[Vec<i64>],
        now_ms: Option<i64>,
    ) -> Result<Value, String> {
        let t_ms = now_ms.unwrap_or_else(crate::ledger::now_epoch_ms);
        let x_prev = store.current();
        let x_next = next_state.clone();

        // 1. Sovereignty check
        let sigma_val = (self.sigma)(actor_id, t_ms);
        if sigma_val != 1 {
            let reason = format!(
                "Sovereignty violation: actor={}, sigma={}, t_ms={}",
                actor_id, sigma_val, t_ms
            );
            self.handle_violation(store, &reason, actor_id, t_ms);
            return Err(reason);
        }

        // 2. Ethics check
        let predicted = matmul(m, &x_prev, self.q);
        let residual = subtract(&x_next, &predicted);

        let mut check_failed = false;
        let mut fail_reason = String::new();

        let (u, _) = lstsq(e_alpha, &residual, self.q, self.atol_q);
        let e_u = matmul(e_alpha, &u, self.q);
        let reconstructed: Vec<i64> = predicted
            .iter()
            .zip(e_u.iter())
            .map(|(&p, &e)| p.saturating_add(e))
            .collect();
        if !allclose(&x_next, &reconstructed, self.atol_q) {
            check_failed = true;
            fail_reason = format!(
                "Ethics violation: state transition not valid for actor={}",
                actor_id
            );
        }

        if check_failed {
            self.handle_violation(store, &fail_reason, actor_id, t_ms);
            return Err(fail_reason);
        }

        // 3. Generate proofs
        let payload = serde_json::json!({
            "actor_id": actor_id,
            "t_ms": t_ms,
            "x_prev": x_prev,
            "x_next": x_next
        });
        let sov_proof = self
            .proofs
            .generate("sovereignty_gate".to_string(), payload.clone());
        let eth_proof = self
            .proofs
            .generate("ethics_commutation".to_string(), payload);

        // 4. Commit to store
        store.update(x_next.clone());

        // 5. Append to ledger
        let entry = serde_json::json!({
            "actor_id": actor_id,
            "t_ms": t_ms,
            "x_prev": x_prev,
            "x_next": x_next,
            "M": m,
            "E_alpha": e_alpha,
            "proofs": [sov_proof.proof_id, eth_proof.proof_id]
        });
        let entry_id = self.ledger.append(entry.clone());

        // 6. Broadcast
        self.ledger.broadcast(&entry_id);

        Ok(entry)
    }

    fn handle_violation(&mut self, store: &mut StateStore, reason: &str, actor_id: &str, t_ms: i64) {
        // 1. Lockdown
        self.lockdown(true);

        // 2. Rollback
        store.rollback();

        // 3. Broadcast violation
        let violation_entry = serde_json::json!({
            "type": "violation",
            "reason": reason,
            "actor_id": actor_id,
            "t_ms": t_ms
        });
        let entry_id = self.ledger.append(violation_entry);
        self.ledger.broadcast(&entry_id);
    }
}

pub struct CommitWrapper {
    pub store: StateStore,
    pub watchdog: Watchdog,
}

impl CommitWrapper {
    pub fn new(store: StateStore, watchdog: Watchdog) -> Self {
        Self { store, watchdog }
    }

    pub fn commit(
        &mut self,
        next_state: Vec<i64>,
        actor_id: &str,
        m: &[Vec<i64>],
        e_alpha: &[Vec<i64>],
        now_ms: Option<i64>,
    ) -> Result<Value, String> {
        self.watchdog.enforce(
            &mut self.store,
            next_state,
            actor_id,
            m,
            e_alpha,
            now_ms,
        )
    }
}
