## Build tools & versions used

| Tool          | Version |
|---------------|---------|
| rustc / cargo | 1.98.1  |
| Docker        | 29.5.1  |
| PostgreSQL    | 17      |

Key crates:

- [axum](https://crates.io/crates/axum) 0.8 - HTTP framework
- [sqlx](https://crates.io/crates/sqlx) 0.9 - Postgres driver + migrations
- [tokio](https://crates.io/crates/tokio) 1.53 - async runtime
- [reqwest](https://crates.io/crates/reqwest) 0.13 - mempool API client
- [chrono](https://crates.io/crates/chrono) 0.4 - timestamp handling
- [tracing](https://crates.io/crates/tracing) - structured logging
- [thiserror](https://crates.io/crates/thiserror) - error types

## Steps to run the app

### With Docker Compose (recommended)

```sh
docker compose up --build
```

This starts Postgres and the app. The app is available at `localhost:3000`:

```sh
curl localhost:3000/nodes
```

### Without Docker

Install Rust and PostgreSQL, create a database (or run with), then run:

```sh
DATABASE_URL=your-database-url cargo run
```

Migrations in `db/migrations` are applied automatically at startup.

### Running the tests

```sh
cargo test
```

### Environment variables

- `DATABASE_URL` (required): Postgres connection string. The app fails to start if it is not set.
- `LISTEN_ADDR`: address the HTTP server binds to.
- `UPDATE_INTERVAL_SECS`: import interval in seconds, clamped to a minimum of `1`.
- `MEMPOOL_URL`: mempool endpoint.

## What was the reason for your focus? What problems were you trying to solve?

Making sure serialization and deserialization of data was correct.

Guaranteeing correctness of data using newtypes, having everything in the nodes be integers and
strings could very easily lead to invariant violation.

Making the import loop fail-safe.

## How long did you spend on this project?

About two days.

## Did you make any trade-offs for this project? What would you have done differently with more time?

With more time I would add integration tests, these tests would require a little more separation
between the modules which would be nice. I'd also store in the db unused data from each node such
as `updatedAt`, `country` and `city` as plain text or jsonb so it can be retrieved later if needed.

## What do you think is the weakest part of your project?

No integration tests and no backoff strategy for the polling loop, the retry interval should not be
the same as the regular fetch interval.

## Is there any other information you'd like us to know?

I focused on correctness and reliability over scope, I wanted the required features to be robust
before trying to implement extra features.

I used rebase to fix a typo in a commit message so some timestamps may be inaccurate.
