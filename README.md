# rustake
Built with 💙 by a full-stack engineer learning in public.

🚀 A fast, modular, and modern staking backend written in Rust with [Axum](https://github.com/tokio-rs/axum), [SQLx](https://github.com/launchbadge/sqlx), and [ethers-rs](https://github.com/gakonst/ethers-rs).

`rustake` is a sandbox project exploring blockchain staking logic, off-chain computation, and system design in Rust. It aims to be production-grade, with an emphasis on clean architecture, observability, and extensibility.

## Features

- 🌐 Axum HTTP API framework
- 🧠 Modular logic for staking, unstaking, and reward calculation
- 📦 SQLx integration for PostgreSQL
- 🔐 JWT auth (planned)
- 📊 Prometheus metrics (planned)
- 📜 Auto-generated OpenAPI docs via Utoipa (planned)
- 🛠️ Tracing via `tracing` & `tracing-subscriber` with `.env`-configurable log levels
- 🎨 Startup ASCII header + TODO preview (can be toggled via env vars)

## Quickstart

```bash
# Run the dev server
cargo run
```

## Environment Variables

These are read from your local `.env` file (or system env):

| Variable                 | Purpose                                      |
|--------------------------|----------------------------------------------|
| `RUSTAKE_LOG`            | Log level (`info`, `debug`, `trace`, etc.)  |
| `RUSTAKE_TODO`           | If set, prints the TODO list at startup     |
| `RUSTAKE_DISABLE_ASCII` | If set, disables the ASCII header banner     |

## TODO

See [`TODO.md`](./TODO.md) for upcoming features, nice-to-haves, and roadmap progress.

