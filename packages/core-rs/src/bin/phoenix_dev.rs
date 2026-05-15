use clap::{Parser, Subcommand};
use core_rs::{PhoenixArchitect};
use std::env;

#[derive(Parser)]
#[command(name = "PhoenixDev")]
#[command(about = "Grade S AI-Architect for Rust development", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Audit a Rust file using AI
    Audit {
        #[arg(short, long)]
        path: String,
        #[arg(short, long, default_value = "ollama")]
        engine: String, // ollama or openrouter
    },
    /// Generate a Rust module from a description
    Manifest {
        #[arg(short, long)]
        description: String,
        #[arg(short, long, default_value = "ollama")]
        engine: String,
    },
    /// Reason about a problem using a Hermetic Principle
    Think {
        #[arg(short, long)]
        prompt: String,
        #[arg(short, long, default_value = "mentalism")]
        principle: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    let openrouter_key = env::var("OPENROUTER_KEY").ok();
    let phoenix = PhoenixArchitect::new(openrouter_key);

    match cli.command {
        // ... previous cases ...
        Commands::Think { prompt, principle } => {
            use core_rs::HermeticPrinciple;
            let p_enum = match principle.to_lowercase().as_str() {
                "mentalism" => HermeticPrinciple::Mentalism,
                "correspondence" => HermeticPrinciple::Correspondence,
                "vibration" => HermeticPrinciple::Vibration,
                "polarity" => HermeticPrinciple::Polarity,
                "rhythm" => HermeticPrinciple::Rhythm,
                "cause" => HermeticPrinciple::CauseEffect,
                "gender" => HermeticPrinciple::Gender,
                _ => anyhow::bail!("Unknown hermetic principle"),
            };

            let response = phoenix.hermetic_reasoning(&prompt, p_enum).await?;
            println!("\n--- PHOENIX HERMETIC REASONING ---\n");
            println!("{}", response);
        }
        Commands::Audit { path, engine } => {
            println!("🔥 Phoenix is auditing: {} using {} engine...", path, engine);
            let code = std::fs::read_to_string(&path)?;
            let prompt = format!("Audit this Rust code for security and performance. Use vanguard 2026 standards:\n\n{}", code);
            
            let response = if engine == "openrouter" {
                phoenix.ask_openrouter(&prompt).await?
            } else {
                phoenix.ask_ollama(&prompt).await?
            };

            println!("\n--- PHOENIX AUDIT REPORT ---\n");
            println!("{}", response);
        }
        Commands::Manifest { description, engine } => {
            println!("⚡ Phoenix is manifesting code from: '{}'...", description);
            let prompt = format!("Generate a high-performance Rust module based on this description. Use modern 2026 patterns:\n\n{}", description);
            
            let response = if engine == "openrouter" {
                phoenix.ask_openrouter(&prompt).await?
            } else {
                phoenix.ask_ollama(&prompt).await?
            };

            println!("\n--- PHOENIX MANIFESTATION ---\n");
            println!("{}", response);
        }
    }

    Ok(())
}
