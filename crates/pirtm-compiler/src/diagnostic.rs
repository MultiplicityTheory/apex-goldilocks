use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiagnosticEnvelope {
    pub version: String, // e.g., "1.0.0"
    pub diagnostics: Vec<DiagnosticReport>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum DiagnosticCode {
    SUCCESSOR_PREDICATE_VIOLATION,
    MULTIPLICITY_VIOLATION,
    STRATUM_CROSS_BOUNDARY_VIOLATION,
    CIRCUIT_BUDGET_EXCEEDED,
    CONTRACTIVITY_INVARIANT_BREACH,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiagnosticReport {
    pub code: DiagnosticCode,
    pub severity: Severity,
    pub start_line: u32,
    pub start_col: u32,
    pub end_line: u32,
    pub end_col: u32,
    pub message: String,
    pub stratum_metadata: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Severity {
    ERROR,
    WARNING,
    ADVISORY,
}
