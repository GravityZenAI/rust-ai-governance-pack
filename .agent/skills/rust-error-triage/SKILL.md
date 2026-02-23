---
name: rust-error-triage
description: Systematic diagnosis of rustc error codes and Clippy lints; updates ERROR_PATTERNS.md.
---

# Rust Error Triage

## When to use
- Any time `cargo check`, `cargo test`, or `cargo clippy` produces errors.
- Complements `rust-compile-loop` — use this skill to diagnose, use compile-loop to iterate.

## Inputs
- Compiler output (`cargo check` / `cargo test` / clippy output)

## Procedure

1. **Extract** from the error output:
   - Error code (E0xxx) if present
   - File:line location
   - The highlighted snippet

2. **Classify** into exactly ONE category:

   | Category | Common codes | Typical fix |
   |----------|-------------|-------------|
   | Ownership/move | E0382, E0507 | Clone, borrow, restructure scope |
   | Borrowing/lifetime | E0502, E0505, E0597 | Reduce borrow scope, add lifetime |
   | Types/traits | E0277, E0308 | Add bound, implement trait, cast |
   | Modules/imports | E0432, E0433 | Fix path, add `use`, check visibility |
   | Mutability | E0596, E0594 | Add `mut`, use `RefCell`/`Cell` |
   | Temporaries | E0716 | Bind to variable before borrowing |

3. **Apply** the canonical fix pattern:
   - Consult `ERROR_PATTERNS.md` first.
   - If the error is NOT in ERROR_PATTERNS.md, create a new entry with:
     - Root cause
     - Canonical fix
     - Minimal failing snippet
     - Minimal test proving the fix

4. **Validate**:

```bash
./scripts/verify.sh --fast
```

5. If still failing, repeat from step (1) using the **new first error**.

## Fix patterns for error types

### Library errors
- Use `thiserror` with `#[error("...")]` for Display and `#[from]` for auto-conversion.
- Chain underlying causes with `#[source]` to preserve the error chain.
- When a variant holds a large payload, `Box` it: `ParseError(Box<serde_json::Error>)`.

### Application errors
- Use `anyhow` with `.context("why it failed")` for quick, informative error propagation.
- Add `.context()` at module boundaries, NOT on every call.

## Warnings vs errors
- Clippy warnings: fix them immediately or add `#[allow(clippy::...)]` with a comment explaining why.
- NEVER suppress warnings without a documented justification.

