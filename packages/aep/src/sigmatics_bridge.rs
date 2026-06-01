use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WordType {
    #[serde(rename = "phase")]
    Phase,
    #[serde(rename = "generator")]
    Generator,
    #[serde(rename = "rotate")]
    Rotate,
    #[serde(rename = "twist")]
    Twist,
    #[serde(rename = "mirror")]
    Mirror,
    #[serde(rename = "pi_atom")]
    PiAtom,
    #[serde(rename = "mult")]
    Multiplicity,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SigmaticsWord {
    pub r#type: WordType,
    pub name: String,
    pub params: HashMap<String, Value>,
}

impl std::fmt::Display for SigmaticsWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.params.is_empty() {
            write!(f, "{}", self.name)
        } else {
            let mut parts: Vec<String> = self.params
                .iter()
                .map(|(k, v)| {
                    let v_str = match v {
                        Value::String(s) => s.clone(),
                        _ => v.to_string(),
                    };
                    format!("{}={}", k, v_str)
                })
                .collect();
            parts.sort();
            write!(f, "{}[{}]", self.name, parts.join(", "))
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MultiplicityWord {
    pub opcode: String,
    pub args: Vec<Value>,
    pub metadata: HashMap<String, Value>,
}

impl std::fmt::Display for MultiplicityWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args_str: Vec<String> = self.args
            .iter()
            .map(|v| match v {
                Value::String(s) => s.clone(),
                _ => v.to_string(),
            })
            .collect();
        write!(f, "{}({})", self.opcode, args_str.join(", "))
    }
}

pub fn classify_word(word_str: &str) -> SigmaticsWord {
    if word_str.starts_with("phase[") {
        let re = Regex::new(r"h₂=(\d+)").unwrap();
        let h2 = if let Some(cap) = re.captures(word_str) {
            cap.get(1)
                .map(|m| m.as_str().parse::<i32>().unwrap_or(0))
                .unwrap_or(0)
        } else {
            0
        };
        let mut params = HashMap::new();
        params.insert("h2".to_string(), Value::from(h2));
        return SigmaticsWord {
            r#type: WordType::Phase,
            name: "phase".to_string(),
            params,
        };
    }

    let generators = vec![
        "mark", "copy", "swap", "merge", "split", "quote", "evaluate",
    ];
    for gen in generators {
        if word_str.starts_with(gen) {
            let re = Regex::new(r"d=(\d+)").unwrap();
            let mut params = HashMap::new();
            if let Some(cap) = re.captures(word_str) {
                if let Some(m) = cap.get(1) {
                    if let Ok(d) = m.as_str().parse::<i32>() {
                        params.insert("d".to_string(), Value::from(d));
                    }
                }
            }
            return SigmaticsWord {
                r#type: WordType::Generator,
                name: gen.to_string(),
                params,
            };
        }
    }

    if word_str.contains("→ρ[") || word_str.contains("←ρ[") {
        let re = Regex::new(r"[→←]ρ\[(\d+)\]").unwrap();
        let q = if let Some(cap) = re.captures(word_str) {
            cap.get(1)
                .map(|m| m.as_str().parse::<i32>().unwrap_or(1))
                .unwrap_or(1)
        } else {
            1
        };
        let dir = if word_str.contains('→') { "→" } else { "←" };
        let mut params = HashMap::new();
        params.insert("q".to_string(), Value::from(q));
        params.insert("dir".to_string(), Value::from(dir));
        return SigmaticsWord {
            r#type: WordType::Rotate,
            name: "rotate".to_string(),
            params,
        };
    }

    if word_str.to_lowercase().contains("twist") {
        return SigmaticsWord {
            r#type: WordType::Twist,
            name: "twist".to_string(),
            params: HashMap::new(),
        };
    }

    if word_str.contains('~') || word_str.to_lowercase().contains("mirror") {
        return SigmaticsWord {
            r#type: WordType::Mirror,
            name: "mirror".to_string(),
            params: HashMap::new(),
        };
    }

    SigmaticsWord {
        r#type: WordType::Generator,
        name: word_str.to_string(),
        params: HashMap::new(),
    }
}

pub fn extract_pi_atoms(words: &[SigmaticsWord]) -> (Vec<SigmaticsWord>, Vec<SigmaticsWord>) {
    let mut pi_atoms = Vec::new();
    let mut other_words = Vec::new();
    for w in words {
        if w.r#type == WordType::Generator && w.params.contains_key("d") {
            pi_atoms.push(w.clone());
        } else {
            other_words.push(w.clone());
        }
    }
    (pi_atoms, other_words)
}

