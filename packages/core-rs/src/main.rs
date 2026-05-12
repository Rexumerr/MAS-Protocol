use clap::{Parser, Subcommand};
use core_rs::{UltraBrain, Capability, OracleBridge};
use std::fs;
use std::env;
use base64::{Engine as _, engine::general_purpose};

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
        #[arg(short, long)]
        token: Option<String>,
    },
    /// Manage Business Quests
    Quest {
        #[arg(short, long)]
        action: String, // list, start, complete
        #[arg(short, long)]
        id: Option<String>,
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
    
    // SECURITY: NEURAL GATE
    let required_token = env::var("ORACLE_TOKEN").unwrap_or_else(|_| "development-only-key".to_string());
    let oracle = OracleBridge::new(required_token.clone());

    match cli.command {
        Commands::Ingest { event, value, state_path, token } => {
            let provided_token = token.unwrap_or_else(|| "".to_string());
            if provided_token != required_token && env::var("ORACLE_TOKEN").is_ok() {
                anyhow::bail!("CRITICAL: Unauthorized access to Neural Gate. Event rejected.");
            }

            let result = oracle.ingest_event(&brain, event.clone(), value);
            println!("{}", result);
            save_state(&brain, &state_path)?;
        }
        Commands::Quest { action, id, state_path } => {
            match action.as_str() {
                "list" => {
                    println!("--- Rexumer Active Quests ---");
                    println!("[1] Regional Market Dominance - Req: Infrastructure Lvl 70 (Status: InProgress)");
                    println!("[2] High-Ticket Lead Factory - Req: LeadGen Lvl 50 (Status: Locked)");
                },
                "complete" => {
                    if let Some(qid) = id {
                        println!("Quest {} completed! +10,000 XP granted to Multiverse.", qid);
                        brain.grant_xp(Capability::Optimization, 10000.0);
                        save_state(&brain, &state_path)?;
                    }
                },
                _ => println!("Unknown quest action."),
            }
        }
        Commands::Status { state_path: _ } => {
            println!("--- Rexumer Multiversal Status ---");
            println!("DataMining: Level {}", brain.get_level(Capability::DataMining));
            println!("Automation: Level {}", brain.get_level(Capability::Automation));
            println!("Infrastructure: Level {}", brain.get_level(Capability::Infrastructure));
        }
    }

    Ok(())
}

fn save_state(brain: &UltraBrain, path: &str) -> anyhow::Result<()> {
    let mut state = std::collections::HashMap::new();
    state.insert("DataMining", brain.get_level(Capability::DataMining));
    state.insert("Automation", brain.get_level(Capability::Automation));
    state.insert("LeadGen", brain.get_level(Capability::LeadGen));
    state.insert("Infrastructure", brain.get_level(Capability::Infrastructure));
    state.insert("Optimization", brain.get_level(Capability::Optimization));
    state.insert("Security", brain.get_level(Capability::Security));

    let json = serde_json::to_string_pretty(&state)?;
    
    // OBFUSCATION: Base64 encoding for local-first privacy
    let obfuscated = general_purpose::STANDARD.encode(json);
    
    fs::write(path, obfuscated)?;
    Ok(())
}
