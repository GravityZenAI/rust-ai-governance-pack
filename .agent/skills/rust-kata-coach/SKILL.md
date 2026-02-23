---
name: rust-kata-coach
description: Run and solve Rust katas with compiler-driven feedback and scoring.
---

# Rust Kata Coach

> Inherits all rules from `rust-core`. This skill adds the kata coaching loop.

## When to use
- Training exercises in `training/kata_suite`.
- For error diagnosis during katas, use `rust-error-triage`.

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

## Example: idiomatic kata solution

```rust
// WRONG — fighting the borrow checker
fn longest(words: &Vec<String>) -> String {
    let mut best = words[0].clone();
    for w in words {
        if w.len() > best.len() { best = w.clone(); }
    }
    best
}

// RIGHT — idiomatic Rust
fn longest(words: &[String]) -> &str {
    words.iter().max_by_key(|w| w.len()).map(|w| w.as_str()).unwrap_or("")
}
```

## Test structure in katas

- Follow Arrange/Act/Assert:
  1. **Arrange**: set up input data.
  2. **Act**: call the function under test.
  3. **Assert**: check the result.
- Pattern: `test_<function>_<scenario>_<expected_result>`

## Scoring

| Result | Criteria |
|--------|----------|
| PASS | All tests green + `cargo fmt --check` green + `cargo clippy` green |
| FAIL | Any compile error, failing test, clippy warning, or fmt drift |

## Common mistakes

| Mistake | Fix |
|---------|-----|
| Fixing multiple tests at once | Fix ONE test, verify, then move to the next |
| Using `.clone()` to pass every kata | First solve idiomatically; clone only if borrowing fails |
| Not recording the learning | ALWAYS write a 1-line learning note — it builds ERROR_PATTERNS.md |

