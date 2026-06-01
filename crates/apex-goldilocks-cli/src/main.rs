use clap::{Parser, Subcommand};
use apex_goldilocks_core::boundary_lattice::LatticeCertificate;
use multiplicity_runtime::{MultiplicityRuntime, CRMFConfig};
use multiplicity_runtime::harness::{NeuralHarness, EchoBraidState, HarnessAdapter};
use apex_hologram::{AtlasEmbeddingProof, RecursiveProof};
use goldilocks::{GoldilocksField, PrimeMask};
use apex_goldilocks_core::GoldVector;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Verify the 12,288 Boundary Lattice invariants.
    AuditLattice,
    /// Run an end-to-end EchoBraid certification pilot.
    Pilot {
        #[arg(short, long, default_value_t = 0x42)]
        domain: u64,
        #[arg(short, long, default_value_t = 100)]
        budget: u64,
    },
    /// Verify ACE stability for a set of operators (PIRTM Phase C).
    VerifyStability {
        /// Sum of operator norms (in units of 0.000001)
        #[arg(short, long)]
        total_norm: i64,
        /// Governance constant c (in units of 0.000001)
        #[arg(short, long, default_value_t = 50000)]
        gov_c: i64,
    },
    /// Validate PIRTM source code using tree-sitter (PIRTM Phase A).
    ValidatePirtm {
        /// PIRTM source code or file path
        #[arg(short, long)]
        source: String,
        /// JSON set of allowed primes
        #[arg(short, long, default_value = "[2, 3, 5, 7, 11, 13]")]
        primes: String,
        /// Stratum ID for admissibility check
        #[arg(short, long, default_value = "S1")]
        stratum: String,
    },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::AuditLattice => {
            println!("Auditing 12,288 Boundary Lattice...");
            let cert = LatticeCertificate::verify();
            println!("Total Elements: {}", cert.total_elements);
            println!("Orbit Sizes: {:?}", cert.orbit_sizes);
            println!("Free Action: {}", cert.is_free_action);
            if cert.total_elements == 12288 && cert.is_free_action {
                println!("Lattice Verification: PASSED");
            } else {
                println!("Lattice Verification: FAILED");
                std::process::exit(1);
            }
        }
        Commands::ValidatePirtm { source, primes, stratum } => {
            println!("Validating PIRTM Source (PIRTM Phase A)...");
            // Check if source is a file path
            let content = if std::path::Path::new(source).exists() {
                std::fs::read_to_string(source).expect("Failed to read PIRTM source file")
            } else {
                source.clone()
            };

            match pirtm_compiler::compiler::validate_source(&content, primes, stratum) {
                Ok(envelope) => {
                    println!("Validation Complete.");
                    println!("Version: {}", envelope.version);
                    if envelope.diagnostics.is_empty() {
                        println!("Result: ADMISSIBLE");
                    } else {
                        println!("Result: INADMISSIBLE");
                        for diag in envelope.diagnostics {
                            println!("  [{}:{}] {:?} - {}", diag.start_line, diag.start_col, diag.code, diag.message);
                        }
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    println!("Validation Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::VerifyStability { total_norm, gov_c } => {
            println!("Verifying ACE Stability (PIRTM Phase C)...");
            let op = pirtm_compiler::ace::DynamicOperator {
                signature: pirtm_compiler::types::Sig::new(),
                norm: pirtm_compiler::ace::FixedPoint(*total_norm),
            };
            let c = pirtm_compiler::ace::FixedPoint(*gov_c);
            
            match pirtm_compiler::ace::verify_stability(&[op], c) {
                Ok(_) => println!("Stability Check: PASSED"),
                Err(e) => {
                    println!("Stability Check: FAILED ({})", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Pilot { domain, budget } => {
            println!("Starting EchoBraid Pilot for Domain 0x{:X}...", domain);
            
            // 1. Initialize Runtime
            let config = CRMFConfig {
                domain_tag: *domain,
                prime_index: 256,
                prime_mask: PrimeMask(0xFFFFFFFFFFFFFFFF),
                signature: None,
            };
            let mut runtime = MultiplicityRuntime::new(config, *budget);
            let harness = NeuralHarness::new(10);
            
            // 2. Initial State
            let initial_theta = GoldVector::new(vec![GoldilocksField::new(1); 10]);
            let current_state = EchoBraidState {
                theta: initial_theta,
                iteration: 0,
            };

            // 3. Harness Adaptation
            let mut adapter = HarnessAdapter::new(&mut runtime, harness);
            let proposal = adapter.harness.propose_adaptation(&current_state);
            
            println!("Committing EchoBraid proposal (Iteration {})...", proposal.proposed_state.iteration);
            
            // 4. Veto-Enforced Certification
            match adapter.commit_proposal(proposal) {
                Ok(_) => {
                    println!("Certification: SUCCESS");
                    
                    // 5. Generate AEP
                    let initial_hash = [0x55; 32]; // Mock hash
                    let proof = RecursiveProof::new_initial(initial_hash);
                    let aep = AtlasEmbeddingProof {
                        proof,
                        domain_tag: *domain,
                    };
                    println!("Atlas-Embedding Proof (AEP) generated for Domain 0x{:X}", aep.domain_tag);
                }
                Err(e) => {
                    println!("Certification: VETOED ({})", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
