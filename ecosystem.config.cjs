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
        PORT: 3000,
        ORACLE_TOKEN: process.env.ORACLE_TOKEN
      }
    },
    {
      name: "neural-sync",
      script: "pnpm",
      args: "run sync",
      autorestart: false,
      cron_restart: "0 */12 * * *",
      env: {
        NODE_ENV: "production",
        ORACLE_TOKEN: process.env.ORACLE_TOKEN
      }
    },
    {
      name: "sync-orchestrator",
      script: "python3",
      args: "scripts/sync-orchestrator.py",
      autorestart: true,
      watch: false,
      env: {
        NODE_ENV: "production",
        ORACLE_TOKEN: process.env.ORACLE_TOKEN
      }
    },
    {
      name: "mas-sentinel",
      script: "./packages/vanguard-os/target/release/sentinel",
      interpreter: "none",
      autorestart: true,
      env: {
        NODE_ENV: "production",
        ORACLE_TOKEN: process.env.ORACLE_TOKEN
      }
    },
    {
      name: "power-leveling",
      script: "python3",
      args: "scripts/power-leveling.py",
      autorestart: false,
      env: {
        NODE_ENV: "production",
        ORACLE_TOKEN: process.env.ORACLE_TOKEN
      }
    }
  ]
};
