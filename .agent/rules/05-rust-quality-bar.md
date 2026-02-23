# Rule: Rust Quality Bar

- Treat warnings as errors in CI (`clippy` runs with `-D warnings`).
- Keep the code formatted (`cargo fmt` check must pass).

## Prohibited by default

- Introducing `unsafe` without:
  - a written invariant in comments
  - a test that would fail if the invariant breaks
  - an entry in `DECISIONS.md`

- Using `unwrap()` / `expect()` in production code.
  - Allowed only with a documented exception in `EXCEPTIONS.md`.

- Indexing into collections with `[]` when input is not proven in-bounds.
  - Prefer `.get()` + explicit handling.

