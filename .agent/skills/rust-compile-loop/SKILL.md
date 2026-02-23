---
name: rust-compile-loop
description: Incremental Rust implementation using compiler/tests/clippy/fmt as the judge.
---

# Rust Compile Loop

## When to use
- Any task that modifies `.rs` files.
- Use this skill for **incremental implementation**. For full verification of finished work, use `rust-verifier`.

## Critical rules (read first)
- ALWAYS prefer `&T` / `&mut T` over `.clone()` — only clone when the borrow checker demands it.
- ALWAYS return `Result<T, E>` for fallible operations — NEVER panic on expected errors.
- ALWAYS use the `?` operator for error propagation — NEVER use manual `match` on `Result` unless adding context.
- Accept `&[T]` instead of `&Vec<T>`, and `&str` instead of `&String` in function parameters.

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
| Borrow checker (E0382, E0505) | Reduce scope; introduce bindings; reorder operations. |
| Type mismatch (E0308) | Print/inspect types; add explicit annotations only where needed. |
| Trait not satisfied (E0277) | Add bounds or implement the trait; do NOT over-generalize. |
| Incompatible error types | Implement `From` or use `.map_err()`. |
| Format/clippy | Run `cargo fmt` to fix; for clippy, apply the suggested fix or add `#[allow()]` with justification. |

## Ownership rules during implementation

- Use `Cow<'a, T>` when a function must sometimes own and sometimes borrow data.
- If you clone to satisfy the borrow checker, first try reducing the scope of the borrow.

