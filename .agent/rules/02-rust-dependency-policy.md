# Rust Dependency Policy

## Allowed by default
- Standard library
- Existing dependencies already in `Cargo.toml`

## Introducing a new crate
Before adding a crate, you MUST answer:
1) What capability is missing without it?
2) Why is this crate preferable vs alternatives?
3) What is the maintenance risk (activity, bus factor)?
4) What is the license compatibility?
5) What features will you enable/disable?

Then you MUST run:
- `cargo audit` (RustSec advisories)
- `cargo deny check` (licenses/bans/duplicates)
- `cargo geiger` (unsafe footprint) for sensitive projects

## Feature hygiene
- Prefer `default-features = false` and enable only required features.
- Avoid pulling heavy ecosystems for trivial needs.

## Pinning
- Commit `Cargo.lock` for applications/binaries.
- Avoid loose version ranges for security-sensitive builds.

## Exceptions
Only with explicit approval documented in `docs/ai/DECISIONS.md`.
