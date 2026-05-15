import os
import subprocess
import secrets
import hashlib
import sys

def run_command(cmd):
    try:
        result = subprocess.run(cmd, shell=True, check=True, capture_output=True, text=True)
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        return None

def get_hw_id():
    # Derivamos una semilla del hardware para la entropía del token
    boot_id = run_command("cat /proc/sys/kernel/random/boot_id")
    if not boot_id:
        boot_id = "vanguard-fallback-seed-2026"
    return hashlib.sha256(boot_id.encode()).hexdigest()

def setup_apex_secrets():
    print("\n--- 🛡️ REXUMER APEX: ZERO-TOUCH SECRET AUTOMATION ---")
    
    # 1. Verificar GitHub CLI
    gh_auth = run_command("gh auth status")
    if not gh_auth:
        print("[!] ERROR: GitHub CLI (gh) not authenticated. Please run 'gh auth login' once.")
        sys.exit(1)

    # 2. Generar o Recuperar ORACLE_TOKEN
    oracle_token = os.getenv("ORACLE_TOKEN")
    if not oracle_token:
        print("[*] Generating new high-entropy ORACLE_TOKEN...")
        hw_seed = get_hw_id()
        random_seed = secrets.token_hex(32)
        oracle_token = hashlib.sha3_512(f"{hw_seed}:{random_seed}".encode()).hexdigest()
        
        # Guardar localmente de forma persistente (invisible node)
        with open(".env", "a") as f:
            f.write(f"\nORACLE_TOKEN={oracle_token}\n")
        print("[+] Local ORACLE_TOKEN manifested.")

    # 3. Sincronizar con GitHub Secrets automáticamente
    print("[*] Synchronizing secrets with GitHub repository...")
    
    # Subir ORACLE_TOKEN
    sync_result = run_command(f"gh secret set ORACLE_TOKEN --body '{oracle_token}'")
    if sync_result is not None:
        print("[✔] ORACLE_TOKEN synchronized with Cloud Gate.")
    else:
        print("[✘] Failed to sync ORACLE_TOKEN. Check repo permissions.")

    print("\n[COMPLETE] Your infrastructure is now cloud-synchronized and secured.")
    print("Zero-touch execution successful.\n")

if __name__ == "__main__":
    setup_apex_secrets()
