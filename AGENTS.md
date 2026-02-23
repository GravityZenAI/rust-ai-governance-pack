# AGENTS.md — Rust Development Rules
# Universal instructions for ALL AI coding agents
# Compatible with: Claude Code, Cursor, Windsurf, Copilot, Codex, Gemini CLI,
# Aider, Zed, Amp, Cline, and any AGENTS.md-compliant agent.
# Source: GravityZen AI — CC BY 4.0

## Identity
This is a Rust project. All code MUST compile with `cargo check`, pass
`cargo clippy --all-targets -- -D warnings`, and be formatted with `cargo fmt`.

## Compilation is LAW
- Run `cargo check` after EVERY change to .rs files
- Run `cargo clippy --all-targets -- -D warnings` before marking any task complete
- Run `cargo fmt` before committing
- NEVER deliver code that doesn't compile
- Read compiler errors COMPLETELY — Rust tells you exactly what's wrong and how to fix it
- Fix errors starting from the FIRST one (later errors are often caused by earlier ones)

## Dependencies — VERIFY BEFORE USING
- ALWAYS verify a crate exists on crates.io BEFORE adding it to the project
- Use `cargo add <crate>` to add dependencies — NEVER edit Cargo.toml manually
- Preferred crate ecosystem:
  - Async runtime: `tokio` (with explicit features: rt-multi-thread, macros, full)
  - HTTP server: `axum`
  - HTTP client: `reqwest`
  - Serialization: `serde` + `serde_json`
  - Error handling (libraries): `thiserror`
  - Error handling (applications): `anyhow`
  - CLI: `clap` (with derive feature)
  - Logging/tracing: `tracing` + `tracing-subscriber`
  - Testing: built-in + `tokio::test` for async + `proptest` for property-based

## Ownership & Borrowing
- Prefer `&T` (borrow) over `T` (move) in function parameters
- Prefer `&str` over `String` in function parameters
- Prefer `impl Trait` over generic `<T: Trait>` when there's only one trait bound
- NEVER use `.clone()` just to satisfy the borrow checker — restructure code instead
- When the borrow checker complains, try solutions in this EXACT order:
  1. Adjust scope (move code so borrows don't overlap)
  2. Use references only when needed, drop them quickly
  3. Clone ONLY if data is small (<100 bytes) or genuinely needed
  4. Restructure the algorithm (separate read phase from write phase)
  5. Smart pointers: `Rc<T>` (single-thread) or `Arc<T>` (multi-thread)
  6. Interior mutability: `RefCell<T>` or `Mutex<T>` as LAST resort

## Error Handling
- ALWAYS use `Result<T, E>` for operations that can fail
- NEVER use `.unwrap()` in production code
- Use `.expect("descriptive message")` ONLY for programming errors / invariants
- Use `?` operator for error propagation
- Use `thiserror` for library error types, `anyhow` for application errors
- Error messages: lowercase, no trailing punctuation
- Chain errors with `.context()` or `.with_context()` for debuggability

## Patterns to ALWAYS Use
- `std::sync::OnceLock` instead of `lazy_static!`
- HashMap Entry API: `.entry(k).or_insert(v)` instead of `contains_key` + `insert`
- Collect-then-mutate instead of mutating during iteration
- `tokio::sync::Mutex` (not `std::sync::Mutex`) in async code
- `impl Display` and `impl Debug` for all custom types

## What NEVER to Do
- NEVER invent crate names — verify on crates.io first
- NEVER use `unsafe` unless absolutely necessary; document with `// SAFETY: reason`
- NEVER use `#[allow(unused)]` to hide problems
- NEVER use `Box<dyn Error>` as error type in libraries — use `thiserror`
- NEVER ignore clippy warnings — fix them or justify suppression with comment
- NEVER use `lazy_static!` — use `std::sync::OnceLock` (Rust 1.80+)
- NEVER create functions longer than 50 lines — extract helper functions
- NEVER hardcode secrets, keys, or tokens in source code

## Style & Naming
- `snake_case` for functions, variables, modules
- `PascalCase` for types, traits, enums, enum variants
- `SCREAMING_SNAKE_CASE` for constants and statics
- Max 100 characters per line
- One module per file, files under 300 lines
- Document ALL public items with `///` doc comments

## Documentation
- Every public function: `///` with description
- Include `# Examples` section with compilable code
- Include `# Errors` section when function returns Result
- Include `# Panics` section if function can panic
- Include `# Safety` section for any unsafe code
- Module-level docs with `//!`

## Testing
- Write tests BEFORE implementation when possible (TDD)
- Unit tests in same file: `#[cfg(test)] mod tests { ... }`
- Integration tests in `tests/` directory
- Run `cargo test` after every implementation
- Never hardcode expected values that bypass actual logic verification
- Use `#[should_panic]` for expected panic tests
- Use `proptest` for property-based testing on complex logic

## Async Patterns
- Use `tokio` as the async runtime (not `async-std`)
- Every async function doing I/O should return `Result`
- Use `tokio::sync::mpsc` for multi-producer channels
- Use `tokio::sync::oneshot` for single-response channels
- Always `.await` futures — a future that isn't awaited does NOTHING
- Use `tokio::select!` for racing multiple futures
- Add timeouts to all network operations

## Project Structure
- `src/lib.rs` for libraries, `src/main.rs` for binaries
- Cargo workspace for multi-crate projects
- Shared dependencies in `[workspace.dependencies]`
- Integration tests in `tests/`
- Examples in `examples/`
- Benchmarks in `benches/`

## Security
- Validate ALL external input before processing
- Run `cargo audit` to check for known vulnerabilities
- Never implement custom cryptography — use `ring` or `rustls`
- Never log secrets, tokens, passwords, or PII
- Pin dependency versions (commit Cargo.lock for binaries)
- Use `secrecy::Secret<T>` for sensitive data

## Verification Checklist (run before completing ANY task)
1. `cargo check --workspace` — compiles
2. `cargo clippy --all-targets -- -D warnings` — no warnings
3. `cargo fmt --check` — properly formatted
4. `cargo test --workspace` — all tests pass
5. No `.unwrap()` in non-test code
6. No invented crate names
7. No `unsafe` without `// SAFETY:` comment
