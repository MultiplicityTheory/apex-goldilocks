use crate::types::Sig;
use crate::diagnostic::{DiagnosticCode, DiagnosticReport, Severity};
use std::collections::HashSet;

pub struct AdmissibilityValidator {
    pub current_prime: Option<u64>,
    pub prime_set: HashSet<u64>,
    pub sorted_primes: Vec<u64>,
    pub diagnostics: Vec<DiagnosticReport>,
    pub stratum_id: String,
}

impl AdmissibilityValidator {
    pub fn new(prime_set: HashSet<u64>, stratum_id: String) -> Self {
        let mut sorted_primes: Vec<u64> = prime_set.iter().cloned().collect();
        sorted_primes.sort_unstable();
        
        AdmissibilityValidator {
            current_prime: None,
            prime_set,
            sorted_primes,
            diagnostics: Vec::new(),
            stratum_id,
        }
    }

    pub fn validate_op(&mut self, p_next: u64, _op_sig: Sig, line: u32, col: u32) {
        // 1. Stratum Boundary Check
        if !self.prime_set.contains(&p_next) {
            self.add_diagnostic(DiagnosticCode::STRATUM_CROSS_BOUNDARY_VIOLATION, line, col, format!("Prime {} outside stratum {}", p_next, self.stratum_id));
            return;
        }

        // 2. Successor Predicate Check
        if let Some(p_curr) = self.current_prime {
            if let Ok(expected_p) = self.next_prime(p_curr) {
                if p_next != expected_p {
                    self.add_diagnostic(DiagnosticCode::SUCCESSOR_PREDICATE_VIOLATION, line, col, format!("Expected successor {}, got {}", expected_p, p_next));
                }
            } else {
                // If p_curr has no successor, this operator is invalid in this tower
                self.add_diagnostic(DiagnosticCode::SUCCESSOR_PREDICATE_VIOLATION, line, col, format!("Prime {} has no valid successor in this stratum", p_curr));
            }
        }
        
        self.current_prime = Some(p_next);
    }

    fn next_prime(&self, p: u64) -> Result<u64, ()> {
        let idx = self.sorted_primes.iter().position(|&x| x == p).ok_or(())?;
        if idx + 1 < self.sorted_primes.len() {
            Ok(self.sorted_primes[idx + 1])
        } else {
            Err(())
        }
    }

    fn add_diagnostic(&mut self, code: DiagnosticCode, line: u32, col: u32, message: String) {
        self.diagnostics.push(DiagnosticReport {
            code,
            severity: Severity::ERROR,
            start_line: line,
            start_col: col,
            end_line: line,
            end_col: col + 5, // Simplified range
            message,
            stratum_metadata: Some(self.stratum_id.clone()),
        });
    }
}
