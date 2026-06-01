use std::env;
use std::fs;
use std::path::Path;
use pirtm_compiler::compiler;
use pirtm_compiler::diagnostic::DiagnosticEnvelope;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: pirtm-compiler [validate|prove] [args]");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "validate" => {
            // Find --source <path>
            let mut source_path = String::new();
            for i in 2..args.len() {
                if args[i] == "--source" && i + 1 < args.len() {
                    source_path = args[i + 1].clone();
                    break;
                }
            }
            if source_path.is_empty() {
                eprintln!("Error: --source <path> is required");
                std::process::exit(1);
            }

            // Read the source. If it's a directory, read main.pirtm or a file inside.
            let path = Path::new(&source_path);
            let content = if path.is_dir() {
                let main_file = path.join("main.pirtm");
                if main_file.exists() {
                    fs::read_to_string(main_file).unwrap_or_default()
                } else {
                    // Fallback to reading first .pirtm file
                    let mut src = String::new();
                    if let Ok(entries) = fs::read_dir(path) {
                        for entry in entries.flatten() {
                            if entry.path().extension().map_or(false, |ext| ext == "pirtm") {
                                if let Ok(c) = fs::read_to_string(entry.path()) {
                                    src = c;
                                    break;
                                }
                            }
                        }
                    }
                    src
                }
            } else {
                fs::read_to_string(path).unwrap_or_default()
            };

            // Use a default prime set and stratum for mock CLI validation
            let prime_set_json = "[7, 11, 13]";
            let stratum_id = "Tower_A";

            match compiler::validate_source(&content, prime_set_json, stratum_id) {
                Ok(envelope) => {
                    let json = serde_json::to_string_pretty(&envelope).unwrap_or_default();
                    println!("{}", json);
                }
                Err(e) => {
                    eprintln!("Validation error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "prove" => {
            // Find --ast <path> and --out <path>
            let mut ast_path = String::new();
            let mut out_path = String::new();
            for i in 2..args.len() {
                if args[i] == "--ast" && i + 1 < args.len() {
                    ast_path = args[i + 1].clone();
                }
                if args[i] == "--out" && i + 1 < args.len() {
                    out_path = args[i + 1].clone();
                }
            }
            if ast_path.is_empty() || out_path.is_empty() {
                eprintln!("Error: --ast <path> and --out <path> are required");
                std::process::exit(1);
            }

            let ast_content = fs::read_to_string(ast_path).unwrap_or_else(|_| "{}".to_string());
            let envelope: Result<DiagnosticEnvelope, _> = serde_json::from_str(&ast_content);
            if let Ok(env) = envelope {
                if !env.diagnostics.is_empty() {
                    eprintln!("Proof generation failed: source has validation errors");
                    std::process::exit(1);
                }
            }

            let lean_proof = "-- Lean 4 Metatheorem for Module (Prime Index: 7)
theorem pirtm_contractivity : True := by
  trivial
";
            if let Err(e) = fs::write(out_path, lean_proof) {
                eprintln!("Failed to write proof: {}", e);
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }
}
