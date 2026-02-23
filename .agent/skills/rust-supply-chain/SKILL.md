---
name: rust-supply-chain
description: Prevent vulnerable or risky dependencies from entering the build.
---

# Skill: Rust Supply Chain Hardening

> Inherits all rules from `rust-core`. This skill adds dependency governance.

## When to use
- ALWAYS activate when `Cargo.toml` or dependency versions change.
- ALWAYS activate before adding any new crate.
- Complements `rust-verifier` — this handles dependency risk, verifier handles code quality.

## Critical rules
1. NEVER add a new crate if the standard library provides equivalent functionality.
2. ALWAYS use `default-features = false` and enable only the features you need.
3. ALWAYS document every dependency decision in `docs/ai/DECISIONS.md`.
4. NEVER ignore `cargo audit` findings — fix, replace, or document the risk.

## Required checks before merging

| Check | Command | Action on failure |
|-------|---------|-------------------|
| Known vulnerabilities | `cargo audit` | Update, replace, or document waiver |
| License & ban policy | `cargo deny check` | Remove banned crate or request exception |
| Supply chain review | `cargo vet` (if configured) | Audit the crate or find a vetted alternative |

## Example: adding a dependency correctly

```toml
# Cargo.toml — CORRECT
[dependencies]
serde = { version = "1", default-features = false, features = ["derive"] }

# WRONG — pulls in ALL features unnecessarily
# serde = "1"
```

## What to document in DECISIONS.md

For every new dependency, record:
- **Why this crate**: what problem it solves
- **Alternatives considered**: at least one and why rejected
- **License**: compatible with project license?
- **Feature flags**: which are enabled and why
- **Risk assessment**: maintenance status, unsafe count, transitive deps

## Red flags (NEVER accept without justification)
- Unmaintained crates (no commits in 12+ months)
- Many `unsafe` lines in transitive deps for a simple task
- Duplicate major versions of the same crate
- Network or crypto crates with low scrutiny
- Yanked versions in the dependency tree

## Feature flag discipline
- Gate optional functionality behind feature flags.
- In workspaces: `[workspace.dependencies]` for version inheritance.

## Cargo.toml hygiene
- `#![warn(clippy::cargo)]` to catch manifest issues.
- `cargo update --dry-run` periodically to check for updates.

## Common mistakes

| Mistake | Fix |
|---------|-----|
| Adding `serde` with all features on | Use `default-features = false, features = ["derive"]` |
| Not checking license compatibility | Always run `cargo deny check licenses` before merging |
| Ignoring `cargo audit` because "it's a dev dependency" | Dev deps can still run arbitrary code during build |

