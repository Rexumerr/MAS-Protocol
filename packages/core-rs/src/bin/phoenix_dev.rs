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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    let openrouter_key = env::var("OPENROUTER_KEY").ok();
    let phoenix = PhoenixArchitect::new(openrouter_key);

    match cli.command {
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
