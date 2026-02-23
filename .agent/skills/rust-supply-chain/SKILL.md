# Skill: Rust Supply Chain Hardening

## When to use
- ALWAYS activate when `Cargo.toml` or dependency versions change.
- ALWAYS activate before adding any new crate.
- Complements `rust-verifier` — this handles dependency risk, verifier handles code quality.

## Critical rules
1. NEVER add a new crate if the standard library provides equivalent functionality.
2. ALWAYS use `default-features = false` and enable only the features you need.
3. ALWAYS document every dependency decision in `docs/ai/DECISIONS.md`.
4. NEVER ignore `cargo audit` findings — either fix, replace, or document the risk.

## Required checks before merging

| Check | Command | Action on failure |
|-------|---------|-------------------|
| Known vulnerabilities | `cargo audit` | Update, replace, or document waiver |
| License & ban policy | `cargo deny check` | Remove banned crate or request exception |
| Supply chain review | `cargo vet` (if configured) | Audit the crate or find a vetted alternative |

## What to document in DECISIONS.md

For every new dependency, record:
- **Why this crate**: what problem it solves
- **Alternatives considered**: at least one alternative and why it was rejected
- **License**: compatible with project license?
- **Feature flags**: which are enabled and why
- **Risk assessment**: maintenance status, unsafe usage, transitive dependency count

## Red flags (NEVER accept without justification)
- Unmaintained crates (no commits in 12+ months, no maintainer response)
- Many `unsafe` lines in transitive deps for a simple task
- Duplicate major versions of the same crate in the dependency tree
- Network or crypto crates with low scrutiny or no audit history
- Yanked versions in the dependency tree

## Feature flag discipline
- Gate optional functionality (like `serde` support) behind feature flags.
- In workspaces: use `[workspace.dependencies]` for version inheritance to prevent version drift.

## Cargo.toml hygiene
- Enable `#![warn(clippy::cargo)]` to catch manifest issues.
- Periodically run `cargo update --dry-run` to check for available updates.

