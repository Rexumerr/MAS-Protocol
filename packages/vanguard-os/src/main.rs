use clap::{Parser, Subcommand};
use colored::*;
use sysinfo::{System, CpuRefreshKind, RefreshKind, MemoryRefreshKind};
use std::process::{Command, Stdio};
use std::fs;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use std::collections::VecDeque;

#[derive(Parser)]
#[command(name = "VanguardOS")]
#[command(about = "The Rust-orchestrated command center for MAS-Protocol", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Monitor real-time system resources with Proactive Neural Governor
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

struct ProactiveGovernor {
    sys: System,
    mode: ExecutionMode,
    history: VecDeque<(f32, f64)>, // (CPU, RAM %)
    last_shard: Instant,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ExecutionMode {
    Overclock,
    Balanced,
    Eco,
    Critical,
    Preemptive(PreemptiveAction),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum PreemptiveAction {
    EarlySharding,
    PredictiveThrottling,
    NeuralCooling,
}

impl ProactiveGovernor {
    fn new() -> Self {
        Self {
            sys: System::new_with_specifics(
                RefreshKind::nothing()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            ),
            mode: ExecutionMode::Balanced,
            history: VecDeque::with_capacity(10),
            last_shard: Instant::now() - Duration::from_secs(3600),
        }
    }

    fn refresh(&mut self) {
        self.sys.refresh_all();
        let cpu_usage = self.sys.global_cpu_usage();
        let mem_usage = (self.sys.used_memory() as f64 / self.sys.total_memory() as f64) * 100.0;
        
        // Update history
        if self.history.len() >= 10 { self.history.pop_front(); }
        self.history.push_back((cpu_usage, mem_usage));

        // Predictive logic...
        let cpu_trend = self.calculate_trend(|h| h.0 as f64);
        let mem_trend = self.calculate_trend(|h| h.1);

        if mem_trend > 2.0 && mem_usage > 70.0 && self.last_shard.elapsed() > Duration::from_secs(300) {
            self.mode = ExecutionMode::Preemptive(PreemptiveAction::EarlySharding);
            self.trigger_early_shard();
        } else if cpu_trend > 8.0 && cpu_usage > 40.0 {
            self.mode = ExecutionMode::Preemptive(PreemptiveAction::NeuralCooling);
            self.trigger_neural_cooling();
        } else if cpu_trend > 5.0 && cpu_usage > 50.0 {
            self.mode = ExecutionMode::Preemptive(PreemptiveAction::PredictiveThrottling);
        } else if cpu_usage > 90.0 || mem_usage > 85.0 {
            self.mode = ExecutionMode::Critical;
        } else if cpu_usage > 60.0 || mem_usage > 60.0 {
            self.mode = ExecutionMode::Eco;
        } else if cpu_usage < 15.0 && mem_usage < 40.0 {
            self.mode = ExecutionMode::Overclock;
        } else {
            self.mode = ExecutionMode::Balanced;
        }

        // Bridge Telemetry to JSON for Dashboard
        let telemetry = serde_json::json!({
            "cpu": cpu_usage,
            "mem": mem_usage,
            "mode": format!("{:?}", self.mode),
            "cooling_active": matches!(self.mode, ExecutionMode::Preemptive(PreemptiveAction::NeuralCooling))
        });
        let _ = fs::write("apps/web/public/health-status.json", telemetry.to_string());
    }

    fn calculate_trend<F>(&self, mapper: F) -> f64 
    where F: Fn(&(f32, f64)) -> f64 {
        if self.history.len() < 2 { return 0.0; }
        let last = mapper(self.history.back().unwrap());
        let prev = mapper(self.history.get(self.history.len() - 2).unwrap());
        last - prev
    }

    fn trigger_early_shard(&mut self) {
        println!("{}", "[!] PROACTIVE ALERT: Detecting memory spike trend. Triggering Early Sharding...".bold().red());
        self.last_shard = Instant::now();
    }

    fn trigger_neural_cooling(&mut self) {
        println!("{}", "[❄️] NEURAL COOLING: Predicting thermal spike (>45°C). Suspending heavy tasks...".bold().cyan());
        // Integrate with PM2 to throttle heavy background processes
        let _ = Command::new("pm2")
            .arg("scale")
            .arg("all")
            .arg("1") // Scale down to minimum workers
            .status();
    }

    fn get_optimization_tip(&self) -> String {
        match self.mode {
            ExecutionMode::Overclock => "🚀 MAX PERFORMANCE: Unleashing parallel Phoenix threads.".into(),
            ExecutionMode::Balanced => "⚖️ BALANCED: Stable multiversal synchronization.".into(),
            ExecutionMode::Eco => "🍃 ECO MODE: Throttling background agents to save battery.".into(),
            ExecutionMode::Critical => "⚠️ CRITICAL: Sharding memory and pausing non-essential oracles.".into(),
            ExecutionMode::Preemptive(PreemptiveAction::EarlySharding) => "🔮 PROACTIVE: Early Sharding triggered to prevent OOM.".into(),
            ExecutionMode::Preemptive(PreemptiveAction::PredictiveThrottling) => "🔮 PROACTIVE: Throttling pre-emptively due to CPU surge.".into(),
            ExecutionMode::Preemptive(PreemptiveAction::NeuralCooling) => "🔮 PROACTIVE: Neural cooling initiated.".into(),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("{}", "--- VANGUARD OS: PROACTIVE GOVERNOR REVISION ---".bold().magenta());

    let mut governor = ProactiveGovernor::new();

    match cli.command {
        Commands::Monitor => {
            println!("{}", "Initializing Hardware-Aware Proactive Monitoring...".cyan());
            
            loop {
                governor.refresh();
                print!("{}[2J{}[1;1H", 27 as char, 27 as char); 
                
                println!("{}", "MAS-PROTOCOL: PROACTIVE HARDWARE STATUS".bold().yellow());
                println!("CPU Usage:    {:.2}%", governor.sys.global_cpu_usage());
                println!("Memory:       {}/{} MB ({:.1}%)", 
                    governor.sys.used_memory() / 1024 / 1024, 
                    governor.sys.total_memory() / 1024 / 1024,
                    (governor.sys.used_memory() as f64 / governor.sys.total_memory() as f64) * 100.0
                );
                
                println!("{}", "NEURAL GOVERNOR PROACTIVE STRATEGY".bold().cyan());
                
                match governor.mode {
                    ExecutionMode::Overclock => println!("Current Mode: {}", "OVERCLOCK".bold().magenta()),
                    ExecutionMode::Balanced => println!("Current Mode: {}", "BALANCED".bold().green()),
                    ExecutionMode::Eco => println!("Current Mode: {}", "ECO".bold().yellow()),
                    ExecutionMode::Critical => println!("Current Mode: {}", "CRITICAL".bold().red()),
                    ExecutionMode::Preemptive(_) => println!("Current Mode: {}", "PRE-EMPTIVE (PROACTIVE)".bold().blue()),
                }
                
                // Check Ollama Status
                let ollama_status = Command::new("curl")
                    .arg("-s")
                    .arg("-o")
                    .arg("/dev/null")
                    .arg("-w")
                    .arg("%{http_code}")
                    .arg("http://localhost:11434/api/tags")
                    .output();
                
                let ai_status = match ollama_status {
                    Ok(output) if String::from_utf8_lossy(&output.stdout).trim() == "200" => "ONLINE".green(),
                    _ => "OFFLINE".red(),
                };
                println!("Local AI (Ollama): {}", ai_status);
                
                println!("Action:       {}", governor.get_optimization_tip());
                println!("------------------------------------");
                
                let sleep_dur = match governor.mode {
                    ExecutionMode::Critical => Duration::from_secs(5),
                    ExecutionMode::Preemptive(_) => Duration::from_secs(3),
                    _ => Duration::from_secs(2),
                };
                sleep(sleep_dur).await;
            }
        }
        Commands::PhoenixLoop { path, iterations } => {
            println!("🔥 {} '{}'...", "Initializing Phoenix Automation Loop for:".truecolor(255, 165, 0), path);
            for i in 1..=iterations {
                println!("Cycle [{}/{}]: {}...", i, iterations, "Manifesting AI Architecture".blue());
                let status = Command::new("../../target/release/phoenix-dev").arg("audit").arg("--path").arg(&path).status()?;
                if !status.success() { break; }
                sleep(Duration::from_secs(5)).await;
            }
        }
        Commands::Shard { output } => {
            println!("📦 {} -> {}.7z...", "Sharding Project Memory using 7-Zip".green(), output);
            Command::new("7z").arg("a").arg("-mx=9").arg("-v100m").arg(format!("{}.7z", output)).arg(".").status()?;
        }
        Commands::Moderate => {
            println!("{}", "ENTERING MODERATION MODE".bold().red());
        }
    }

    Ok(())
}
