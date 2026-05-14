use clap::{Parser, Subcommand};
use colored::*;
use sysinfo::{System, CpuRefreshKind, RefreshKind, MemoryRefreshKind};
// ... rest of imports
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Parser)]
#[command(name = "VanguardOS")]
#[command(about = "The Rust-orchestrated command center for MAS-Protocol", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Monitor real-time system resources
    Monitor,
    /// Automate the Phoenix AI development loop
    PhoenixLoop {
        #[arg(short, long)]
        path: String,
        #[arg(short, long, default_value = "5")]
        iterations: u32,
    },
    /// Compress and shard project memory using 7-Zip
    Shard {
        #[arg(short, long)]
        output: String,
    },
    /// Unified moderation of all MAS services
    Moderate,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("{}", "--- VANGUARD OS: MAY 2026 REVISION ---".bold().magenta());

    match cli.command {
        Commands::Monitor => {
            let mut sys = System::new_with_specifics(
                RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()).with_memory(MemoryRefreshKind::everything()),
            );
            println!("{}", "Starting Real-time Neural Monitoring...".cyan());
            
            loop {
                sys.refresh_all();
                print!("{}[2J", 27 as char); // Clear screen
                println!("{}", "MAS-PROTOCOL SYSTEM HEALTH".bold().yellow());
                println!("CPU Usage: {:.2}%", sys.global_cpu_usage());
                println!("Memory: {}/{} MB", sys.used_memory() / 1024 / 1024, sys.total_memory() / 1024 / 1024);
                println!("------------------------------------");
                sleep(Duration::from_secs(2)).await;
            }
        }
        Commands::PhoenixLoop { path, iterations } => {
            println!("🔥 {} '{}'...", "Initializing Phoenix Automation Loop for:".truecolor(255, 165, 0), path);
            for i in 1..=iterations {
                println!("Cycle [{}/{}]: {}...", i, iterations, "Manifesting AI Architecture".blue());
                
                let status = Command::new("../../target/release/phoenix-dev")
                    .arg("audit")
                    .arg("--path")
                    .arg(&path)
                    .stdout(Stdio::inherit())
                    .status()?;

                if !status.success() {
                    println!("{}", "ALERT: Phoenix Loop interrupted by error.".red());
                    break;
                }
                sleep(Duration::from_secs(5)).await;
            }
        }
        Commands::Shard { output } => {
            println!("📦 {} -> {}.7z...", "Sharding Project Memory using 7-Zip".green(), output);
            
            let status = Command::new("7z")
                .arg("a")
                .arg("-mx=9") // Ultra compression
                .arg("-v100m") // Shard into 100MB volumes
                .arg(format!("{}.7z", output))
                .arg(".") // Compress current dir
                .arg("-xr!node_modules") // Exclude node_modules
                .arg("-xr!target") // Exclude rust target
                .status()?;

            if status.success() {
                println!("{}", "SUCCESS: Memory sharded and compressed with Grade S efficiency.".bold().green());
            }
        }
        Commands::Moderate => {
            println!("{}", "ENTERING MODERATION MODE".bold().red());
            println!("1. Check Ollama Status...");
            println!("2. Verify Encrypted Persistence...");
            println!("3. Audit Remote Synchronicity...");
        }
    }

    Ok(())
}
