# Skill: Rust Supply Chain Hardening

## When to use
Whenever `Cargo.toml` / dependency versions change.

## Objective
Prevent vulnerable or risky dependencies from entering the build.

## Required actions
1) Minimize dependency surface:
   - avoid new crates if stdlib suffices
   - avoid enabling broad feature sets
2) Run checks (as configured):
   - `cargo audit`
   - `cargo deny check`
   - `cargo vet` (if repo uses it)
3) Document decisions in `docs/ai/DECISIONS.md`:
   - why this crate
   - alternatives considered
   - license notes
   - feature flags chosen

## Red flags
- Unmaintained crates
- Many unsafe lines in transitive deps for a simple task
- Duplicate major versions of the same crate
- Network / crypto crates with low scrutiny
