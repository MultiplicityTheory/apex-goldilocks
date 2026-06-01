use crate::ace::{ACEParams, ACEProjector};
use crate::lanes::MultiClassLaneStore;
use crate::petc::PETCLedger;
use crate::quantizer::Z96Quantizer;
use crate::toggles::ToggleStreams;
use std::collections::HashMap;
use apex_core::arithmetic::Rational;

#[derive(Debug, Clone)]
pub struct MultiplicityRuntime {
    pub active_class: String,
    pub quantizer: Z96Quantizer,
    pub lanes: MultiClassLaneStore,
    pub toggles: ToggleStreams,
    pub ledger: PETCLedger,
}

impl MultiplicityRuntime {
    pub fn new(active_class: String) -> Self {
        let lanes = MultiClassLaneStore::new(active_class.clone());
        Self {
            active_class,
            quantizer: Z96Quantizer::default(),
            lanes,
            toggles: ToggleStreams::new(),
            ledger: PETCLedger::new(),
        }
    }

    pub fn switch_class(&mut self, new_class: String) {
        self.lanes.switch_class(new_class.clone());
        self.active_class = new_class.clone();

        let payload = serde_json::json!({
            "new_class": new_class
        });
        self.ledger.append("switch_class".to_string(), payload, None);
    }

    pub fn ace_project(
        &mut self,
        w_hat: &HashMap<i32, Rational>,
        bounds_b: &HashMap<i32, Rational>,
        params: &ACEParams,
    ) -> Result<HashMap<i32, Rational>, String> {
        let radius = params.radius();
        let (w, gap_lb) = ACEProjector::project_weighted_l1(w_hat, bounds_b, radius)?;

        let proposal_val = serde_json::to_value(w_hat).unwrap();
        let bounds_val = serde_json::to_value(bounds_b).unwrap();
        let accepted_val = serde_json::to_value(&w).unwrap();

        let payload = serde_json::json!({
            "proposal": proposal_val,
            "bounds_b": bounds_val,
            "R_t": radius,
            "accepted": accepted_val,
            "gapLB": gap_lb
        });
        self.ledger.append("ace_project".to_string(), payload, None);
        Ok(w)
    }

    pub fn ingest(&mut self, t: i32, real_inputs: &HashMap<i32, Rational>) -> HashMap<i32, i32> {
        let mut out = HashMap::new();
        for (&n, &x) in real_inputs {
            if self.toggles.is_on(n, t) {
                let z = self.quantizer.quantize(x);
                self.lanes.write(n, z as i64, None);
                out.insert(n, z);
            }
        }
        if !out.is_empty() {
            let lanes_val = serde_json::to_value(&out).unwrap();
            let payload = serde_json::json!({
                "t": t,
                "class": self.active_class.clone(),
                "lanes": lanes_val
            });
            self.ledger.append("ingest".to_string(), payload, None);
        }
        out
    }

    pub fn read_lanes(&mut self, indices: &[i32]) -> HashMap<i32, i32> {
        indices
            .iter()
            .map(|&n| {
                let val = self.lanes.read(n, None, 0);
                (n, val as i32)
            })
            .collect()
    }

    pub fn last_digest(&self) -> String {
        if let Some(last) = self.ledger.rows.last() {
            last.digest.clone()
        } else {
            "0".repeat(64)
        }
    }

    pub fn verify_ledger(&self) -> bool {
        self.ledger.verify()
    }
}
