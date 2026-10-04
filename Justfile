export DATABASE_URL := "sqlite:tether.db"
export RUST_LOG := "debug"

# (list available commands)
default:
    @just --list

# Run database migrations
migrate:
    cargo sqlx migrate run

# Revert the last migration
revert:
    cargo sqlx migrate revert

# Open the database in the SQLite CLI
console:
    sqlite3 tether.db

# Build and run the proxy
run:
    cargo run
