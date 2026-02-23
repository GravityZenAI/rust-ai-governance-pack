---
name: rust-compile-loop
description: Incremental Rust implementation using compiler/tests/clippy/fmt as the judge.
---

# Rust Compile Loop

## When to use
- Any task that modifies `.rs` files.

## Operating protocol

1) Restate the spec in 3–6 bullets.
2) Identify the smallest compilable step.
3) Implement that step.
4) Run:

```bash
./scripts/verify.sh --fast
```

5) If red:
- read the *first* error
- fix only what’s required
- re-run `--fast`

6) When green:
- run full `./scripts/verify.sh`
- provide evidence (commands + output + diff)

## Rust-specific heuristics

- Borrow checker errors: reduce scope; introduce bindings; reorder operations.
- Type errors: print/inspect types; add explicit annotations only where needed.
- Trait errors: add bounds or implement the trait; avoid over-generalizing.

