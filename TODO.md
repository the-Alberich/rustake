# rustake: Future Enhancements & TODOs

This document outlines features and enhancements that are planned or under consideration for the `rustake` project. These items are not critical for initial functionality but represent areas of future polish, improvements, or expansion.

## ✅ Completed
- [x] Basic Axum HTTP server
- [x] Initial tracing setup with `tracing` and `tracing-subscriber`
- [x] `.env` integration for setting log level
- [x] ASCII banner and TODO.md preview logic
- [x] README.md with environment variable table and project overview
- [x] Wire up Ethereum provider with `ethers-rs` and check Hardhat RPC connectivity
- [x] Basic staking/unstaking and reward logic
- [x] Connect Axum routes to contract methods (Rust services)
- [+] State Management
    - [x] Introduce an `AppState` struct to hold shared resources:
      - [x] Ethereum provider instance
      - [x] Contract interface (or builder function)
    - [x] Use `.with_state(app_state)` on the Axum router

## 🚧 In Progress
- [ ] Update README.md (need to include more details on startup, configuring via .env or CLI, usage via curl or Postman)
- [ ] Unit tests

## 🔜 Planned
- [ ] Improve Blockchain Integration
    - [ ] Update the process to ensure the compiled contract ABI is automatically refreshed in the project directory when the blockchain contract is compiled or built.
    - [ ] Implement a mechanism to share the interface between the blockchain and the Rust API, improving the workflow between the two separate repos.
    - [ ] Expose API endpoint(s) or signal handler(s) to trigger hot reload for contract changes or signer changes.
    - [ ] System signer(s) for contract deployment, pre-funding, dry runs, validations, and more.
- [ ] Add support for JSON or span-rich formatted tracing output
- [ ] Add CLI or UI toggles for dynamic log filtering
- [ ] Set up CI and/or pre-commit hooks for linting, formatting, etc.
- [ ] Add JWT authentication logic and middleware wiring
- [ ] Set up metrics endpoint with Prometheus
- [ ] Add OpenAPI auto-generated docs with `utoipa`
- [ ] State Management
    - [ ] Improve `AppState` struct to hold shared resources:
      - [ ] Possibly metrics registry, config, or DB connection later
    - [ ] Extract and inject state using `State<AppState>` in handlers
    - [ ] Improve /health endpoint
      - [ ] ETH blockchain connectivity check
      - [ ] Contract availability check
    - [ ] Consider introducing helper macros or internal caching utilities to simplify and standardize shared mutable state patterns (e.g., `Arc<RwLock<Option<T>>>` for contract caching).
- [ ] Frontend (start by defining tech stack and game plan)


## 💡 Nice-to-Have Ideas
- [ ] Improve Blockchain Integration
    - [ ] Allow users to define the contract address they wish to interact with, either through configuration or as a parameter when making requests to the backend. This would allow flexibility for different users to connect to different instances or addresses.
- [ ] Add CLI tool for local validator/debugging tasks
- [ ] Explore async job scheduling or background processing
- [ ] Dockerize full stack (backend + frontend)
- [ ] Add a traffic simulation tool for testing reward distribution
- [ ] Make backend composable as a library for future CLI/UI uses
- [ ] Explore usefulness of SQLx with database and define schemas

## 🪪 Project Polish / Community
- [ ] Add status badges to `README.md` (build, license, crates.io, docs.rs, etc.)
- [ ] Include a mini roadmap in `README.md` for quick visibility
- [ ] Improve onboarding by adding a `CONTRIBUTING.md` guide (eventually)
- [ ] Add repo topics and metadata for discoverability on GitHub

---
This list is a living document and will be updated as the project evolves.
