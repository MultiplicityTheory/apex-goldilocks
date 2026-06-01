use crate::validator::AdmissibilityValidator;
use crate::diagnostic::{DiagnosticEnvelope};
use std::collections::HashSet;
use tree_sitter::{Parser, Language};

unsafe extern "C" {
    fn tree_sitter_pirtm() -> Language;
}

pub fn validate_source(source: &str, prime_set_json: &str, stratum_id: &str) -> Result<DiagnosticEnvelope, String> {
    let prime_set: HashSet<u64> = serde_json::from_str(prime_set_json)
        .map_err(|e| e.to_string())?;
    
    let mut validator = AdmissibilityValidator::new(prime_set, stratum_id.to_string());
    
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_pirtm() };
    parser.set_language(&language).map_err(|e| e.to_string())?;
    
    let tree = parser.parse(source, None)
        .ok_or_else(|| "Failed to parse source".to_string())?;

    traverse_ast(tree.root_node(), source, &mut validator);
    
    Ok(DiagnosticEnvelope {
        version: "1.0.0".to_string(),
        diagnostics: validator.diagnostics,
    })
}

fn traverse_ast(node: tree_sitter::Node, source: &str, validator: &mut AdmissibilityValidator) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "prime_indexed_op" {
            // Extract prime index and call validator
            // This assumes structural consistency with grammar.js
            let prime_node = child.child_by_field_name("prime_index").unwrap();
            let prime_str = &source[prime_node.byte_range()];
            if let Ok(p) = prime_str.parse::<u64>() {
                // Dummy signature for now
                validator.validate_op(p, crate::types::Sig::new(), child.start_position().row as u32, child.start_position().column as u32);
            }
        }
        traverse_ast(child, source, validator);
    }
}
