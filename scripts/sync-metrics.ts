import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

/**
 * Sync Metrics Script (Rexumer Edition)
 * Implementación asíncrona para optimizar la ingesta masiva de eventos.
 */

async function runBrainCli(event: string, value: number) {
    const token = process.env.ORACLE_TOKEN || "development-only-key";
    const binPath = "./packages/core-rs/target/release/brain-cli";
    const statePath = "./apps/web/src/data/multiverse-state.json";

    try {
        console.log(`[*] Ingesting event: ${event} (Value: ${value})...`);
        // Ejecución asíncrona para permitir multitasking de eventos
        const { stdout, stderr } = await execAsync(
            `${binPath} ingest --event "${event}" --value ${value} --state-path "${statePath}" --token "${token}"`,
            { env: { ...process.env, ORACLE_TOKEN: token } }
        );
        if (stdout) console.log(`[+] ${stdout.trim()}`);
        if (stderr) console.error(`[!] ${stderr.trim()}`);
    } catch (error: any) {
        console.error(`[CRITICAL] Failed to ingest ${event}:`, error.message);
        // No lanzamos el error para permitir que otros eventos se procesen
    }
}

async function main() {
    console.log("\n🌌 --- STARTING MULTIVERSAL SYNC (ASYNC MODE) ---");
    const startTime = Date.now();

    const tasks: Promise<void>[] = [];

    // 1. Optimización de pauta (Ad Spend)
    const adOptimized = Math.floor(Math.random() * 1000) + 100;
    tasks.push(runBrainCli("ad_spend_optimized", adOptimized));

    // 2. Simular nuevos leads (Multitasking)
    const leads = Math.floor(Math.random() * 5) + 1;
    for (let i = 0; i < leads; i++) {
        tasks.push(runBrainCli("new_lead_generated", 1));
    }

    // 3. Éxito en automatizaciones
    tasks.push(runBrainCli("automation_success", 1));

    // Ejecución paralela de todas las ingestas
    await Promise.all(tasks);

    const duration = ((Date.now() - startTime) / 1000).toFixed(2);
    console.log(`\n✅ --- SYNC COMPLETED IN ${duration}s ---\n`);
}

main().catch(err => {
    console.error("FATAL ERROR IN SYNC:", err);
    process.exit(1);
});
