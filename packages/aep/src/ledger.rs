use crate::petc::to_canonical_json;
use blake2::Blake2bVar;
use blake2::digest::{Update, VariableOutput};
use serde_json::Value;

pub fn blake2b_32_hex(data: &[u8]) -> String {
    let mut hasher = Blake2bVar::new(32).unwrap();
    hasher.update(data);
    let mut buf = [0u8; 32];
    hasher.finalize_variable(&mut buf).unwrap();
    let mut hex_str = String::with_capacity(64);
    for byte in &buf {
        use std::fmt::Write;
        write!(&mut hex_str, "{:02x}", byte).unwrap();
    }
    hex_str
}

pub fn now_epoch_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

#[derive(Debug, Clone, Default)]
pub struct Ledger {
    entries: Vec<Value>,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn append(&mut self, entry_val: Value) -> String {
        let mut entry = entry_val;
        if let Some(obj) = entry.as_object_mut() {
            obj.insert("entry_ms".to_string(), Value::from(now_epoch_ms()));
        }

        let canon = to_canonical_json(&entry);
        let entry_id = blake2b_32_hex(canon.as_bytes());

        if let Some(obj) = entry.as_object_mut() {
            obj.insert("entry_id".to_string(), Value::from(entry_id.clone()));
        }

        self.entries.push(entry);
        entry_id
    }

    pub fn broadcast(&self, _entry_id: &str) {
        // No-op stub. Wire to your messaging layer.
    }

    pub fn entries(&self) -> Vec<Value> {
        self.entries.clone()
    }
}
