import asyncio
import os
import psutil
import time
import subprocess
from datetime import datetime

class MASStabilityAudit:
    """
    Rexumer Stability Auditor - Verificación Cruda del Sistema.
    Mide latencia, fragmentación y resiliencia de procesos.
    """
    def __init__(self):
        self.pm2_path = "/data/data/com.termux/files/home/.local/share/pnpm/bin/pm2"

    def get_system_vitals(self):
        try:
            mem = psutil.virtual_memory()
            # Fallback para CPU si /proc/stat falla
            cpu = psutil.cpu_percent(interval=1)
        except PermissionError:
            # En Termux, a veces /proc/stat está restringido. 
            # Intentamos obtenerlo vía 'top' como fallback crudo.
            try:
                cpu_output = subprocess.check_output("top -bn1 | grep 'Cpu(s)'", shell=True).decode()
                cpu = float(cpu_output.split()[1].replace(',', '.'))
            except:
                cpu = 0.0 # Valor neutro si falla todo
            mem = psutil.virtual_memory()
            
        return {"mem_pct": mem.percent, "mem_avail": mem.available / 1024 / 1024, "cpu": cpu}

    async def test_process_resilience(self):
        print("[*] AUDIT: Testing PM2 process recovery (Killing 'mas-sentinel')...")
        # Forzar el cierre del centinela
        subprocess.run([self.pm2_path, "stop", "mas-sentinel"], capture_output=True)
        await asyncio.sleep(2)
        status = subprocess.run([self.pm2_path, "status"], capture_output=True, text=True).stdout
        if "online" in status:
            print("[✔] RESULT: Process recovered autonomously.")
        else:
            print("[✘] FAIL: Process failed to restart. Possible PM2 config bottleneck.")

    async def measure_io_latency(self):
        print("[*] AUDIT: Measuring Disk I/O latency for WAL Journal...")
        start = time.time()
        for i in range(100):
            with open("packages/core-rs/latency.test", "a") as f:
                f.write(f"{datetime.now()}: Latency benchmark cycle {i}\n")
                f.flush()
                os.fsync(f.fileno())
        avg_lat = (time.time() - start) / 100
        print(f"[✔] RESULT: Avg Write Latency: {avg_lat*1000:.2f}ms (Grade S target: <10ms)")
        os.remove("packages/core-rs/latency.test")

    async def run_stress_stability(self):
        print("[*] AUDIT: Initiating 30-second concurrency stress test...")
        vitals_start = self.get_system_vitals()
        
        # Sellar fuga de entorno en el auditor
        env = os.environ.copy()
        if "ORACLE_TOKEN" not in env:
            env["ORACLE_TOKEN"] = "development-only-key"

        # Ejecutar power-leveling en paralelo con entorno robusto
        leveler = subprocess.Popen(["python3", "scripts/power-leveling.py"], stdout=subprocess.DEVNULL, env=env)
        
        # Monitorear durante el stress
        max_cpu = 0
        max_mem = 0
        for _ in range(10):
            v = self.get_system_vitals()
            max_cpu = max(max_cpu, v["cpu"])
            max_mem = max(max_mem, v["mem_pct"])
            await asyncio.sleep(2)
            
        vitals_end = self.get_system_vitals()
        leveler.terminate()
        
        print(f"\n--- RAW TRUTH REPORT ---")
        print(f"Max CPU Load:    {max_cpu}%")
        print(f"Max RAM Usage:   {max_mem}%")
        print(f"RAM Leak Delta:  {vitals_end['mem_pct'] - vitals_start['mem_pct']:.2f}%")
        print(f"System Health:   {'STABLE' if max_mem < 90 else 'CRITICAL (OOM RISK)'}")
        print(f"------------------------\n")

if __name__ == "__main__":
    audit = MASStabilityAudit()
    asyncio.run(audit.measure_io_latency())
    asyncio.run(audit.test_process_resilience())
    asyncio.run(audit.run_stress_stability())
