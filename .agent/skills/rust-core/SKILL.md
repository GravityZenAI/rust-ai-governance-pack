# Skill: Rust Core (Safe, idiomatic)

## Use for
- API design, ownership/borrowing, error handling, iterators, slices, strings, modules/crates layout.
- This is the **foundation skill** — all other skills build on these rules.

## Critical ownership rules (ALWAYS apply)
1. ALWAYS prefer `&T` / `&mut T` over `.clone()`.
2. ALWAYS accept `&[T]` instead of `&Vec<T>`, and `&str` instead of `&String` in parameters.
3. ALWAYS use iterators over index loops — they eliminate bounds checks and are clearer.
4. NEVER use `unwrap()` or `expect()` in library code. In `#[cfg(test)]` only: allowed.
5. ALWAYS return `Result<T, E>` for operations that can fail — NEVER panic on expected errors.

## Ownership patterns
- Use `Cow<'a, T>` when a function must sometimes own and sometimes borrow.
- Derive `Copy` for small, trivial types (≤ 16 bytes, no heap allocation).
- Move large data instead of cloning — ownership transfer is zero-cost.
- Rely on lifetime elision; add explicit lifetimes only when the compiler requires them.
- Make every `.clone()` call intentional — each one is a potential heap allocation.

## Error handling

### Libraries
- Use `thiserror` to define error enums:
```rust
#[derive(Debug, thiserror::Error)]
enum MyError {
    #[error("failed to parse config")]
    Parse(#[from] serde_json::Error),
}
```
- Use `#[from]` for automatic `From` conversion between error types.
- ALWAYS create custom error types — NEVER use `Box<dyn Error>` in public APIs.

### Applications
- Use `anyhow` for ergonomic error handling: `.context("what failed")`.
- Use `?` for propagation — NEVER write manual `match` on `Result` unless adding context.
- Add `.context()` at module boundaries, not on every line.

## API design

### Type safety (highest priority)
- Use newtypes to encode invariants: `struct Port(u16)` with validating constructor.
- Add `#[must_use]` to functions returning `Result` or types that must not be silently discarded.
- Implement `From<T>`, not `Into<T>` — `Into` is auto-derived.
- Derive `Debug`, `Clone`, `PartialEq` on all public types — users expect them.
- Implement `Default` when there is a sensible default value.

### Patterns for complex APIs
- Use the Builder pattern for types with 3+ optional parameters.
- Use typestate for compile-time state machines: `Connection<Disconnected>` → `Connection<Connected>`.
- Parse, don't validate: convert raw data into validated types at boundaries.

### Future-proofing
- Use `#[non_exhaustive]` on public enums and structs to allow additive changes.
- Accept `impl AsRef<Path>` / `impl Into<String>` for flexible input types.
- Return owned data when the return value must outlive the input borrows.

## Memory awareness
- Use `Vec::with_capacity(n)` when the size is known or estimable.
- Use `clone_from()` instead of `x = y.clone()` to reuse existing allocations.
- Box large enum variants to keep the enum's stack size small (`std::mem::size_of`).

## Performance
- NEVER micro-optimize until tests exist and a profiler confirms the bottleneck.
- NEVER use `unsafe` for performance without a benchmark proving the gain.

