# RUST_PLAYBOOK (persistent)

This file is indexed as knowledge for the IDE agent.
Based on verified research (ICSE 2025, JetBrains, ANSSI, Rust official docs).

## Non-negotiables
- No fabricated outputs.
- No `unsafe` unless required; isolate + document `# Safety`.
- No new dependencies without audit/deny + written rationale.
- Done means verifier GREEN.

## Iterative Compiler Loop
Use compilation errors as a teacher: extract error → analyze context → fix → recompile → repeat until green. This approach achieves ~74% correct fixes in real benchmarks (RustAssistant, ICSE 2025).

When stuck on an error code:
```bash
rustc --explain E0382   # Replace with actual error code
```

## Ownership and Borrowing
- Every value has exactly one owner.
- After a `move`, the original variable is invalid (E0382).
- Prefer `&T` (immutable borrow) over cloning.
- `&mut T` requires exclusive access — no other borrows active.
- Anti-pattern: modifying a value while immutable borrows exist (E0502).

```rust
// ❌ Use after move
let s = String::from("hello");
let s2 = s;
println!("{s}");  // E0382: value used after move

// ✅ Borrow instead
let s = String::from("hello");
let s2 = &s;
println!("{s}");  // Works: s is still valid
```

## Lifetimes
- Annotate lifetimes when returning references from functions.
- E0106: missing lifetime specifier → add `'a` annotation.
- E0515: returning reference to local → restructure to return owned data.
- Avoid `unsafe` to extend lifetimes without justification.

## Traits and Generics
- Common traits: `Debug`, `Clone`, `Display`, `From/Into`, `Default`.
- E0277 (~32% of errors): trait not implemented → derive or implement it.
- E0599 (~27.5%): method not found → check if you need `.iter()` or a trait import.
- Prefer clear trait bounds: `fn foo<T: Clone + Debug>(x: T)`.

## Error Handling (idiomatic)
- Use `Result<T, E>` and `Option<T>`, never `unwrap()` in production.
- Propagate with `?` operator.
- Define small error enums with `thiserror` (if approved as dependency).
- Add context at API boundaries, not everywhere.

```rust
// ❌ Dangerous
let value = parse_input(s).unwrap();

// ✅ Idiomatic
let value = parse_input(s)?;

// ✅ With context
let value = parse_input(s)
    .map_err(|e| AppError::ParseFailed { input: s.to_owned(), source: e })?;
```

## Modules and Crates
- Avoid wildcard imports (`use crate::*`).
- E0432/E0433: unresolved import → check `use` path and `Cargo.toml`.
- Organize: one module per responsibility, re-export through `mod.rs` or `lib.rs`.

## Async/Concurrency (when applicable)
- Use `async/await` with Tokio (preferred) or async-std.
- Never block inside async: no `std::thread::sleep`, use `tokio::time::sleep`.
- Ensure types are `Send + Sync` when shared across tasks.
- Anti-pattern: forgetting `.await` (compiles but never executes).

## Idioms to Prefer
- `Result` / `Option` + `?`
- Iterators over manual index loops
- Newtypes for invariants (`struct Port(u16)`)
- Explicit error enums for boundaries
- `impl AsRef<Path>` / `impl Into<String>` for flexible APIs

## Footguns to Avoid
- `unwrap()` / `expect()` in non-test code
- `unwrap_or_default()` without justification (swallows errors)
- `clone()` to silence borrow checker (fix ownership instead)
- `#[allow(clippy::all)]` (fix lints individually with justification)
- `version = "*"` in dependencies

## Aggressive Clippy Configuration
```toml
[lints.clippy]
unwrap_used = "deny"
expect_used = "warn"
panic = "deny"
indexing_slicing = "warn"
unreachable = "deny"
redundant_clone = "warn"
todo = "deny"
unimplemented = "deny"
```

## Review Checklist
- Are invariants encoded in types?
- Are errors actionable (not just "something went wrong")?
- Is there a regression test for every bug fix?
- Are dependencies minimal and justified?
- Does `verify.sh` pass?

## Sources
- The Rust Book: https://doc.rust-lang.org/book/
- The Rust Reference: https://doc.rust-lang.org/reference/
- Rustonomicon (unsafe): https://doc.rust-lang.org/nightly/nomicon/
- ANSSI Secure Rust Guidelines: https://github.com/ANSSI-FR/rust-guide
- RustAssistant (ICSE 2025): iterative compiler-driven correction
- JetBrains RustRover: empirical error frequency data
