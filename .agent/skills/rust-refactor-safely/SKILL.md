---
name: rust-refactor-safely
description: Refactor Rust code while keeping compile/tests/clippy/fmt green at each step.
---

# Rust Refactor Safely

## Goals
- Preserve behavior.
- Keep the repo green after each small commit.

## Protocol

1) Identify the refactor goal (rename, extract, simplify types, etc.).
2) Add/confirm tests first if behavior is not covered.
3) Refactor in tiny steps:
   - one rename
   - one extraction
   - one signature change
4) After each step:

```bash
./scripts/verify.sh --fast
```

5) At the end:

```bash
./scripts/verify.sh
```

6) Update `DECISIONS.md` if a new convention is introduced.

