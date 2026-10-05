# Tether

Tether is a high-performance, single-binary LLM proxy and semantic caching engine. It is designed to intercept LLM API requests, cache responses using local vector search, and track document dependencies to prevent stale cache poisoning. Built for local-first, plug-and-play deployment.

## Core Features

- **Single-Binary Architecture:** Statically linked SQLite and `sqlite-vec` engine. No external database servers or shared objects required.
- **Local Semantic Routing:** In-memory vector generation using `fastembed` (BAAI/bge-small-en-v1.5, 384-dimensional) to calculate query similarity without hitting external APIs.
- **Tripartite Caching Schema:** A relational structure tracking `cached_queries`, `cached_responses`, and `document_dependencies` to ensure cache validity when underlying RAG chunks are updated.
- **High Performance:** Written in Rust, utilizing Axum for asynchronous HTTP handling and SQLx for connection pooling.

## Tech Stack

- **Language:** Rust
- **Web Framework:** Axum, Tower
- **Database:** SQLite via SQLx
- **Vector Engine:** `sqlite-vec` (statically linked via C-FFI)
- **Embeddings:** `fastembed`
- **Task Runner:** `just`

## Getting Started

### Prerequisites

You will need Rust installed along with the `just` command runner and the SQLx CLI.

```bash
cargo install just
cargo install sqlx-cli --no-default-features --features rustls,sqlite
```

### Running Locally

Tether comes with a `Justfile` to simplify common development tasks. The default `DATABASE_URL` is set to `sqlite:tether.db` automatically when using `just`.

To build and run the proxy server:

```bash
just run
```

This will start the Axum web server and initialize the local vector database.

### Database Management

Tether uses SQLite. You can use the following `just` commands for database management:

- `just migrate` - Runs pending SQLx migrations.
- `just revert` - Reverts the last migration.
- `just console` - Opens the SQLite CLI for `tether.db`.

### Example Usage

Once the server is running, you can test the chat completions proxy endpoint using `curl`:

```bash
curl -X POST http://localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model_slug": "gpt-4o-mini",
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```
