---
name: rust-error-triage
description: Systematic diagnosis of rustc error codes and Clippy lints; updates ERROR_PATTERNS.md.
---

# Rust Error Triage

> Inherits all rules from `rust-core`. This skill adds error diagnosis procedure.

## When to use
- Any time `cargo check`, `cargo test`, or `cargo clippy` produces errors.
- Complements `rust-compile-loop` — use this to diagnose, compile-loop to iterate fixes.

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

3. **Apply** the canonical fix:
   - Consult `ERROR_PATTERNS.md` first.
   - If NOT found, create a new entry with: root cause, canonical fix, minimal snippet, minimal test.

4. **Validate**: `./scripts/verify.sh --fast`

5. If still failing, repeat from step (1) on the **new first error**.

## Example: diagnosing E0502

```rust
// Error: cannot borrow `data` as mutable because it is also borrowed as immutable
let first = &data[0]; // immutable borrow
data.push(42);        // mutable borrow — CONFLICT
println!("{}", first);

// Fix: finish using the immutable borrow before mutating
let first = data[0];  // copy the value (no borrow kept)
data.push(42);        // now safe
```

## Fix patterns for error types

### Library errors
- `thiserror` with `#[error("...")]` + `#[from]` for auto-conversion.
- `#[source]` to preserve error chains.
- `Box` large payloads: `ParseError(Box<serde_json::Error>)`.

### Application errors
- `anyhow` with `.context("why it failed")`.
- `.context()` at module boundaries, NOT on every call.

## Warnings vs errors
- Clippy warnings: fix immediately or `#[allow(clippy::...)]` with justification.
- NEVER suppress warnings without documented justification.

## Common mistakes

| Mistake | Fix |
|---------|-----|
| Trying to fix error by type before reading the message | Read the FULL error message — the compiler suggests the fix |
| Ignoring `rustc --explain Exxxx` | Run it — it gives examples and detailed explanations |
| Suppressing clippy with `#[allow]` without a comment | ALWAYS add `// Reason: ...` next to `#[allow]` |

