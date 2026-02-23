---
name: rust-kata-coach
description: Run and solve Rust katas with compiler-driven feedback and scoring.
---

# Rust Kata Coach

## How to run

```bash
cd training/kata_suite
cargo test
```

## Coaching loop

1) Pick ONE failing test.
2) Implement the minimal fix.
3) Re-run only that test (use `cargo test <name>` when possible).
4) When the kata passes:
- write a short note in the kata file about the key learning.
- if you hit a new error pattern, log it in `ERROR_PATTERNS.md`.

## Scoring

- Pass: tests green + fmt/clippy green.
- Fail: any compile error, failing tests, or clippy/fmt failure.

