---
name: rust-development
description: >-
  Use this skill when writing, refactoring, or debugging Rust code, or when the user needs help with Cargo, lifetimes, or async Rust.
---

# Rust Development Guidelines

When working on Rust code in this workspace, strictly adhere to the following principles, which are based on the most popular community guidelines for AI-assisted Rust development:

## 1. Idiomatic Error Handling
- **Never use `unwrap()` or `expect()`** unless explicitly proving an invariant. Always prefer `Result<T, E>` and the `?` operator.
- Use `anyhow` for application-level errors and ensure error messages are descriptive and actionable.

## 2. Ownership, Borrowing, and Lifetimes
- When designing complex data structures, briefly explain your reasoning regarding ownership and lifetime annotations before writing the code.
- Leverage the type system to make invalid states unrepresentable.

## 3. Async & Concurrency
- Since the project uses `tokio`, adhere strictly to idiomatic async patterns: spawn tasks properly, use `JoinSet` for concurrent execution, and **never** use blocking operations inside async contexts.
- Use `Arc` and `Mutex`/`RwLock` judiciously for shared state (e.g., Axum `AppState`).

## 4. Tooling & Zero Warnings
- Write code that passes `cargo clippy` with **zero warnings**.
- When generating new code, mentally apply `cargo fmt` formatting standards.
- For database queries (`sqlx`), prefer `sqlx::query!` macros for compile-time safety on standard tables, but fallback to `sqlx::query()` for SQLite virtual tables (`sqlite-vec`) that aren't loaded at compile-time.

## 5. Modular Organization
- Avoid dumping everything into `main.rs` or a single file. Favor feature-driven modules and standard Rust directory structures.
