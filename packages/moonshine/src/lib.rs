use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkSketch {
    pub proof_id: String,
    pub valid: bool,
}

pub fn zk_sketch(_seed: &str) -> ZkSketch {
    ZkSketch {
        proof_id: "0".repeat(64),
        valid: true,
    }
}