---
name: rust-compile-loop
description: Incremental Rust implementation using compiler/tests/clippy/fmt as the judge.
---

# Rust Compile Loop

> Inherits all rules from `rust-core`. This skill adds the incremental compile-fix loop.

## When to use
- Any task that modifies `.rs` files.
- For **incremental implementation**. For final verification, use `rust-verifier`.

## Operating protocol

1. Restate the spec in 3–6 bullets.
2. Identify the smallest compilable step.
3. Implement that step.
4. Run:

```bash
./scripts/verify.sh --fast
```

5. If red (compile error, test failure, clippy lint, or fmt drift):
   - Read the **first** error only.
   - Fix only what is required for that error.
   - Re-run `--fast`.
   - Repeat until green.

6. When green:
   - Run full `./scripts/verify.sh`.
   - Provide evidence: commands + output + diff.

## Heuristics for common compiler errors

| Error type | Fix strategy |
|------------|-------------|
| Borrow checker (E0382, E0505) | Reduce scope; introduce bindings; reorder operations |
| Type mismatch (E0308) | Inspect types; add annotations only where needed |
| Trait not satisfied (E0277) | Add bounds or implement the trait; do NOT over-generalize |
| Incompatible error types | Implement `From` or use `.map_err()` |
| Format/clippy | `cargo fmt` to fix; for clippy, apply the fix or `#[allow()]` with justification |

## Example: fixing a borrow checker error

```rust
// WRONG — borrow lives too long
fn process(data: &mut Vec<i32>) -> &i32 {
    data.push(42);
    data.last().unwrap()  // borrows data while we just mutated it
}

// RIGHT — introduce a binding to reduce borrow scope
fn process(data: &mut Vec<i32>) -> i32 {
    data.push(42);
    *data.last().unwrap()  // copy the value, borrow ends
}
```

## Common mistakes

| Mistake | Fix |
|---------|-----|
| Trying to fix ALL errors at once | Fix ONLY the first error — later ones are often cascading |
| Adding `.clone()` to silence the borrow checker | Try reducing borrow scope first; clone only as last resort |
| Skipping `--fast` and running full verify every time | Use `--fast` during iteration, full verify only once at the end |

