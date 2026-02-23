---
name: rust-kata-coach
description: Run and solve Rust katas with compiler-driven feedback and scoring.
---

# Rust Kata Coach

## When to use
- Training exercises in `training/kata_suite`.
- For error diagnosis during katas, use `rust-error-triage`.

## Rules for kata solutions
1. ALWAYS prefer `&T` over `.clone()` — the borrow checker is teaching ownership.
2. ALWAYS use `&[T]` instead of `&Vec<T>`, and `&str` instead of `&String`.
3. ALWAYS return `Result<T, E>` for fallible operations — NEVER panic.
4. ALWAYS use `?` for error propagation.
5. ALWAYS name tests descriptively: `test_empty_input_returns_none`, not `test1`.

## How to run

```bash
cd training/kata_suite
cargo test
```

## Coaching loop

1. Pick ONE failing test (the first one in output).
2. Implement the minimal fix.
3. Re-run only that test: `cargo test <name>`.
4. When the kata passes:
   - Write a 1-line learning note in the kata file.
   - If a new error pattern was encountered, add it to `ERROR_PATTERNS.md`.
5. If ALL tests fail, start with the simplest test (shortest name or first alphabetically).

## Test structure in katas

- Follow Arrange/Act/Assert:
  1. **Arrange**: set up input data.
  2. **Act**: call the function under test.
  3. **Assert**: check the result.

## Scoring

| Result | Criteria |
|--------|----------|
| PASS | All tests green + `cargo fmt --check` green + `cargo clippy` green |
| FAIL | Any compile error, failing test, clippy warning, or fmt drift |

