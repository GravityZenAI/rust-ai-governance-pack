# Workflow: /kata

Goal: train Rust competence with compiler friction.

## Steps

1) Choose the next kata:
- Default: the lowest-numbered kata with failing tests.

2) Run tests:

```bash
cd training/kata_suite
cargo test
```

3) Fix incrementally:
- One kata/module at a time.
- After each change: re-run the smallest test set possible.

4) Pass conditions:
- All tests green.
- `cargo fmt -- --check` and `cargo clippy -- -D warnings` green.

5) Record:
- which errors appeared
- how many iterations
- any new patterns (update `ERROR_PATTERNS.md`)

