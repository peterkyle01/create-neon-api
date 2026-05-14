# Neon API

Rust backend wired for the [Neon Data API](https://neon.tech/docs/data-api/overview) — serverless
Postgres over HTTP.

Uses Axum for the web layer, JWT for auth (tokens carry a `role` claim so they work
directly with PostgREST RLS policies), and Argon2id for password hashing.  All
database operations go through the Neon Data API HTTP endpoint — no driver, no
connection pool, no ORM.

## Project structure

```
src/
├── main.rs           # server setup, routes
├── config.rs         # env configuration
├── auth.rs           # JWT creation/verification, Argon2id hashing
├── data_api.rs       # HTTP client for the Neon Data API
├── errors.rs         # error types + axum IntoResponse
├── models.rs         # request / response types, JWT claims
├── handlers/
│   ├── auth.rs       # POST /signup, POST /login
│   └── user.rs       # GET /me
├── middleware/
│   └── auth.rs       # Bearer-token verification
└── lib.rs
├── justfile
├── Cargo.toml
├── .env.example
migrations/
└── schema.sql        # users table + RLS policies
```

## Setup

### 1. Neon project

Create a project at [console.neon.tech](https://console.neon.tech).  Copy the
connection string and enable the Data API from the project settings.  Grab the
Data API URL (it ends in `/rest/v1`).

### 2. Environment

```bash
cp .env.example .env
```

Fill in:

```ini
DATABASE_URL=postgresql://user:pass@ep-....aws.neon.tech/neondb?sslmode=require
NEON_DATA_API_URL=https://ep-....aws.neon.tech/neondb/rest/v1
JWT_SECRET=<generate with: openssl rand -base64 32>
JWT_EXPIRY_HOURS=24
PORT=8080
HOST=0.0.0.0
RUST_LOG=neon_api=debug,tower_http=debug
```

### 3. Database schema

Paste `migrations/schema.sql` into the Neon SQL Editor, or run:

```bash
psql "$DATABASE_URL" -f migrations/schema.sql
```

### 4. Run

```bash
cargo run
# → http://localhost:8080
```

## API

### Public

```bash
# health
curl http://localhost:8080/health

# signup
curl -X POST http://localhost:8080/signup \
  -H "Content-Type: application/json" \
  -d '{"email":"alice@example.com","password":"s3cret!234","first_name":"Alice","last_name":"Smith"}'

# login
curl -X POST http://localhost:8080/login \
  -H "Content-Type: application/json" \
  -d '{"email":"alice@example.com","password":"s3cret!234"}'
```

### Protected

```bash
curl http://localhost:8080/me \
  -H "Authorization: Bearer <token>"
```

## Configuration

| Variable            | Description                 | Default    |
| ------------------- | --------------------------- | ---------- |
| `DATABASE_URL`      | Neon connection string      | *required* |
| `NEON_DATA_API_URL` | Data API base URL           | *required* |
| `JWT_SECRET`        | JWT signing key (32+ chars) | *required* |
| `JWT_EXPIRY_HOURS`  | Token lifetime in hours     | `24`       |
| `PORT`              | Listen port                 | `8080`     |
| `HOST`              | Listen address              | `0.0.0.0`  |
| `RUST_LOG`          | Tracing filter              | `info`     |

## Just targets

```bash
just build      # compile
just run        # start dev server
just test       # run tests
just fmt        # format
just lint       # clippy
just check      # fmt + lint + test
just release    # optimised build
just migrate    # print schema instructions
```

## How it works

```
Client
  │  POST /signup, POST /login, GET /me
  ▼
Axum (this server)
  │  reqwest (HTTP)
  ▼
Neon Data API (PostgREST)
  │
  ▼
Neon Serverless Postgres
```

The JWT issued at login includes `"role": "authenticated"`.  When passed to the
Data API, PostgREST validates the token and enforces the Row-Level Security
policies defined in `migrations/schema.sql`.

## License

MIT — see [LICENSE](LICENSE).
