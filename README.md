# create-neon-api

[![Crates.io](https://img.shields.io/crates/v/create-neon-api.svg)](https://crates.io/crates/create-neon-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Scaffold a Rust API backend wired for [Neon](https://neon.tech) in a single command —
no network needed, the template is embedded in the binary.

The generated project uses **Axum** for the web framework, **Neon Auth** for authentication,
and the **Neon Data API** for CRUD operations — all over HTTP. No ORM, no connection pool,
no Docker, no system OpenSSL (uses `rustls`).

> **⚠️ Not production-ready out of the box.** This is a starter template intended
> to accelerate development. Before deploying to production, review authentication,
> authorization, rate limiting, input validation, error handling, secrets management,
> and security hardening for your specific use case.

## Features

- **Axum web framework** – fast, ergonomic, and async-first
- **Neon Auth** – sign-up, sign-in, sign-out, and JWT-based session management
- **Neon Data API CRUD** – generic `create`/`get_all`/`get_one`/`update`/`delete` that works for any table
- **`NeonClient` extractor** – automatically pulls the JWT from `Authorization: Bearer` via Axum's `FromRequestParts`
- **Auto-generated types** – `utility-types` reduces boilerplate (e.g. `SignInRequest` derived from `SignUpRequest`)
- **Standard API envelope** – all responses follow `{ "data": ... }` / `{ "error": { "code": "...", "message": "..." } }`
- **Smart health checks** – verifies both Auth and Data API endpoints are reachable
- **Level-based logging** – `INFO` for 2xx, `WARN` for 4xx, `ERROR` for 5xx
- **Comprehensive tests** – integration tests covering the full CRUD flow and error scenarios

## Install

```bash
cargo install create-neon-api
```

## Usage

```bash
create-neon-api                # interactive prompt
create-neon-api my-api         # scaffold directly
create-neon-api my-api -B      # skip cargo build
create-neon-api my-api -q      # quiet (scripts / CI)
```

## What you get

```
my-api/
├── src/
│   ├── main.rs            # Axum server entry point
│   ├── lib.rs             # Router, routes(), TraceLayer
│   ├── response.rs        # Standard API envelope, AppError, helpers
│   ├── config/
│   │   ├── mod.rs         # Config struct (AUTH_URL, DATA_API_URL, etc.)
│   │   └── client.rs      # NeonClient — auth + CRUD over HTTP
│   └── handlers/
│       ├── mod.rs
│       ├── auth.rs        # sign_up, sign_in, sign_out, get_me
│       ├── notes.rs       # Note CRUD handlers
│       └── health.rs      # Health check with component status
├── tests/
│   └── api.rs             # Full integration tests
├── Cargo.toml
├── .env.example
└── LICENSE
```

## After scaffolding

```bash
cd my-api
cp .env.example .env        # add your Neon credentials
# create the notes table in your Neon SQL editor
cargo run                   # → http://localhost:8080
```

## API Endpoints (generated project)

| Method   | Path                     | Auth   | Description       |
|----------|--------------------------|--------|-------------------|
| GET      | `/health`                | public | health check      |
| POST     | `/api/v1/auth/sign-up`   | public | register          |
| POST     | `/api/v1/auth/sign-in`   | public | sign in           |
| POST     | `/api/v1/auth/sign-out`  | Bearer | sign out          |
| GET      | `/api/v1/auth/me`        | Bearer | JWT payload info  |
| GET      | `/api/v1/notes`          | Bearer | list notes        |
| POST     | `/api/v1/notes`          | Bearer | create note       |
| GET      | `/api/v1/notes/{id}`     | Bearer | get note          |
| PATCH    | `/api/v1/notes/{id}`     | Bearer | update note       |
| DELETE   | `/api/v1/notes/{id}`     | Bearer | delete note       |

## How it works

```
create-neon-api my-api
        │
        ▼
   ┌─────────────────────────────┐
   │  Embedded template          │
   │  (include_dir! macro)       │
   └─────────┬───────────────────┘
             │ extract to ./my-api
             ▼
   ┌─────────────────────────────┐
   │  1. Rename Cargo.toml       │
   │  2. Replace neon_api_app    │
   │     placeholders in all     │
   │     .rs & .toml files       │
   │  3. Update Cargo.toml name  │
   │  4. cargo build (optional)  │
   └─────────────────────────────┘
```

## License

MIT — see [LICENSE](LICENSE).
