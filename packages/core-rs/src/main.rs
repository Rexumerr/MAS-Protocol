use clap::{Parser, Subcommand};
use core_rs::{UltraBrain, Capability, OracleBridge};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "BrainCLI")]
#[command(about = "The command-line interface for the UltraBrain", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ingest an event from the real world
    Ingest {
        #[arg(short, long)]
        event: String,
        #[arg(short, long)]
        value: f64,
        #[arg(short, long, default_value = "state.json")]
        state_path: String,
    },
    /// Show current status of all capabilities
    Status {
        #[arg(short, long, default_value = "state.json")]
        state_path: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let brain = UltraBrain::new();
    let oracle = OracleBridge::new("internal-key".to_string());

    match cli.command {
        Commands::Ingest { event, value, state_path } => {
            let result = oracle.ingest_event(&brain, event.clone(), value);
            println!("{}", result);
            
            // Save updated state to JSON for Astro to consume
            save_state(&brain, &state_path)?;
        }
        Commands::Status { state_path: _ } => {
            println!("--- Multiversal Status ---");
            println!("DataMining: Level {}", brain.get_level(Capability::DataMining));
            println!("Automation: Level {}", brain.get_level(Capability::Automation));
            // ... etc
        }
    }

    Ok(())
}

fn save_state(brain: &UltraBrain, path: &str) -> anyhow::Result<()> {
    // Collect levels into a JSON structure
    let mut state = std::collections::HashMap::new();
    state.insert("DataMining", brain.get_level(Capability::DataMining));
    state.insert("Automation", brain.get_level(Capability::Automation));
    state.insert("LeadGen", brain.get_level(Capability::LeadGen));
    state.insert("Infrastructure", brain.get_level(Capability::Infrastructure));
    state.insert("Optimization", brain.get_level(Capability::Optimization));
    state.insert("Security", brain.get_level(Capability::Security));

    let json = serde_json::to_string_pretty(&state)?;
    fs::write(path, json)?;
    Ok(())
}
