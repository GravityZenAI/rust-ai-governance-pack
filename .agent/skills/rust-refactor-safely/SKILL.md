---
name: rust-refactor-safely
description: Refactor Rust code while keeping compile/tests/clippy/fmt green at each step.
---

# Rust Refactor Safely

> Inherits all rules from `rust-core`. This skill adds safe refactoring protocol.

## When to use
- Any structural change to existing Rust code (rename, extract, simplify, reorganize).
- For adding new code, use `rust-compile-loop` instead.

## Critical rules
1. NEVER make a refactor step that breaks compilation — every step MUST compile.
2. ALWAYS add or confirm tests BEFORE refactoring untested behavior.
3. NEVER combine multiple refactors in one step.
4. If a refactor breaks tests, REVERT and try a smaller step.

## Protocol

1. Identify the refactor goal (rename, extract, simplify types).
2. Add or confirm tests — use `rust-testing` skill if needed.
3. Refactor in **one tiny step**: one rename OR one extraction OR one signature change.
4. After each step: `./scripts/verify.sh --fast`
5. If red: REVERT the step, try a smaller change.
6. At the end: `./scripts/verify.sh`
7. Update `DECISIONS.md` if a new convention is introduced.

## Example: safe signature refactoring

```rust
// Step 1: change parameter type (one change only)
// BEFORE
fn process(items: &Vec<String>) -> usize { items.len() }

// AFTER — verify compiles, then update all callers
fn process(items: &[String]) -> usize { items.len() }
// No caller changes needed — &Vec<T> auto-derefs to &[T]
```

## Anti-patterns to fix during refactoring

| Anti-pattern | Fix |
|-------------|-----|
| Excessive `.clone()` | Replace with `&T`; investigate if scoping resolves the borrow |
| `&Vec<T>` or `&String` in params | Refactor to `&[T]` and `&str` |
| Manual `Into` impls | Replace with `From` impls — `Into` is auto-derived |
| Over-abstraction | NEVER add generics without a concrete second use case |
| Manual indexing loops | Replace with iterators for safety and clarity |

## Common mistakes

| Mistake | Fix |
|---------|-----|
| Renaming + changing signature in one commit | Do ONE refactor per step; verify between each |
| Refactoring code without tests | Add tests FIRST, then refactor — otherwise you can't verify behavior |
| Not reverting when stuck | REVERT immediately; git stash is your friend |

