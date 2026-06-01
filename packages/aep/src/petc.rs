use serde_json::Value;
use sha2::{Digest as ShaDigest, Sha256};

pub fn to_canonical_json(val: &Value) -> String {
    match val {
        Value::Null => "null".to_string(),
        Value::Bool(b) => {
            if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        Value::Number(n) => n.to_string(),
        Value::String(s) => serde_json::to_string(s).unwrap(),
        Value::Array(arr) => {
            let parts: Vec<String> = arr.iter().map(to_canonical_json).collect();
            format!("[{}]", parts.join(","))
        }
        Value::Object(obj) => {
            let mut keys: Vec<&String> = obj.keys().collect();
            keys.sort();
            let parts: Vec<String> = keys
                .iter()
                .map(|&k| {
                    let v = &obj[k];
                    format!(
                        "{}:{}",
                        serde_json::to_string(k).unwrap(),
                        to_canonical_json(v)
                    )
                })
                .collect();
            format!("{{{}}}", parts.join(","))
        }
    }
}

pub fn sha256_hex(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LedgerRow {
    pub op: String,
    pub payload_digest: String,
    pub prev_digest: String,
    pub timestamp: i64,
    pub digest: String,
}

#[derive(Debug, Clone, Default)]
pub struct PETCLedger {
    pub rows: Vec<LedgerRow>,
}

impl PETCLedger {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn append(&mut self, op: String, payload: Value, ts: Option<i64>) -> LedgerRow {
        let body = serde_json::json!({
            "op": op,
            "payload": payload
        });
        let pdig = sha256_hex(&to_canonical_json(&body));
        let prev = if let Some(last) = self.rows.last() {
            last.digest.clone()
        } else {
            "0".repeat(64)
        };
        let t = ts.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64
        });
        let digest_input = serde_json::json!({
            "pdig": pdig,
            "prev": prev,
            "t": t
        });
        let digest = sha256_hex(&to_canonical_json(&digest_input));

        let row = LedgerRow {
            op,
            payload_digest: pdig,
            prev_digest: prev,
            timestamp: t,
            digest,
        };
        self.rows.push(row.clone());
        row
    }

    pub fn verify(&self) -> bool {
        let mut prev = "0".repeat(64);
        for r in &self.rows {
            let digest_input = serde_json::json!({
                "pdig": r.payload_digest,
                "prev": prev,
                "t": r.timestamp
            });
            let recomputed = sha256_hex(&to_canonical_json(&digest_input));
            if recomputed != r.digest {
                return false;
            }
            prev = r.digest.clone();
        }
        true
    }
}
