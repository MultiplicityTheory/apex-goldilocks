use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub proof_id: String,
    pub claim: String,
}

pub struct ProofManager;

impl Default for ProofManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ProofManager {
    pub fn new() -> Self { Self }
    pub fn generate(&self, claim: String, _payload: Value) -> Proof {
        Proof {
            proof_id: "0".repeat(64),
            claim,
        }
    }
}