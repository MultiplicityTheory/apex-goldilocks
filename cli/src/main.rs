use clap::{Parser, Subcommand};
use apex_core::Atlas;
use apex_moonshine::zk_sketch;
use apex_hologram::HologramExecutor;
use apex_sigmatics::Generator;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "apex")]
#[command(about = "Unified Apex System CLI (Sedona Spine Edition)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Construct the Atlas graph and output metadata
    ConstructAtlas,
    /// Run Moonshine ZK-sketch generation
    MoonshineSketch {
        #[arg(short, long)]
        seed: String,
    },
    /// Execute a certified holographic compute block
    HologramRun {
        #[arg(short, long)]
        program: String,
        #[arg(short, long)]
        actor: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ConstructAtlas => {
            println!("Constructing Atlas from first principles...");
            let atlas = Atlas::new();
            println!("Atlas construction complete.");
            println!("Vertices: {}", atlas.num_vertices());
            println!("Edges: {}", atlas.num_edges());
            println!("Unity Positions: {:?}", atlas.unity_positions());
            
            let out = serde_json::json!({
                "vertices": atlas.num_vertices(),
                "edges": atlas.num_edges(),
                "unity": atlas.unity_positions(),
            });
            println!("{}", serde_json::to_string_pretty(&out)?);
        }
        Commands::MoonshineSketch { seed } => {
            println!("Generating Moonshine ZK-sketch with exact arithmetic...");
            let sketch = zk_sketch(seed);
            println!("{}", serde_json::to_string_pretty(&sketch)?);
        }
        Commands::HologramRun { program, actor } => {
            println!("Running Certified Holographic Compute...");
            
            let program_content = fs::read_to_string(Path::new(program))?;
            let generators: Vec<Generator> = serde_json::from_str(&program_content)?;
            
            let mut exec = HologramExecutor::new();
            match exec.execute_and_certify(&generators, actor) {
                Ok(cert) => println!("{}", serde_json::to_string_pretty(&cert)?),
                Err(e) => eprintln!("Execution Error: {}", e),
            }
        }
    }
    Ok(())
}
