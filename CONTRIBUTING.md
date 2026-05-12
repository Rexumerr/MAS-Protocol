# 🤝 Contributing to MAS Protocol

We're excited that you're interested in contributing to the **MAS Protocol**! As an elite orchestration ecosystem, we hold our code and architecture to the highest standards.

## 📜 Code of Conduct
We follow a professional and excellence-driven code of conduct. Respect the architecture, follow the patterns, and strive for 10/10 quality.

## 🛠️ Development Workflow

1.  **Fork & Clone:** Work on your own branch.
2.  **Rust Standards:** All core logic must be written in idiomatic Rust. Run `cargo fmt` and `cargo clippy` before submitting.
3.  **WASM First:** Ensure all core changes are compatible with the `wasm32-unknown-unknown` target.
4.  **Conventional Commits:** Use clear, semantic commit messages (e.g., `feat:`, `fix:`, `docs:`, `refactor:`).

## 🧪 Testing
Every capability change must include corresponding unit tests in `packages/core-rs/src/tests.rs` (if applicable) and be verified through the `turbo test` suite.

## 🚀 Submitting Pull Requests
- Provide a clear description of the change.
- Link any related issues or "Quests".
- Ensure the CI/CD pipeline passes.

Thank you for helping us build the future of autonomous scaling!
