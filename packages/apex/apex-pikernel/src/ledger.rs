use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LedgerRecord {
    pub step: usize,
    pub pi: Vec<usize>,
    pub alpha: String, // Represented as string for exact rational serialization
    pub tau: String,
    pub lambda_soft: String,
    pub l1_weight_sum: String,
    pub change_norm: String,
}

pub struct Ledger {
    pub path: String,
}

impl Ledger {
    pub fn new(path: &str) -> Self {
        Ledger { path: path.to_string() }
    }

    pub fn append(&self, record: LedgerRecord) {
        let json = serde_json::to_string(&record).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .unwrap();
        writeln!(file, "{}", json).unwrap();
    }
}
