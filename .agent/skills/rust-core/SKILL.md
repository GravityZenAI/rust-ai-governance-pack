# Skill: Rust Core (Safe, idiomatic)

## Use for
- API design
- ownership/borrowing issues
- error handling
- iterators, slices, strings
- modules/crates layout

## Safe defaults
- Prefer `&T` / `&mut T` over cloning.
- Prefer iterators over index loops when it improves safety/readability.
- Use `Result<T, E>` for recoverable errors. Avoid panics.
- In library code: avoid `unwrap()/expect()`. In tests: allowed.

## API patterns
- Accept inputs as `impl AsRef<Path>` / `impl Into<String>` when it reduces friction.
- Return owned data when you must outlive input borrows.
- Use newtypes to encode invariants (e.g., `struct Port(u16);` with constructor).

## Error patterns
- Define a small error enum; implement `Display` (or use `thiserror` if approved).
- Add context at boundaries, not everywhere.

## Performance reality check
- Do not micro-optimize until tests exist.
- Avoid premature `unsafe`. If performance is critical, benchmark and justify.
