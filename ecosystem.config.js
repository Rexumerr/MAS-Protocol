module.exports = {
  apps: [
    {
      name: "vanguard-monitor",
      script: "./packages/vanguard-os/target/release/vanguard-os",
      args: "monitor",
      interpreter: "none",
      autorestart: true,
      watch: false,
      max_memory_restart: "200M",
      env: {
        NODE_ENV: "production",
      }
    },
    {
      name: "astro-dashboard",
      script: "pnpm",
      args: "run dev",
      cwd: "./apps/web",
      autorestart: true,
      env: {
        PORT: 3000
      }
    },
    {
      name: "neural-sync",
      script: "pnpm",
      args: "run sync",
      autorestart: false,
      cron_restart: "0 */12 * * *",
      env: {
        NODE_ENV: "production"
      }
    },
    {
      name: "sync-orchestrator",
      script: "python3",
      args: "scripts/sync-orchestrator.py",
      autorestart: true,
      watch: false,
      env: {
        NODE_ENV: "production"
      }
    }
  ]
};
