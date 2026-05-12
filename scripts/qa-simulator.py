import os
import subprocess
import time
import shutil

def log(msg):
    print(f"[QA-SIMULATOR] {msg}")
    with open("debug_report.log", "a") as f:
        f.write(f"{time.strftime('%Y-%m-%d %H:%M:%S')} - {msg}\n")

def check_environment():
    log("Checking environment prerequisites...")
    tools = ["cargo", "node", "npm", "python3"]
    for tool in tools:
        if shutil.which(tool) is None:
            log(f"CRITICAL: {tool} not found.")
            return False
    log("Environment OK.")
    return True

def simulate_build():
    log("Simulating 'sync-and-deploy' job...")
    
    # 1. Rust Build
    log("Step 1: Compiling Rust Core...")
    res = subprocess.run("cd packages/core-rs && cargo build --release", shell=True, capture_output=True, text=True)
    if res.returncode != 0:
        log(f"FAILED: Rust compilation. Error: {res.stderr}")
        return False
    
    # 2. Sync Script
    log("Step 2: Running Sync Metrics...")
    res = subprocess.run("npx ts-node scripts/sync-metrics.ts", shell=True, capture_output=True, text=True)
    if res.returncode != 0:
        log(f"FAILED: Sync script execution. Error: {res.stderr}")
        return False
    
    # 3. Astro Build
    log("Step 3: Building Dashboard...")
    res = subprocess.run("npm run build", shell=True, capture_output=True, text=True)
    if res.returncode != 0:
        log(f"FAILED: Astro build. Error: {res.stderr}")
        return False

    log("Simulation SUCCESSFUL.")
    return True

if __name__ == "__main__":
    if os.path.exists("debug_report.log"):
        os.remove("debug_report.log")
    if check_environment():
        simulate_build()
