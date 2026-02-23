---
name: rust-refactor-safely
description: Refactor Rust code while keeping compile/tests/clippy/fmt green at each step.
---

# Rust Refactor Safely

## When to use
- Any structural change to existing Rust code (rename, extract, simplify, reorganize).
- For adding new code, use `rust-compile-loop` instead.

## Critical rules
1. NEVER make a refactor step that breaks compilation — every step MUST compile.
2. ALWAYS add or confirm tests BEFORE refactoring behavior that is not covered.
3. NEVER combine multiple refactors in one step — one rename OR one extraction OR one signature change.
4. If a refactor breaks tests, REVERT and try a smaller step.

## Anti-patterns to fix during refactoring

| Anti-pattern | Fix |
|-------------|-----|
| Excessive `.clone()` | Replace with `&T` borrowing; investigate if scoping resolves the borrow |
| `&Vec<T>` or `&String` in params | Refactor to `&[T]` and `&str` |
| Manual `Into` impls | Replace with `From` impls — `Into` is auto-derived |
| Over-abstraction | NEVER add generics or trait objects without a concrete second use case |
| Manual indexing loops | Replace with iterators for safety and clarity |

## Protocol

1. Identify the refactor goal (rename, extract, simplify types, etc.).
2. Add or confirm tests first — use `rust-testing` skill if needed.
3. Refactor in **one tiny step**:
   - One rename, OR
   - One extraction, OR
   - One signature change
4. After each step, run:

```bash
./scripts/verify.sh --fast
```

5. If red: REVERT the step, try a smaller change.
6. At the end, run full verification:

```bash
./scripts/verify.sh
```

7. Update `DECISIONS.md` if a new convention is introduced.

## Refactor patterns

- Replace `.clone()` with `&T` — each removed clone saves a heap allocation.
- Use `Cow<'a, T>` when a function conditionally needs ownership.
- After renaming or extracting types, ALWAYS derive: `Debug`, `Clone`, `PartialEq`.

