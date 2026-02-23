# Rule: Rust Operating Loop (non-negotiable)

When working on Rust in this workspace:

1) **Spec first**
   - State expected behavior, edge cases, and success criteria.

2) **Incremental implementation only**
   - Make the smallest change that can be compiled.
   - Prefer smaller PR-sized edits.

3) **Compiler and tests are the judge**
   - After each meaningful edit: run `./scripts/verify.sh --fast`.
   - Before marking done: run `./scripts/verify.sh`.

4) **No unverifiable claims**
   - Never claim “fixed”, “works”, or “green” without showing:
     - the exact commands executed
     - the relevant output
     - the `git diff`

5) **Error memory**
   - If a rustc error code (E0xxx) or a clippy lint appears and is not already documented, add an entry to `ERROR_PATTERNS.md`.

