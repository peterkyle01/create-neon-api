# create-neon-api

[![Crates.io](https://img.shields.io/crates/v/create-neon-api.svg)](https://crates.io/crates/create-neon-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Scaffold a Rust backend wired for the Neon Data API in a single command.

The generated project includes JWT authentication (compatible with PostgREST
Row-Level Security), Argon2id password hashing, and an HTTP client pre-configured
for the Neon Data API.  No ORM, no Docker, no connection pools — just HTTP to
your serverless Postgres.

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
│   ├── main.rs            # Axum server, routes
│   ├── config.rs          # env-based configuration
│   ├── auth.rs            # JWT + Argon2id
│   ├── data_api.rs        # Neon Data API HTTP client
│   ├── errors.rs          # unified error type
│   ├── models.rs          # request / response types
│   ├── handlers/          # signup, login, me
│   └── middleware/        # JWT verification
├── migrations/            # SQL schema + RLS policies
├── Cargo.toml
├── justfile
├── .env.example
└── LICENSE
```

## After scaffolding

```bash
cd your-project
cp .env.example .env    # add your Neon credentials
# run migrations/schema.sql in the Neon SQL Editor
cargo run               # → http://localhost:8080
```

## Endpoints

| Method | Path      | Auth   | Description  |
| ------ | --------- | ------ | ------------ |
| GET    | /health   | public | health check |
| POST   | /signup   | public | create user  |
| POST   | /login    | public | get JWT      |
| GET    | /me       | Bearer | user profile |

## License

MIT — see [LICENSE](LICENSE).
