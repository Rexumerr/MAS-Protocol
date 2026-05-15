# 📖 MAS-PROTOCOL: OPERATIONAL MANUAL (Q2 2026)

> **"Sovereign node operation requires absolute discipline."**

## 1. 🏗️ Environment Architecture
The MAS-Protocol operates as a localized edge node, primarily optimized for mobile hardware via **proot-distro (Ubuntu)**.

### Prerequisites
- **Runtime:** Node.js v20+ / pnpm v9+
- **Kernel:** Rust 2024 Edition (Cargo 1.85+)
- **Process Manager:** PM2 (Industrial Grade)
- **Encryption Seed:** Valid `ORACLE_TOKEN`

## 2. 🚀 Node Initialization
To manifest a node on a new device, follow the elite boot sequence:

```bash
# 1. Clone and authenticate
git clone https://github.com/Rexumerr/MAS-Protocol.git -b develop
cd MAS-Protocol

# 2. Setup environment (One-time manual step for cloud gate)
gh auth login

# 3. Elite Boot (Automation takes over)
./VANGUARD_BOOT.sh
```

The boot loader now incorporates **Apex Security Auto-Handshake**, which automatically generates and synchronizes `ORACLE_TOKEN` with the GitHub cloud repository.

## 3. 🛡️ Security & Sovereignty
### 3.1 Hardware-Bound Encryption
Every node's state is cryptographically bound to the host chip.
- **Boot ID Binding:** The `NeuralCrypt` module uses `/proc/sys/kernel/random/boot_id`.
- **Portability Note:** Encrypted state files (`multiverse-state.json`) **cannot** be decrypted on secondary hardware.

### 3.3 Autonomous Self-Healing (mas-sentinel)
The system employs an autonomous sentinel agent to ensure process integrity.
- **Monitoring:** The `mas-sentinel` scans the PM2 process ecosystem every 10 seconds.
- **Recovery:** If any core service (Monitor, Sync, Dashboard) transitions to a state other than `online`, the sentinel autonomously triggers a recovery restart.
- **Resilience:** This closed-loop system maintains 99.9% uptime by eliminating the need for manual intervention during transient failures.

## 4. 📈 Proactive Monitoring
Use the **Vanguard Governor** to audit health trends in real-time:

```bash
# Monitor CPU/RAM trends and Thermal Protection
./packages/vanguard-os/target/release/vanguard-os monitor
```

- **Eco Mode:** Automatic throttling at >60% resource usage.
- **Neural Cooling:** Predictive task suspension if projected temp >45°C.

## 5. 🧬 Multiversal Synchronization
Sync cycles are managed by PM2 every 12 hours.
- **Store-and-Forward:** If WAN is offline, events are journaled in `*.journal`.
- **Atomic Sync:** Once online, the `sync-orchestrator` flushes logs to the global hive.

---
*Authorized by Rexumer Signature | Internal Document v1.0*