pub fn reorder_pi_first(words: &[SigmaticsWord]) -> Vec<SigmaticsWord> {
    let (pi_atoms, other_words) = extract_pi_atoms(words);
    let mut out = pi_atoms;
    out.extend(other_words);
    out
}

pub fn compile_word(word: &SigmaticsWord) -> MultiplicityWord {
    match word.r#type {
        WordType::Phase => {
            let h2 = word.params.get("h2").and_then(|v| v.as_i64()).unwrap_or(0);
            let mut metadata = HashMap::new();
            metadata.insert("source".to_string(), Value::from("phase"));
            MultiplicityWord {
                opcode: "SET_QUADRANT".to_string(),
                args: vec![Value::from(h2)],
                metadata,
            }
        }
        WordType::Generator => {
            let opcode = match word.name.as_str() {
                "mark" => "MARK",
                "copy" => "COPY",
                "swap" => "SWAP",
                "merge" => "MERGE",
                "split" => "SPLIT",
                "quote" => "QUOTE",
                "evaluate" => "EVALUATE",
                other => other,
            }
            .to_string();
            let mut args = Vec::new();
            let mut metadata = HashMap::new();
            if let Some(d_val) = word.params.get("d") {
                args.push(d_val.clone());
                metadata.insert("modality".to_string(), d_val.clone());
            }
            MultiplicityWord {
                opcode,
                args,
                metadata,
            }
        }
        WordType::Rotate => {
            let q = word.params.get("q").and_then(|v| v.as_i64()).unwrap_or(1);
            let dir = word
                .params
                .get("dir")
                .and_then(|v| v.as_str())
                .unwrap_or("→");
            let mut metadata = HashMap::new();
            metadata.insert("direction".to_string(), Value::from(dir));
            MultiplicityWord {
                opcode: "ROTATE".to_string(),
                args: vec![Value::from(q)],
                metadata,
            }
        }
        WordType::Twist => MultiplicityWord {
            opcode: "TWIST".to_string(),
            args: Vec::new(),
            metadata: HashMap::new(),
        },
        WordType::Mirror => MultiplicityWord {
            opcode: "MIRROR".to_string(),
            args: Vec::new(),
            metadata: HashMap::new(),
        },
        _ => MultiplicityWord {
            opcode: "OP".to_string(),
            args: vec![Value::from(word.name.clone())],
            metadata: HashMap::new(),
        },
    }
}

pub fn compile_sigmatics_to_multiplicity(
    sigmatics_words: &[String],
    apply_pi_first: bool,
) -> Vec<MultiplicityWord> {
    let mut classified: Vec<SigmaticsWord> =
        sigmatics_words.iter().map(|w| classify_word(w)).collect();
    if apply_pi_first {
        classified = reorder_pi_first(&classified);
    }
    classified.iter().map(compile_word).collect()
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompilationResult {
    pub sigmatics_words: Vec<String>,
    pub multiplicity_words: Vec<MultiplicityWord>,
    pub pi_first_applied: bool,
    pub metadata: HashMap<String, Value>,
}

impl CompilationResult {
    pub fn summary(&self) -> String {
        let mut lines = Vec::new();
        lines.push("Compilation Summary".to_string());
        lines.push("=".repeat(60));
        lines.push(format!(
            "Input: {} Sigmatics words",
            self.sigmatics_words.len()
        ));
        lines.push(format!(
            "Output: {} Multiplicity words",
            self.multiplicity_words.len()
        ));
        lines.push(format!(
            "Π-first reduction: {}",
            if self.pi_first_applied {
                "Applied"
            } else {
                "Not applied"
            }
        ));
        lines.push(String::new());
        lines.push("Sigmatics words:".to_string());
        for w in &self.sigmatics_words {
            lines.push(format!("  {}", w));
        }
        lines.push(String::new());
        lines.push("Multiplicity words:".to_string());
        for w in &self.multiplicity_words {
            lines.push(format!("  {}", w));
        }
        lines.push("=".repeat(60));
        lines.join("\n")
    }
}

pub fn bridge_compile(
    sigmatics_words: &[String],
    apply_pi_first: bool,
    metadata: Option<HashMap<String, Value>>,
) -> CompilationResult {
    let multiplicity_words = compile_sigmatics_to_multiplicity(sigmatics_words, apply_pi_first);
    CompilationResult {
        sigmatics_words: sigmatics_words.to_vec(),
        multiplicity_words,
        pi_first_applied: apply_pi_first,
        metadata: metadata.unwrap_or_default(),
    }
}
