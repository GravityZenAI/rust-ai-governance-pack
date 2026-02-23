# Rule: Command Safety (secondary, but enforced)

If executing terminal commands:

- Prefer read-only / build commands (`cargo`, `rustc`, `git status`, `git diff`).
- Do **not** run destructive commands unless explicitly requested and reviewed:
  - `rm -rf`, `sudo`, `dd`, `mkfs`, mass delete, credential tools.

If a workflow requires bypassing this rule, log an exception in `EXCEPTIONS.md`.

