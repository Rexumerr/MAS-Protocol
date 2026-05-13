use clap::{Parser, Subcommand};
use core_rs::{UltraBrain, Capability, OracleBridge, NeuralCrypt};
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
        #[arg(short, long, default_value = "state.bin")]
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
        #[arg(short, long, default_value = "state.bin")]
        state_path: String,
    },
    /// Show current status of the Vanguard Avatar
    Status {
        #[arg(short, long, default_value = "state.bin")]
        state_path: String,
    },
    /// Show the Vanguard Avatar's profile and inventory
    Profile {
        #[arg(short, long, default_value = "state.bin")]
        state_path: String,
    },
    /// Forge a new infrastructure tool using resources
    Forge {
        #[arg(short, long)]
        item: String,
        #[arg(short, long, default_value = "state.bin")]
        state_path: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // SECURITY: NEURAL GATE - Mandatory ORACLE_TOKEN
    let is_prod = env::var("NODE_ENV").unwrap_or_default() == "production";
    let required_token = env::var("ORACLE_TOKEN").map_err(|_| {
        if is_prod {
            anyhow::anyhow!("CRITICAL: ORACLE_TOKEN must be set in production.")
        } else {
            anyhow::anyhow!("ORACLE_TOKEN not found. Set it to continue.")
        }
    })?;

    let oracle = OracleBridge::new(required_token.clone());

    match cli.command {
        Commands::Ingest { event, value, state_path, token } => {
            let provided_token = token.unwrap_or_else(|| "".to_string());
            if !oracle.validate_token(&provided_token) {
                anyhow::bail!("CRITICAL: Unauthorized access to Neural Gate. Event rejected.");
            }

            let brain = load_state(&state_path, &required_token).unwrap_or_else(|_| UltraBrain::new());
            let result = oracle.ingest_event(&brain, event.clone(), value);
            println!("{}", result);
            save_state(&brain, &state_path, &required_token)?;
        }
        Commands::Quest { action, id, state_path } => {
            let brain = load_state(&state_path, &required_token).unwrap_or_else(|_| UltraBrain::new());
            match action.as_str() {
                "list" => {
                    println!("--- Vanguard Active Quests ---");
                    println!("[1] Data Farmer - Goal: Farming Lvl 10 (Status: InProgress)");
                    println!("[2] Infrastructure Forge - Req: Smithing Lvl 5 (Status: Locked)");
                },
                "complete" => {
                    if let Some(qid) = id {
                        println!("Quest {} completed! +1,000.00 XP granted.", qid);
                        brain.grant_xp(Capability::Optimization, 1000.0);
                        save_state(&brain, &state_path, &required_token)?;
                    }
                },
                _ => println!("Unknown quest action."),
            }
        }
        Commands::Status { state_path } => {
            let brain = load_state(&state_path, &required_token).unwrap_or_else(|_| UltraBrain::new());
            let avatar = brain.avatar();
            println!("--- {}'s Vanguard Status ---", avatar.name());
            println!("Title: {}", avatar.title());
            for cap in [
                Capability::Farming,
                Capability::Woodcutting,
                Capability::Smithing,
                Capability::Alchemy,
                Capability::Combat,
                Capability::Merchanting,
            ] {
                println!("{:?}: Level {} ({:.2} XP)", cap, avatar.get_level(cap), avatar.get_xp(cap));
            }
        }
        Commands::Profile { state_path } => {
            let brain = load_state(&state_path, &required_token).unwrap_or_else(|_| UltraBrain::new());
            let avatar = brain.avatar();
            println!("--- Vanguard Profile: {} ---", avatar.name());
            println!("Title: {}", avatar.title());
            println!("\n[Inventory]");
            println!("RawData: {} units", avatar.get_material_count("RawData"));
            println!("Infrastructure Scraps: {} units", avatar.get_material_count("Scraps"));
        }
        Commands::Forge { item, state_path } => {
            let brain = load_state(&state_path, &required_token).unwrap_or_else(|_| UltraBrain::new());
            let avatar = brain.avatar();
            
            if item == "automation_bot" {
                if avatar.get_material_count("RawData") >= 100 {
                    println!("Success: Forged a new Automation Bot!");
                    avatar.grant_xp(Capability::Smithing, 2000.0);
                    // consume resources logic would go here
                    save_state(&brain, &state_path, &required_token)?;
                } else {
                    println!("Error: Insufficient RawData (Need 100). current: {}", avatar.get_material_count("RawData"));
                }
            } else {
                println!("Unknown item forging recipe.");
            }
        }
    }

    Ok(())
}

fn save_state(brain: &UltraBrain, path: &str, token: &str) -> anyhow::Result<()> {
    let json = serde_json::to_vec(brain)?;
    let encrypted = NeuralCrypt::encrypt(&json, token)?;
    
    // We use Base64 only for the final storage string to keep it "text-friendly" 
    // but the underlying data is AES-GCM encrypted.
    let encoded = general_purpose::STANDARD.encode(encrypted);
    
    fs::write(path, encoded)?;
    Ok(())
}

fn load_state(path: &str, token: &str) -> anyhow::Result<UltraBrain> {
    let encoded = fs::read_to_string(path)?;
    let encrypted = general_purpose::STANDARD.decode(encoded.trim())?;
    let decrypted = NeuralCrypt::decrypt(&encrypted, token)?;
    let brain: UltraBrain = serde_json::from_slice(&decrypted)?;
    Ok(brain)
}
