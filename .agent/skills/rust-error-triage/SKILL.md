---
name: rust-error-triage
description: Systematic diagnosis of rustc error codes and Clippy lints; updates ERROR_PATTERNS.md.
---

# Rust Error Triage

## Inputs
- Compiler output (`cargo check` / `cargo test` / clippy output)

## Procedure

1) Extract:
- error code (E0xxx) if present
- file:line
- the highlighted snippet

2) Classify (choose one):
- Ownership/move
- Borrowing/lifetime
- Types/traits
- Modules/imports
- Mutability
- Temporaries

3) Apply the canonical fix pattern:
- Consult `ERROR_PATTERNS.md`.
- If missing, create a new entry with:
  - root cause
  - canonical fix
  - minimal snippet
  - minimal test

4) Validate:

```bash
./scripts/verify.sh --fast
```

5) If still failing, repeat from step (1) using the new first error.

