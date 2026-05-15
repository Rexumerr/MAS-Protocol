# 🏛️ PROJECT GOVERNANCE: MAS-Protocol

## 🛠️ High-Discipline Workflow (Mandatory)

1.  **Branching Strategy:** 
    *   `main`: Production-ready, stable, and verified.
    *   `develop`: Integration branch for new features.
    *   `feature/*`: Granular branches for specific tasks.
2.  **Pull Requests:** 
    *   NO direct commits to `main` or `develop`. 
    *   All changes must go through a PR and pass the **Build-Check** job.
3.  **Issue Tracking:**
    *   If it isn't an Issue, it doesn't exist. All tasks, bugs, and ideas must be documented.
4.  **CI/CD Stability:**
    *   A failing pipeline is a **Blocker**. No new features until the pipeline is green.

---
*Authorized by Rexumer Signature - May 15, 2026*
