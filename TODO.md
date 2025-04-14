# rustake: Future Enhancements & TODOs

This document outlines features and enhancements that are planned or under consideration for the `rustake` project. These items are not critical for initial functionality but represent areas of future polish, improvements, or expansion.

## ✅ Completed
- [x] Basic Axum HTTP server
- [x] Initial tracing setup with `tracing` and `tracing-subscriber`
- [x] `.env` integration for setting log level and other simple features

## 🚧 In Progress
- [ ] Build out staking/unstaking and reward logic
- [ ] Connect SQLx with database and define schemas

## 🔜 Planned
- [ ] Add support for JSON or span-rich formatted tracing output
- [ ] Introduce `.env` toggles for log verbosity (e.g. `TRACE` level)
- [ ] Add CLI or UI toggles for dynamic log filtering
- [ ] Set up CI and/or pre-commit hooks for linting, formatting, etc.
- [ ] Add JWT authentication logic and middleware wiring
- [ ] Connect Axum routes to real business logic (Rust services)
- [ ] Set up metrics endpoint with Prometheus
- [ ] Add OpenAPI auto-generated docs with `utoipa`

## 💡 Nice-to-Have Ideas
- [ ] Add CLI tool for local validator/debugging tasks
- [ ] Explore async job scheduling or background processing
- [ ] Dockerize full stack (backend + frontend)
- [ ] Add a traffic simulation tool for testing reward distribution
- [ ] Make backend composable as a library for future CLI/UI uses

## 🪪 Project Polish / Community

- [ ] Add status badges to `README.md` (build, license, crates.io, docs.rs, etc.)
- [ ] Include a mini roadmap in `README.md` for quick visibility
- [ ] Improve onboarding by adding a `CONTRIBUTING.md` guide (eventually)
- [ ] Add repo topics and metadata for discoverability on GitHub


---
This list is a living document and will be updated as the project evolves.
